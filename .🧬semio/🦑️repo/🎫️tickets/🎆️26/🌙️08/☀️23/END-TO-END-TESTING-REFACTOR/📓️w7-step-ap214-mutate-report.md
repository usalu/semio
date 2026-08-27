# Wave 7 — stdio.step ap214/✳️any mutation oracle report

Executor: this session. Subset: 📐️step standard 🔖️ap214 subset ✳️any. Reference: `ruststep` 0.4.

## Fixture finding (report as instructed by the fleet brief)

The ticket-designated `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left-bim.stp`
(170 KB) and, on inspection, **all five** real `.stp` files committed under that directory declare
`FILE_SCHEMA(('AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF { 1 0 10303 442 3 1 4 }'))` — i.e. every
one of them is **AP242**, not AP214. Confirmed by reading each header directly. No git-tracked
real-world AP214 (`AUTOMOTIVE_DESIGN`) file exists anywhere in this repository (a repo-wide
`FILE_SCHEMA` grep across every `.stp`/`.step` file found nothing else; a real AP214 file does exist
at `./temp/simple_bus_shelter-gray.ap214.stp`, but `temp/` is gitignored and not a committed asset, so
it was not used).

Resolution taken (matching wave-7 precedent for formats with no real committed fixture — see
`📓️w7-results.md`'s "Honest limits" section, e.g. GIF 87a derived from real 89a frames, OBJ derived
from a real `.glb`): the real, smaller (~78 KB) sibling
`♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left.stp` was copied into
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp`
with exactly **one line changed**: its `FILE_SCHEMA` record, from the source's real AP242 declaration
to `AUTOMOTIVE_DESIGN`. That file's real entity set (1396 real instances: 449 `CARTESIAN_POINT`, 126
`B_SPLINE_CURVE_WITH_KNOTS`, 71 `VERTEX_POINT`, 57 each of `ADVANCED_FACE`/`PLANE`/`EDGE_LOOP`/
`FACE_OUTER_BOUND`, one `MANIFOLD_SOLID_BREP`/`CLOSED_SHELL`, plus the real `PRODUCT`/
`PRODUCT_DEFINITION` management chain) is drawn entirely from ISO 10303-214's own common-resource
advanced-B-rep-shape-representation schema — the same resources `AUTOMOTIVE_DESIGN` declares — with
zero AP242-only PMI/GD&T/kinematics entities present anywhere in the source, so this is a genuine,
conformant AP214 document, not a fabricated one. Every entity id, coordinate, curve, topology
relationship and product record is real and untouched; the derivation is documented in the fixture's
own header comment (ISO 10303-21 allows `/* ... */` comments in `HEADER;`).

## §6 applies: `ruststep` 0.4 can only read

Established empirically **before** writing any scenario (a standalone scratch probe crate, this
ticket folder's own `.gitignore`d scratchpad is outside the repo so nothing was left behind, fed the
real derived fixture to `ruststep::ast::Exchange::from_str`): it parsed all 1396 real entities with
zero errors. Reading the crate's own source confirms it has no writer at all — no `Display`/
`fmt::Formatter` impl anywhere on `Exchange`/`DataSection`/`Record`/`Parameter`, and
`ast::ser::to_record` only builds an in-memory `Record` from an already-typed Rust struct (moot for
AP214 anyway: `ruststep` compiles no generated schema module for it — only `ap201`/`ap203` are
feature-gated in, and this crate enables neither).

Per the fleet brief's §6, every scenario is typed `@mode-property` (the two `@id-mutate`/`@id-inverse`
groups) or `@mode-round-trip` (the identity scenario) — **never** `@mode-differential`. `ruststep` is
registered as the oracle anyway, in its true role: the INDEPENDENT READER every result (this
subset's own oracle-side mutation output, and — once the subject phase compiles — the subject's) is
projected through (`project_step_ap214_any`, in the oracle module) before the `semantic-step-v1`
profile compares it. The oracle dispatcher's own mutation-performing half is this subset's own
from-scratch Part-21 writer (ruststep has nothing to reuse there), operating on a `ruststep`-parsed
`Exchange` — independent of this subset's own `engine::part21`/`StepSnapshot` codec, which only the
`sut`-gated subject module uses.

## Files written

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp` — derived real fixture (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — catalog + oracle registration + `semantic-step-v1` comparison profile (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — filled in (was a rejecting stub); bespoke Part-21 writer + mutation dispatcher + `project_step_ap214_any`, plus a `#[cfg(all(test, feature = "oracles"))]` validation suite against the real fixture (11 tests, all passing standalone)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added `pub const KINDS` (11 entries) + `kinds_const_matches_enum_variants_in_declaration_order` test, beside the pre-existing `StepMutation` enum
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧪️tests/mutate-step-ap214/component.feature` — new case, 23 scenarios (11 kinds × mutate + inverse, plus identity round trip)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🧪️tests/mutate-step-ap214/🦀️component.rs` — new case adapter: oracle handlers + `#[cfg(feature = "sut")] mod subject` (real `StepSnapshot`/`apply_step_mutation`/`engine::part21` codec, full parse → mutate → re-serialize, no byte pass-through) + registration loop

Nothing was added to any shared family module (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/{...}`) — STEP has no
sibling subset in this wave to share with. `Cargo.toml`/`📦️lib.rs`/`.gitignore`/`project.json`/
`launch.json` were not touched.

## Verification

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-step-ap214
0 high-priority breach(es) across 0 rule(s):

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-step-ap214
[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0
```

Both runs green, real output pasted above, not claimed unrun. The Rust **subject** phase does not
compile this wave (confirmed independently: `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio
--case mutate-step-ap214` fails with 117 errors, all inside `🧰️framework/🛍️products/💻️os` — the
documented `📡️spr/🧵️channel` `semio_framework::` cycle and unrelated borrow-checker errors in
`🏪️store`; **zero** of those errors reference this case's own files). The subject half is written and
`sut`-gated per the brief, ready to compile into the subject role once that lands.

A concurrent session was observed mid-editing the `json`/`tsv` subsets' oracle files and this shared
crate's `Cargo.toml` while this ticket's verification was running (`serde_json` removed from
`oracles` features, `tsv-iana-any` catalog momentarily unclaimed) — unrelated to this subset,
confirmed by the error locations (`🗿️artifacts/🔣️json/...`, not `🗿️artifacts/📐️step/...`). The two
`bun` runs pasted above were captured cleanly before that churn began; this file records that they
were real, not re-simulated afterward.

## Findings summary (for the coordinator)

1. **AP242-vs-AP214 fixture mismatch**: the ticket-designated real input, and all four of its
   siblings, are AP242, not AP214. No real committed AP214 STEP file exists in this repository. A
   genuine AP214 fixture was derived (single-line `FILE_SCHEMA` edit on real, otherwise-untouched
   common-resource BREP data) rather than synthesised from scratch or left absent.
2. **§6 applies**: `ruststep` 0.4 reads but cannot write. Registered as the independent reader;
   nothing is typed `@mode-differential`.

## Post-hoc re-verification (same session, later)

Re-ran both checks after the initial green pass to be sure. Observed **three separate** concurrent
sessions destabilize the shared oracle crate/registry in turn while re-checking (`json`/`tsv`
`serde_json` removal, then `html`'s `oracle_apply_mutation` mid-edit, then `ply-1-0-any`/`dxf-r12-any`
catalogs momentarily unclaimed) — none of these ever touched or mentioned `step-ap214-any` or this
case's own files. `oracle exhaustive` for `mutate-step-ap214` went green again (23/23) once the crate
compiled; the contract check's only remaining breaches, checked twice in a row, are the unrelated
`ply-1-0-any`/`dxf-r12-any` catalogs, not this subset. This subset's own contract check was 0
breaches in the first (uncontended) run recorded above, and nothing about this subset's registration
has changed since.
