# Wave 8 — stdio.ifc 2x3/✳️any mutation oracle report

Executor: this session. Subset: 🏗️ifc standard 🔖️2x3 subset ✳️any. Reference: `ruststep` 0.4
(already linked, already registered for the sibling `step/🔖️ap214/✳️any` subset this wave).

## Vocabulary (confirmed by reading, not assumed)

`Ifc2x3Mutation` (`.../🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) declares 5 kinds:
`NoMutation`, `SetSnapshot`, `UpsertInstance`, `RemoveInstance`, `SetHeader` — real per-instance
vocabulary (upsert/remove operate on `Part21Instance`, a real `{id, entities: Vec<(name, args)>}`
graph node), richer than the sibling `4` standard's `{NoMutation, SetSnapshot}` stub. Added
`pub const KINDS` + `kinds_const_matches_enum_variants_in_declaration_order` beside the enum, per
the fleet brief.

## §6 applies: `ruststep` 0.4 can only read

IFC2X3 is physically ISO 10303-21 (STEP physical file) syntax under a different EXPRESS schema —
`ruststep` parses it exactly as it parses the sibling `step/🔖️ap214/✳️any` subset, which already
established this exact §6 situation this wave: no writer exists anywhere in the crate (confirmed
again by reading `ast::ser::to_record` and grepping for `Display`/`fmt::Formatter` impls — none).
Every scenario is typed `@mode-property` (the two `@id-mutate`/`@id-inverse` groups) or
`@mode-round-trip` (the identity scenario) — never `@mode-differential`. The oracle dispatcher's own
mutation-performing half is this subset's own from-scratch Part-21 writer (independent of this
subset's own production `step::engine::part21` codec — reusing that would compare this repository's
implementation against itself, the exact failure mode this platform exists to prevent), operating on
a `ruststep`-parsed `Exchange`.

## Real input and derivation

`temp/wellness-center-sama.ifc` (21,282,588 bytes) — its `FILE_SCHEMA(('IFC2X3'));` line and header
`* Schema: IFC2X3` comment were both read directly to confirm it. 21 MB is too large to copy into
every work directory, so a real, self-consistent SUBSET was derived once (never all 21 MB
committed), per the ticket's own instruction, rather than committing the whole file:

- Parsed all 409,102 real `DATA;` entities (one Part-21 record per line, confirmed) with a scratch
  Python script (`ifc-2x3-oracle-scratch/01-parse-ifc.py`, this ticket folder).
- Picked real `IFCBUILDINGSTOREY` #139 ("Street level") — one of 5 real storeys — because its own
  real `IFCRELCONTAINEDINSPATIALSTRUCTURE` names a modest, real 14-element mix (slabs, a ramp, two
  wall-standard-cases, three columns, five building-element proxies, a stair), not the largest or
  the smallest storey.
- Root set = the storey + its real spatial-structure ancestor chain (`IFCRELAGGREGATES` up through
  `IFCBUILDING`/`IFCSITE`/`IFCPROJECT`) + its containment relationship + its 14 real elements +
  every real `IFCREL*` relationship anywhere in the source that references the storey or any of
  those 14 elements (72 real relationships: 56 `IFCRELDEFINESBYPROPERTIES`, 6
  `IFCRELASSOCIATESMATERIAL`, 6 `IFCRELDEFINESBYTYPE`, 2 `IFCRELAGGREGATES`, 1
  `IFCRELVOIDSELEMENT`, 1 `IFCRELCONTAINEDINSPATIALSTRUCTURE`).
- Forward-reference closure of that root set to a fixed point (`ifc-2x3-oracle-scratch/
  02-derive-street-level-subset.py`): every `#id` any kept entity points to is itself kept.
  **Verified zero dangling references** in the derived file — checked programmatically, not
  assumed.
- Result: 3464 of the 409,102 real entities, 193,915 bytes, committed at
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️wellness-center-sama-street-level.ifc`. Every
  id, coordinate, geometry definition and relationship is real and untouched — nothing renumbered,
  nothing synthesised. The derivation is documented in the fixture's own `HEADER;` comment (ISO
  10303-21 allows `/* ... */` there, same precedent the sibling AP214 fixture uses) and in the
  feature file's own Feature description.
- `git check-ignore -v` on the committed fixture: matched `.gitignore:624: !**/🧫️fixtures/**`
  (a negation rule) and `git status --porcelain` shows it as a plain untracked (`??`) addable file —
  confirmed trackable. `temp/` itself stays gitignored; only the derived slice inside `🧫️fixtures/`
  is committed.

## The deliberate real-integrity exercise (ticket instruction)

`remove-instance`'s own scenario removes `#270549`, a real `IFCWALLSTANDARDCASE`. In the FULL 21 MB
source it is referenced by 8 real entities; 7 of those (5 `IFCRELDEFINESBYPROPERTIES`, 1
`IFCRELASSOCIATESMATERIAL`, 1 `IFCRELDEFINESBYTYPE`, plus the storey's own
`IFCRELCONTAINEDINSPATIALSTRUCTURE`) are carried into this fixture's own closure specifically so the
removal's real effect is observable inside the committed fixture, not just asserted from outside it.

**Chosen strategy: mechanical, no cascading integrity check** — matching production
`Ifc2x3Mutation::RemoveInstance`'s own bare `.retain(|instance| instance.id != *id)` exactly. The
oracle-side unit test `remove_instance_deletes_a_referenced_real_wall_and_leaves_a_documented_
dangling_reference` (`.../🧪️oracle/🦀️component.rs`) asserts the removal succeeds AND that the real
`IFCRELCONTAINEDINSPATIALSTRUCTURE` (`#710858`) still lists a now-dangling `#270549` reference
afterward — recorded as a real, deliberate finding, not hidden by picking an unreferenced entity or
by silently repairing the reference. `remove-instance`'s own inverse is a cross-kind
`upsert-instance` re-inserting the exact original entity (same id), which heals the dangling
reference by construction — the same cross-kind inversion pattern the sibling `step/🔖️ap214/✳️any`
subset's `insert-entity`/`remove-entity` pair already uses.

`remove-instance` on an absent id is a hard error (`remove_instance_of_absent_id_is_an_error_not_a_
silent_no_op`), a deliberately stricter design than production's own silent-no-op `retain` — chosen
so a mistyped id in a scenario fails loudly rather than quietly passing.

`upsert-instance`'s own scenario updates an existing real entity (`#619887`, a real `IFCCOLUMN`,
also referenced by 7 real entities) — whole-instance replace, matching production's `Some(existing)
=> *existing = instance.clone()` semantics exactly (not a per-argument patch: this subset's
vocabulary, unlike the sibling STEP subset's finer-grained `set-entity-arg`, only ever replaces a
whole instance). A second oracle-only unit test (`upsert_instance_appends_a_brand_new_id`) exercises
the OTHER upsert branch — appending a brand-new id at the end, matching production's `next.document.
instances.push(...)` — even though the feature file's own single Examples row for `upsert-instance`
exercises the update branch (only one row is allowed per kind per the parser rules).

## Files written

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️wellness-center-sama-street-level.ifc` —
  derived real fixture (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`
  — catalog + oracle registration + `semantic-ifc-v1` comparison profile (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` —
  filled in (was a rejecting stub); bespoke Part-21 writer + mutation dispatcher +
  `project_ifc_2x3_any`, plus a `#[cfg(all(test, feature = "oracles"))]` validation suite against the
  real fixture (10 tests, all passing standalone — `cargo test --features oracles v2x3` from
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
  🦀️component.rs` — added `pub const KINDS` (5 entries) +
  `kinds_const_matches_enum_variants_in_declaration_order` test, beside the pre-existing
  `Ifc2x3Mutation` enum (only edit to this pre-existing file)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/mutate-ifc-2x3/component.feature` — new case, 11
  scenarios (5 kinds × mutate + inverse, plus identity round trip)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/mutate-ifc-2x3/🦀️component.rs` — new case adapter:
  oracle handlers + `#[cfg(feature = "sut")] mod subject` (real `Ifc2x3Snapshot`/
  `apply_ifc2x3_mutation`/`decode_ifc2x3`/`encode_ifc2x3` codec, full parse → mutate → re-serialize,
  no byte pass-through) + registration loop
- `.🧬semio/.../END-TO-END-TESTING-REFACTOR/ifc-2x3-oracle-scratch/` — the two derivation scripts
  (this ticket folder, per the fleet brief's scratch-file rule)

Nothing was added to any shared family module (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/{document,raster,
archive,audio,tabular,mesh}`) — no sibling subset shares this Part-21 writer in a way that fits one
of those six families, and adding a new shared module would mean editing `Cargo.toml`/`📦️lib.rs`,
which the fleet brief forbids; the writer is duplicated from the sibling `step/🔖️ap214/✳️any`
subset's own equivalent for the identical reason that subset's own report already gives.
`Cargo.toml`/`.gitignore`/`project.json`/`launch.json` were not touched. `🏗️ifc 🔖️4` was not touched
(confirmed by `git status`: the only uncommitted changes under that standard belong to a concurrent
peer session, not this one).

## Verification

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-ifc-2x3
0 high-priority breach(es) across 0 rule(s):

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-ifc-2x3
[test] level=exhaustive cases=1 executed=11 passed=11 failed=0 errored=0 parity=0/0
```

Both re-run a second time back-to-back for stability; identical results both times. The full breach
cache (`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`) was also grepped directly for any mention of
`ifc` — zero matches, confirming no breach (blocking or non-blocking) names this subset.

`cargo test --features oracles v2x3` from `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`: 10/10
passed (the oracle module's own embedded unit tests against the real fixture, including the
deliberate dangling-reference case above).

**The Rust SUBJECT phase could not be checked**, for a different but equally unrelated reason than
the documented `📡️spr/🧵️channel` os-kernel cycle: `cargo check --features sut` from
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` fails at the WORKSPACE-MANIFEST level, before reaching any
of this subset's own files, because a concurrent session has `✏️s/🔌️plugins/✒️writer/📦️packages/
🦀️rust/Cargo.toml` mid-edit (`dependency.js-sys` not found in `workspace.dependencies` — a sibling
plugin's `Cargo.toml`, not this artifact's). This matches the fleet brief's own explicit expectation
("Rust SUBJECT phase cannot compile — expected; verify ORACLE only") and this repository's own
recorded pattern for concurrent workspace churn; it was not chased. The subject module was written
in full, `sut`-gated, and carefully hand-verified against the same real ids/values the oracle's own
passing unit tests already exercise (`#619887`, `#270549`, the real header fields) — it is ready to
compile into the subject role the moment the workspace is uncontended, same as the sibling
`step/🔖️ap214/✳️any` subject module already is.
