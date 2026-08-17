# The Canonical Real Conformance Subset Pattern

Written by the W2 pilot agent (`s.stdio.pdf`) after restructuring `✳️a-2b` → `✳️a` on 1.7 and
adding the schema-gapped `✳️a`/`✳️x` on 1.4. Read this BEFORE writing any new subset in this
ticket — it supersedes ad-hoc re-derivation from the old `✳️a-2b` pilot, which itself had one
standing breach (missing `🎹️composer/🟦️component.ts`) that is now fixed at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/`.

Two reference examples exist post-W2, deliberately at opposite ends of the honesty spectrum:
- **Full-depth**: `📄️pdf` 1.7 `✳️a` — the engine retains a full object graph
  (`PdfSnapshot.objects: Vec<PdfIndirectObject>`), so the analyzer implements real, hard-gating
  ISO 19005-2/-3 checks.
- **Honestly-scope-limited**: `📄️pdf` 1.4 `✳️a`/`✳️x` — the engine retains only
  `PageDoc{width,height,text}`, so the analyzer implements the ONE real check each field
  genuinely supports, plus a mandatory schema-gap diagnostic naming exactly what's missing.

Every subsequent subset in this ticket is one of these two shapes. Pick honestly, don't split the
difference.

## 1. The five-component shape

Every real (non-`✳️any`) subset lives at `🏅️standards/🔖️<std>/🪆️subsets/✳️<id>/` and MUST carry
all five children from `🔣️taxonomy.json`'s `subsetChildDirs` — the W1 policy rule
`policyStandardsCoverageBreaches` enforces this on-disk, and `policyStandardSubsetVocabularyBreaches`
enforces the manifest match (see §5):

| dir | rs | ts | purpose |
|---|---|---|---|
| `🧬️schema/` | yes | yes | `pub use …::subsets::any::schema::*` — NEVER a new type, see §2 |
| `🧐️analyzer/` | yes | yes | `DIALECT` const + `check_<subset>_conformance` fn + `impl ArtifactAnalyzer` |
| `🎹️composer/` | yes | yes | `impl ArtifactComposer` + `impl SubsetValidator` + `register()` — see §4 |
| `🏗️builder/` | yes | yes | `impl ArtifactBuilder`, `build()` re-runs the conformance fn |
| `🚪️io/` | yes | yes | doc-leaf ONLY — never duplicate the `✳️any` import/export tree |

The `.ts` leaf on every one of these is REQUIRED even though nothing typechecks against it today
— its absence was the one standing breach on the pre-W2 `✳️a-2b` pilot. Content is always the
same tiny shape (see `🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧐️analyzer/🟦️component.ts` for the
template):

```ts
/** <emoji> <TypeName> (✳️<id> subset) meta. */
export const meta = {
  artifactKind: "s.stdio.<artifact>",
  standard: "<std>",
  subset: "<id>",
} as const;
```

`🚪️io/🟦️component.ts` is the one exception — it has nothing to export, so it's just `export {};`.

## 2. Schema = pure `pub use`, always

A subset is a **validation-gated dialect stamp** on an existing snapshot type, never a new one
(D4's Tier-1 "same snapshot type, subset moves" semantics — `ArtifactCommand::MigrateDialect`).
`🧬️schema/🦀️component.rs` is ALWAYS exactly:

```rust
pub use crate::artifacts::<artifact>::standards::v<std>::subsets::any::schema::*;
```

No exceptions, regardless of whether the subset can hard-gate (1.7 `✳️a`) or is schema-gapped
(1.4 `✳️a`/`✳️x`). The `.ts` leaf is a `meta` export naming the subset, same template as above.

## 3. Analyzer: `DIALECT`, `CODE_*`, the conformance fn, severity convention

- `pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.<artifact>", standard: StandardId("<std>"), subset: SubsetId("<id>") };`
- `CODE_*` consts are namespaced `stdio.<artifact>.<subset>.<check-slug>` — kebab-case check
  slugs, e.g. `stdio.pdf.a.encrypt-present`, `stdio.pdf.a.schema-gap-unverifiable`.
- Severity convention, using `dsl::Severity` (only four variants exist — `Fatal`, `Error`,
  `Warning`, `Hint`; there is no distinct `Info`):
  - **HARD** finding (blocks the dialect stamp) → `Severity::Error`.
  - **SOFT** finding (advisory, doesn't block) → `Severity::Warning`.
  - **Purely informational, non-defect data** (e.g. a detected conformance LEVEL that isn't
    itself right or wrong) → `Severity::Hint`, the softest severity available. Document in the
    diagnostic message that this is informational, since the type system doesn't distinguish it
    from a weak warning.
- Every diagnostic uses `span: TextSpan::at(1, 1)` — these analyzers work over a decoded object
  graph or struct, not source text, so there is never a real span; `(1,1)` is the repo's
  established placeholder (see the pre-W2 `✳️a-2b` pilot).
- The real work is a pure fn: `pub fn check_<subset>_conformance(&Snapshot) -> Vec<Diagnostic>`.
  This is the SINGLE source of truth every other facet (analyzer, composer hard-gate, builder
  hard-gate, `SubsetValidator`) calls — never reimplement the check twice.
- `impl ArtifactAnalyzer`: `sniff` delegates to the `✳️any` analyzer's `sniff` (recognizing the
  container format is a shared, subset-independent question); `analyze` delegates the actual
  parse to `✳️any`'s `analyze`, then appends `check_<subset>_conformance`'s diagnostics on top of
  whatever `✳️any` already found. If any appended diagnostic is `Error`/`Fatal`, downgrade
  `confidence` to `IoConfidence::Low`.
