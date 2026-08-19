# Dialect Collision Audit: `s.stdio.dwg@ac1018/*`

## 1. Dialect Declaration & Reference Map

| Plugin | Artifact | Dialect ID | File:Line | Type | Content |
|--------|----------|-----------|----------|------|---------|
| 🌀️procedural | procedural3d | s.stdio.dwg@ac1018/* | `🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs:96-97` | **DECLARATION** | `.descriptor(b"s.stdio.dwg@ac1018/*")?` + `.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)` |
| 🌍️gis | gismap | s.stdio.dwg@ac1018/* | `🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs:238-239` | **DECLARATION** | `.descriptor(b"s.stdio.dwg@ac1018/*")?` + `.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)` |
| 🏭️process | process3d | s.stdio.dwg@ac1018/* | `🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs:1028-1029` | **DECLARATION** | `.descriptor(b"s.stdio.dwg@ac1018/*")?` + `.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)` |
| 🗄️stdio | (artifact definition) | s.stdio.dwg@ac1024/* | `🗄️stdio/🗿️artifacts/🖊️dwg/🧬️schema/📜️artifact-definition.json:139-147` | **DECLARATION** | `"id": "s.stdio.dwg.runtime.composer.dialect-s-stdio-dwg-ac1024.v1"`, claims dialect `s.stdio.dwg@ac1024/*` only |
| 12+ plugins (🎥️shooting, 💠️lowpoly, 📐️cad, 📸️remodel, 🖍️draw, 🖨️raster, 🗒️note, 🧩️puzzle, etc.) | N/A | s.stdio.dwg@ac1018/* | Multiple component.rs files | REFERENCE | Listed in composer metadata tuples: `("composer", "s.stdio.dwg@ac1018/*", ...)` — **not in ArtifactDefinition claims** |

**Key Finding**: Three plugins (procedural, gis, process) each register an `ArtifactCapability` with kind `composer()` that CLAIMS ownership of the SAME dialect namespace via `ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")`. This triggers the registry's atomicity rule.

---

## 2. Registry Validation Rule

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`

**Test**: `registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically()` (line ~1460-1480)

**Validation Code** (excerpt):
```rust
/// 🛡️ Registers exactly one new definition or rejects it without partial mutation.
pub fn register(&mut self, definition: ArtifactDefinition) -> Result<(), ArtifactDefinitionError> {
    // ... [identity checks]
    for capability in definition.capabilities() {
        for claim in capability.claims() {
            if let Some(previous) = self.claims.get(claim) {
                return Err(ArtifactDefinitionError::new(
                    "artifact-definition.conflicting-claim",
                    format!("{}:{} is already registered by {}", 
                        claim.namespace.as_str(), claim.value(), previous)
                ));
            }
        }
    }
}
```

**Rule**: The `ArtifactDefinitionRegistry::register()` method forbids ANY two `ArtifactDefinition`s from claiming the SAME value in the SAME namespace (dialect, schema, codec, mime, extension). The rule does NOT distinguish between:
- Who legitimately OWNS the artifact
- Whether a plugin is EXPORTING vs. IMPORTING a dialect
- The distinction between a "capability to write" vs. "capability to read"

**Atomicity**: If ANY claim conflicts, the entire registration fails without partial mutation (as guaranteed by the `Result<(), ..>` early-return pattern).

---

## 3. Ownership Analysis

### 3.1 Artifact Tree Ownership

**🗄️stdio owns the DWG artifact tree**:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/` contains:
  - `🏅️standards/🔖️ac1018/` → Editor/Viewer implementations
  - `🏅️standards/🔖️ac1024/` → Editor/Viewer implementations  
  - `🧬️schema/📜️artifact-definition.json` → Canonical schema definition

### 3.2 Codec Implementation Ownership

**🗄️stdio owns the DWG codec**:
- `s.stdio.dwg.runtime.codec.codec-stdio-dwg-extension-dwg.v1` → stdio declares
- Codec claims: `("codec", "stdio.dwg")`, `("extension", "dwg")`
- No other plugin claims the `stdio.dwg` codec

### 3.3 Dialect Declaration Analysis

**ac1024 (R2004+)**:
- **Declared by**: 🗄️stdio (line 139-147 of artifact-definition.json)
- **Reasoning**: stdio is the canonical format owner; ac1024 is the modern standard

**ac1018 (R2000)**:
- **Declared by**: 🌀️procedural, 🌍️gis, 🏭️process (each in their own component.rs)
- **Reasoning**: Three domain plugins export to the legacy ac1018 format (older CAD standard)
- **Problem**: All three claim the SAME dialect id, creating atomic conflict

### 3.4 Composition Pattern in Working Plugins

**🗒️note and 🖍️draw** (working, no collision):
- Reference `s.stdio.dwg@ac1018/*` in composer metadata: `("s.note.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", ...)`
- **Do NOT** register an `ArtifactCapability` with a `.claim()` in their `ArtifactDefinition`
- Rely on stdio's (or a single owner's) prior dialect claim

**Excerpt from 🗒️note/🦀️component.rs:48**:
```rust
("s.note.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], ...)
```
This is a **REFERENCE in metadata**, NOT an `ArtifactIdentityClaim` in the definition.

---

## 4. Root Cause

**Primary Cause**: Three plugins (procedural, gis, process) attempt to EXPORT to the same dialect (`s.stdio.dwg@ac1018/*`) but each independently declares ownership via `ArtifactIdentityClaim` in their `ArtifactDefinition`.

**Why ac1018 instead of ac1024?**
- ac1018 is the legacy R2000 format; ac1024 is R2004+
- Each domain (procedural geometry, GIS map, manufacturing process) may have different ac1018 export rules or feature fidelity
- No mechanism exists to distinguish "exported variant A" from "exported variant B" of the same standard within the current dialect namespace

---

## 5. Recommended Fix

**Location**: Data edits in `🌍️gis` and `🏭️process` component.rs files only (NOT in procedural or stdio).

**Mechanism**: 
1. **Keep procedural3d's declaration** of `s.stdio.dwg@ac1018/*` (it is the first/primary exporter)
2. **Remove the .claim() call** from 🌍️gis/🗺️gismap/component.rs (lines 238-239)
3. **Remove the .claim() call** from 🏭️process/🧊️process3d/component.rs (lines 1028-1029)
4. **Retain** both plugins' composer metadata references to `s.stdio.dwg@ac1018/*` (for routing)

**Rationale**:
- The dialect ID is a **format capability shared by all exporters**, not an exclusive ownership claim
- Each plugin CAN export to ac1018 as long as ONE plugin declares it in the registry
- The registry only needs ONE `.claim()` per dialect to reserve the namespace
- Other plugins reference it via composer metadata (as 🗒️note and 🖍️draw already do)
- This follows the **same pattern used by procedural2d** (which omits the PNG/JSON claims, deferring to other plugins)

**Rationale from Procedural2d Comment** (component.rs:195):
```
// 🖼️ No `compose_export_dwg`/`EXPORT_DWG_DIALECT` here: procedural3d owns the `s.stdio.dwg@ac1018/*` EXPORT claim (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3, a documented tie-break, not evidence-backed like the DWG↔mesh-bridge split below).
```
This is already the intended pattern — but gis and process did not follow it.

**Alternative Fix** (if different ac1018 variants are needed):
Would require a **mechanism change** to support dialect variants (e.g., `s.stdio.dwg@ac1018/procedural`, `s.stdio.dwg@ac1018/gis`). This repo's design forbids compatibility layers, so it would require architectural review.

---

## 6. Layout's DwgSnapshot/DwgDecodeStatus Issue

**Different Root Cause** — NOT related to dialect collision.

**File**: `📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs:8-16`

**Issue**: Layout's DWG serializer used a stale `DwgSnapshot { decode_status: SentinelOnly, .. }` sentinel that no longer exists in stdio's evolved R2004+ `DwgSnapshot` struct (lines 8-16 comment).

**Status**: Already fixed (lines 17-20):
```rust
pub fn serialize(from: &LayoutSnapshot) -> Result<DwgSnapshot, store::PackError> {
    let text = <LayoutSnapshot as store::ArtifactDsl>::print_dsl(from);
    let bytes = semio_framework_os::svg_to_dwg_bytes(&text).map_err(store::PackError::Schema)?;
    decode_dwg(&bytes).map_err(store::PackError::Schema)
}
```
Routes through the honest `svg_to_dwg_bytes` → `decode_dwg` pipeline (same fix applied to 🗒️note).

**Conclusion**: This is **API drift, not a dialect collision**. Already mitigated.

---

## How I Measured

1. **Python3 os.walk** over `/Users/ueli/Documents/semio/✏️s/🔌️plugins/` (33 plugins) to search all `.rs` and `.json` files for `s.stdio.dwg` references (glob patterns unreliable on emoji paths).

2. **Grep + manual inspection** of each plugin's `component.rs` and registry JSON to distinguish:
   - **Declarations** = `.descriptor()` + `.claim(ArtifactIdentityClaim::new(...dialect...))` in `ArtifactCapability`
   - **References** = Composer metadata tuples (not formal claims)

3. **Registry validation logic** from `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`ArtifactDefinitionRegistry::register()` method)

4. **Artifact ownership** verified by checking:
   - Artifact tree location (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/`)
   - Codec implementation claims (stdio only)
   - Editor/Viewer implementations (stdio only)

5. **Layout issue** isolated from dialect collision via separate file analysis.

---

## Cannot Verify

- Whether each plugin's ac1018 export **actually differs** in fidelity/features (would require format domain expertise or integration tests)
- Whether there is documented precedent for "variant selection" within a single dialect namespace (no formal mechanism found)
- Whether this collision is preventing descriptor emission for **both** procedural and gis simultaneously or only one

