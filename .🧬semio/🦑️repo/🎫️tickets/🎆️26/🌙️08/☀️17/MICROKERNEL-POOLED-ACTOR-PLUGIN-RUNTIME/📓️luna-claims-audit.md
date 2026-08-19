# Plugin Capability Claims Audit — Mechanical Data Findings

## 1. Enforcing Code & Mechanism

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:2577–2593`

**Function:** `require_declared_capability_or_record()`

```rust
// Line 2577–2593
fn require_declared_capability_or_record(
    definition: &ArtifactDefinition,
    definition_error: &mut Option<ArtifactDefinitionError>,
    kind: ArtifactCapabilityKind,
    claims: Result<Vec<ArtifactIdentityClaim>, ArtifactDefinitionError>
) {
    let result = claims.and_then(|mut claims| {
        claims.sort();
        let capability = definition
            .capabilities_of(&kind)
            .find(|capability| capability.claims() == claims)  // ← EXACT EQUALITY CHECK
            .ok_or_else(|| ArtifactDefinitionError::new(
                "artifact-definition.runtime-capability",
                format!("no declared {} capability owns the runtime claims", kind.as_str())
            ))?;  // ← ERROR MESSAGE (line 2587)
        definition.require_declared_capability(&kind, capability.identity(), &claims, None)
    });
}
```

**Comparison:** Line 2586 performs `capability.claims() == claims` — a sorted set equality check. Both are `Vec<ArtifactIdentityClaim>` after sorting; the comparison is structural field-by-field equality.

**Declared vs. Runtime Claims:**
- **DECLARED:** Hardcoded tuples in `definition()` functions (e.g., `🧬️puzzle2d:429`), passed to `.claim(ArtifactIdentityClaim::new(namespace, value))` builders. Example: `("dialect", "s.puzzle2d@1/*")` becomes one claim in a "composer" capability.
- **RUNTIME:** Computed from actual registrations passed to `.composers(entries)`, `.languages(...)`, `.document_codec::<App>()` builders in `declaration()` (e.g., `🧬️puzzle2d:475`). Each entry's own claims are extracted and gathered into a set.

**Error Surface:** When `try_build()` (line 2478) invokes this check and finds zero matches, `plugin_manifest()` returns an `assembly-failed` stub instead of a real manifest.

---

## 2. Working Template: Note Plugin

**Reference Artifact:** `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs`

**Declared Capabilities (definition):** Lines 21–71
- Line 43: `("s.note.composer.note", "composer", "s.note@1/*", &[("dialect", "s.note@1/*")], None)`
  - Capability kind: `"composer"`, descriptor: `"s.note@1/*"`, **claims:** `[("dialect", "s.note@1/*")]`
- Line 44: `("s.note.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None)`
  - Same pattern for other formats.

**Runtime Claims (declaration):** Line 79–83
```rust
pub fn artifact() -> ArtifactDeclaration {
    ArtifactDeclaration { 
        kind: ArtifactKindId::parse("s.note.note").expect("canonical note kind"), 
        localization: &[], 
        standards: vec![crate::artifacts::note::standards::v1::standard()] 
    }
}
```

Note migrated to the NEW `ArtifactDeclaration` struct (not `ArtifactDeclarationBuilder`), which reads runtime claims from `.standards[].subsets[].io_registry::entries()` automatically. This declaration structure DOES NOT call `.try_build()` — it is pre-validated by the standards tree structure itself. ✅ **NO FAILURE.**

---

## 3. Failing Plugins: Detailed Findings

### Puzzle (Representative of 6 failing plugins using `.artifact(declaration())` pattern)

**File:** `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs:41`
```rust
.artifact(crate::artifacts::puzzle2d::declaration().map_err(...)?)
```

**Declared Capabilities:** `🧬️puzzle2d/🦀️component.rs:437–443` (lines excerpted)
```rust
("s.puzzle2d.composer.native", "composer", "s.puzzle2d@1/*", &[("dialect", "s.puzzle2d@1/*")], None),
("s.puzzle2d.composer.format-1", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
// ... 4 more composer rows (PDF, PNG, JSON, DWG, DXF)
```
**Count:** 7 "composer" capabilities, each with exactly ONE claim: `[("dialect", "...")]`

**Runtime Registration:** `🧬️puzzle2d/🦀️component.rs:475`
```rust
.composers(crate::artifacts::puzzle2d::standards::v1::subsets::any::io::io_registry::entries())
```

**Actual Runtime Claims:** Located at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

The `entries()` function returns ComposerEntry structs whose `.writes` fields DO contain the declared dialects, BUT `.try_build()` extracts claims from ComposerEntry metadata that likely includes ADDITIONAL fields beyond dialect (e.g., source format, target format, MIME type). These extra claims do not match any declared capability row.

**Pattern Across 7 Failing Plugins:**
| Plugin | Artifact Count | Known Issue |
|--------|---|---|
| FEM | 2 (fem2d, fem3d) | Uses `.artifact(declaration())` pattern; likely missing "inference" or "composer" claims |
| Layout | 1 (layout) | Same pattern |
| Playbook | 1 (playbook) | Same pattern |
| Trinity | 2 (jack, rewrite) | Same pattern |
| Puzzle | 3 (2d, 3d, 5d) | Detailed above |
| Block | 1 (block) | Same pattern |
| Stdio | ~36 formats | Uses OLD `.editor::<>()/.viewer::<>()` pattern, not declaration(); error code unknown |

### Puzzle 2D Fix Delta

**File to Edit:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs:432–456` (definition rows)

**The Exact Issue:** Declared composers specify ONLY `("dialect", "...")` claim. Runtime composers likely require:
- `("dialect", "...")`  [already declared]
- `("format", "...")`    [MISSING from declared]
- Or other metadata claims

**Fix Classification:** **(a) Pure data** — add missing claim tuples to each composer row in `definition()`. No shared mechanism change needed; parallel identical patterns across fem/layout/playbook/trinity/block/puzzle suggest a systematic under-declaration.

---

## 4. Blocked Verification

- **Stdio structure:** Still uses OLD `.editor()` / `.viewer()` channel (no `.artifact(declaration())` pattern found). The assembly error for stdio's ~35 formats cannot be verified against the NEW enforcing code path without reading the old channel's own try_build validation (not traced this pass).
- **Exact runtime claim structure:** The `.composers(entries())` entries' own claim extraction logic lives in framework code not read this pass. The delta requires reading ComposerEntry struct definition and its claims() method.
- **FEM, Layout, Playbook, Trinity, Block:** All follow puzzle's pattern identically but were not traced in detail; assumed same root cause (under-declared composer claims).

---

## How I Measured

```bash
# Enforcing code location
grep -n "no declared.*capability owns the runtime claims" \
  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs

# Working example: note artifact declaration
grep -n "fn artifact()" /Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs

# Puzzle definition rows
grep -n "pub fn definition()" \
  /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs

# Puzzle declaration call
grep -n "pub fn declaration()" \
  /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs

# FEM declaration count via Python walk
python3 -c "
import os
fem = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem'
count = sum(1 for r,d,f in os.walk(fem) if 'declaration' in open(os.path.join(r,'🦀️component.rs')).read())
print(f'FEM artifacts with declaration(): {count}')
"
```

---

## Summary Recommendations

**For all seven failing plugins**, add missing claim tuples to `definition()` rows. The mechanical fix is identical across all: audit each declared capability (especially "composer", "inference", "codec") and ensure claimed namespaces/values match EXACTLY what the runtime registration (`.composers()`, `.inferences()`, `.document_codec::<>()`) will extract.

**Stdio** requires separate investigation: trace the old `.editor()` / `.viewer()` channel's try_build failure mode, or migrate it to the new `.artifact(declaration())` pattern first.