- Inline `#[cfg(test)] mod tests` with one case per check (conforming case with zero/expected
  diagnostics, one violation case per HARD/SOFT check). Never a separate test file.

### When the engine can't support a check (the "honest schema gap")

If the snapshot type doesn't retain a field a real check needs (see PDF 1.4's `PageDoc` vs 1.7's
full `objects: Vec<PdfIndirectObject>` graph), you do NOT have license to fabricate a check
against an unmodeled field. Instead:
1. Implement the smallest set of checks that ARE genuinely verifiable from the fields that
   exist — even a weak signal is fine as long as it's real (PDF 1.4 `✳️a` checks `page.text`
   non-empty; `✳️x` checks `page.width/height` are non-degenerate). One real check is enough if
   that's honestly all the schema supports.
2. Always emit one `Severity::Warning` diagnostic, code
   `stdio.<artifact>.<subset>.schema-gap-unverifiable`, with a message of this exact shape:
   > "<Artifact> <std>'s retained snapshot has no <the missing structure>; full <the real spec
   > citation> conformance cannot be checked from this schema; upgrade <std>'s engine to retain
   > <the missing structure> (see <a sibling standard that already has it>) to implement real
   > checks here."
3. The composer and builder become PASS-THROUGH (§4/§ — no field exists to inject/strip/hard-gate
   on), but STILL implement `SubsetValidator` and call `register_subset_validator` — the W1 policy
   rule `policyStandardSubsetVocabularyBreaches` requires this on every real subset regardless of
   whether it can hard-gate. Document the pass-through nature plainly in the module doc comment
   rather than silently omitting the gate.

### Detecting DATA, not just PASS/FAIL

Some real-world conformance profiles bundle a family + a level/part in one colloquial name (e.g.
"PDF/A-2b" = family `a`, part `2`, letter `b`). The binding decision for this ticket: the subset
id is the FAMILY only; anything finer-grained is DATA the analyzer detects and reports as a
`Severity::Hint` diagnostic, never baked into the id or the directory name. See
`🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧐️analyzer/🦀️component.rs`'s `detect_pdfa_level` for the
template: return `Option<YourLevelEnum>`, document EXACTLY which axis is detectable from the
retained schema and which isn't (never guess the undetectable axis — pick an honestly-labeled
default and say so), and only return `Some` when there's a real basis for claiming the profile
applies at all.

## 4. Composer: hard gate (or pass-through) + `SubsetValidator` + registration

`🎹️composer/🦀️component.rs` for a subset that CAN hard-gate (1.7 `✳️a` shape):

```rust
const DIALECT_SELF: Dialect = Dialect { artifact_kind: "s.stdio.<artifact>", standard: StandardId("<std>"), subset: SubsetId("<id>") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.<artifact>", standard: StandardId("<std>"), subset: SubsetId("*") };
// + one const per catalog DAG dependency (e.g. binary, deflate)

pub struct <Name>Composer;
impl ArtifactComposer for <Name>Composer {
    type Snapshot = <Snapshot>;
    const WRITES: Dialect = DIALECT_SELF;
    fn reads() -> &'static [Dialect] { &[DIALECT_ANY, DIALECT_SELF, /* deps */] }
    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = <AnySubsetComposer>::compose(sources)?;
        let checks = check_<subset>_conformance(&inner.snapshot);
        let (hard, soft): (Vec<_>, Vec<_>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
        if !hard.is_empty() {
            let mut all = hard.clone(); all.extend(soft);
            return Err(ComposeError { message: format!("... {} hard issue(s) ...", hard.len()), diagnostics: all });
        }
        let mut diagnostics = inner.diagnostics; diagnostics.extend(soft);
        Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
    }
}
```

For a PASS-THROUGH subset (schema-gapped, no hard checks exist): skip the partition/`Err` branch
entirely — just extend `inner.diagnostics` with `check_<subset>_conformance`'s output and always
`Ok(...)`. Document why (see 1.4 `✳️a`/`✳️x`'s module doc comments).

Every real subset's composer ALSO carries, unconditionally regardless of hard-gate capability:

```rust
pub struct <Name>Validator;
impl SubsetValidator for <Name>Validator {
    const DIALECT: Dialect = DIALECT_SELF;
    fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <Snapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <Snapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_<subset>_conformance(&snapshot),
            None => vec![/* Warning: payload did not decode, skipped */],
        }
    }
}
static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<<Name>Validator>) }
pub fn register() { register_subset_validator(validator_entry()); }
```

This is the SAME `check_<subset>_conformance` fn the composer's own hard gate (if any) uses — one
source of truth, checked pre-serialization (authoritative) AND post-hoc against the wire payload
(the D5 validate-on-build hook, `io_dispatch`/`wire_artifact_compose`).

**Registration wiring** (two separate call sites, don't conflate them):
1. This subset's `register()` (the `SubsetValidator` registration above) is called from the
   STANDARD's own `⚙️engine::register()` — e.g.
   `crate::artifacts::pdf::standards::v1_7::subsets::a::composer::register();` inside
   `🏅️standards/🔖️1.7/⚙️engine/🦀️component.rs`'s `register()`.
2. This subset's `ComposerEntry` (the `ArtifactComposer` impl) is aggregated value-level by the
   STANDARD's own `🎹️composer/🦀️component.rs::entries()` — add
   `composer_entry_of::<<Name>Composer>()` to that `vec![...]` alongside the existing entries.
   Never register a `ComposerEntry` twice (once via `entries()`, once by hand) — `entries()` is
   the only call site.

## 5. Manifest (`🏅️standards/🔖️<std>/🪆️subsets/🔣️component.json`)

One manifest per STANDARD, shared across all its subsets — if it already has entries (e.g. from
`✳️any` or a sibling subset landed in the same wave), ADD your entry, never overwrite the file:

```json
{
  "artifact": "s.stdio.<artifact>",
  "standard": "<std>",
  "subsets": {
    "*": { "name": "Unconstrained <artifact> <std>" },
    "<id>": { "name": "<human name + real spec citation>", "levels": ["<optional level enum>"] }
  }
}
```

`policyStandardSubsetVocabularyBreaches` (W1, `📜️script.ts`) enforces, in BOTH directions, that
the manifest's declared subset ids exactly equal the on-disk `🪆️subsets/✳️<id>/` dirs for that
standard, that the id matches `^[a-z0-9][a-z0-9.\-]*$` (or `*`), and that every real (non-`*`)
subset's composer both `impl SubsetValidator` AND calls `register_subset_validator` (see §4).
`"levels"` is optional free-form data for whatever finer-grained enum the family has (PDF/A's
2b/2u/3b/3u, PDF/X's 1a/3, …) — never encode it in the id itself (§3's "Detecting DATA" note).

## 6. Glue handoff protocol

You may never edit `📦️glue.rs` (stdio) yourself — it's orchestrator-only, shared across every
concurrent unit in this ticket. Instead:
1. Read the CURRENT `pub mod <artifact> { ... }` block in
   `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` in full (note its exact start/end line
   numbers).
2. Write your FULL replacement block (every existing leaf, byte-for-byte, PLUS your new/renamed
   `#[path = "."] pub mod <id> { ... 5 leaves ... }` subset block(s)) to
   `<ticket>/🧩glue/<artifact>-glue.rs.txt`.
3. At the top of that file, state the exact anchor (current line span of the block being
   replaced) so the orchestrator can find-and-replace it verbatim.
4. If renaming/deleting an existing subset dir (e.g. `✳️a-2b` → `✳️a`), delete the old dir
   yourself (you own the artifact's files) but leave the glue.rs edit to the orchestrator — until
   they apply your snippet, `cargo check` on the shared tree will show exactly one expected error
   ("couldn't read .../<old-id>/.../component.rs") for the stale path. That is the correct,
   expected state to hand off in — do not attempt to work around it by touching glue.rs.

## 7. Self-verification given the glue.rs constraint

`cargo check -p semio-s-plugin-stdio` will NOT exercise your new subset's Rust files until the
orchestrator applies your glue snippet (they aren't referenced by any `#[path]` yet). Verify what
you can without touching glue.rs:
- Structural: brace/paren balance, that every import path matches an actually-`pub` item in the
  module you're importing from (cross-check against the sibling `✳️any` files you delegated to).
- Compare 1:1 against a subset that DID compile (the 1.7 `✳️a` pilot, or by the time you read
  this, whichever real subset landed most recently) for signature shape: `ArtifactAnalyzer`,
  `ArtifactComposer`, `ArtifactBuilder`, `SubsetValidator` trait method signatures don't change
  between subsets, only the types/consts plugged into them.
- Report the exact single expected error from `cargo check` (the stale/missing glue.rs path) in
  your `w<N>-<artifact>-report.json`'s `blockers` or a note, so the orchestrator knows that's the
  ONLY thing standing between your subset and a real green compile once their glue.rs edit lands.
