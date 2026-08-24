# Wave 7 — semio Pattern-A subsets `✳️model` and `✳️presentation`

Date: 2026-08-24. Ticket: 26/08/23/END-TO-END-TESTING-REFACTOR.
Scratch, generators and raw verification output: `w7-semio-model-presentation/`.

## Headline

Both subsets are now covered end to end at the contract level and neither could be given a real
oracle. Two new cases exist, `mutate-semio-model` (23 scenarios) and `mutate-semio-presentation`
(31 scenarios), both contract-clean and both **`not-exercised` in the oracle phase** — which is what
a recorded no-oracle decision means in this runner, exactly as the seven landed Pattern-B semio cases
already behave. Their evidence is written and waits on the blocked subject phase.

## 1. The mutations already existed and are genuinely their own

Step 1 of the brief did not apply. Both subsets already own a handcrafted
`🧬️schema/🧬️mutations/🦀️component.rs` with hand-written `diff()`/`inverse()` per variant, and both
are genuinely distinct from `✳️any` rather than a copy of it:

* `✳️any`'s `SemioMutation` is a **dispatch envelope** — 18 newtype variants (`Brep`, `Mesh`, `Model`,
  … `Kit`) that delegate to a sibling subset. It is not a vocabulary these two could inherit.
* `SemioModelMutation` is 11 kinds over three ID-keyed collections (spatial / elements / relations),
  modelled on IFC's `IfcRelAggregates` / `IfcRelContainedInSpatialStructure` shape.
* `SemioPresentationMutation` is 15 kinds over an **index-addressed** slide and shape tree plus two
  ID-keyed master/layout collections, modelled on pptx's `p:sld` / `p:sldMaster` / `p:sldLayout`.

Nothing was invented to manufacture a difference. What was missing was the declaration, the catalog
and the case.

## 2. `pub const KINDS` and its conformance test

Both files already carried `const OP_KEYWORDS: [&str; N]` — the exact kebab-case keyword list, in
enum declaration order, indexed by `variant_ordinal` for the binary op frame's `tag` byte. So `KINDS`
aliases it rather than restating it:

```rust
pub const KINDS: &[&str] = &OP_KEYWORDS;
```

`kinds_match_the_enum_and_the_catalog` (plain `#[test]`, never `async_test`) was added to each file's
own test module. It asserts two things: every enum variant reaches `KINDS` at its own
`variant_ordinal` under exactly the keyword `print_op` emits — `demo_mutation_cases()` supplies one
instance per variant — and `KINDS` equals the catalog's `kinds` array read straight out of
`../../🧪️oracle/🔣️component.json` with `include_str!`.

**Not run.** The production crate does not compile (see §6), so like every other subset's `KINDS`
test this one is written and waiting. The catalogs were instead audited by hand against the enums:
11/11 and 15/15, in order.

## 3. No oracle, and why — both candidates were surveyed, not assumed away

Cross-language host support **has** landed and works: `bun ./📜️script.ts oracle exhaustive --owner
🗄️stdio --case extract-text-pdf-1-4` executes 2 Python (`pypdf`) scenarios green today. So "no
Python library" was not available as an excuse and was not used as one.

**`✳️presentation` → `python-pptx`, rejected on three findings.** It is the obvious candidate: the
subset is deliberately pptx-shaped, and a real `.pptx` fixture is committed in this repository.
(a) Reaching a `SemioPresentationSnapshot` from pptx bytes requires this repository's OWN
`🚪️io/📥️import`/`📤️export` pptx bridge, so a differential would compare our importer against our
exporter with a third party merely re-reading the result — the self-comparison the platform exists to
prevent. (b) `python-pptx` cannot create slide masters or slide layouts at all, which leaves
`insert-master`, `remove-master`, `insert-layout`, `remove-layout` and `set-layout-master` — a third
of the vocabulary — with no reference counterpart. (c) `set-snapshot`'s semantics are
`SemioPresentationDiff::between`, a whole-state structural comparison no presentation library models.

**`✳️model` → IfcOpenShell / `ruststep`, rejected on the same first finding.** Both are authoritative
over IFC, neither is authoritative over `SemioModelSnapshot`, and the only path between them is again
our own IFC bridge.

Recorded as `semio-model-mutation-semantics` and `semio-presentation-mutation-semantics` in the two
new `🧪️oracle/🔣️component.json` manifests, `substitutes: ["specification-vectors",
"metamorphic-laws"]`, with the rejection reasoning written into the rationale rather than left in a
report.

## 4. What replaces the oracle: a real artifact, not an invented one

The before-state of every vector is the **real committed example artifact** of its subset:

| Subset | Asset | What it really is |
|---|---|---|
| `✳️model` | `🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio` | site → ground storey, one external wall with a `Pset_WallCommon` fire-rating property set (`IsExternal`/`REI60`/`0.24`), one `containedIn` relation |
| `✳️presentation` | `…/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio` | one master (title placeholder), one layout (subtitle placeholder), one slide whose shape tree exercises **all four** shape kinds — text box, embedded PNG picture, one-cell table, `other`-typed placeholder — plus a speaker-notes page |

Each subset's own `fixture_honesty_law` asserts that asset is byte-identical to its
`demo_semio_*_snapshot()`, so the fixture cannot silently drift back into a synthetic one.

That the committed JSON vectors really transcribe those assets is **checked, not asserted**. Two
independent scripts (`w7-semio-model-presentation/verify_fixture_vs_asset.py` and `verify_numbers.py`)
walk the raw `.dsl.semio` text, hex-decode every string run and unpack every IEEE-754 bit pattern,
and confirm every leaf value of the committed before-fixture is recoverable from the real asset:

```
model:        13 distinct fixture strings, 0 not recoverable from the real asset
presentation: 10 distinct fixture strings, 0 not recoverable from the real asset
model:         2 distinct non-trivial fixture numbers, 0 not recoverable
presentation:  9 distinct non-trivial fixture numbers, 0 not recoverable
```

On top of that, each case carries an `identity-round-trip` scenario that reads the real `.dsl.semio`
AND its `.pack.semio` sibling directly through `asset://`, asserts both envelopes decode to the same
snapshot, carries it back through pack and dsl in turn, and compares against the committed vector.
Nothing in that path transcribes the model.

Fixtures live in each case's own `🧫️fixtures/<kind>/{⬅️before,🦠️mutation,➡️after}.json` (33 files for
model, 45 for presentation), generated by `gen_model.py` / `gen_presentation.py` which implement the
apply semantics read out of `🔺️diff/🦀️component.rs` (`apply_named` = retain → patch → **append**;
`apply_indexed` = patch → remove descending → insert at index). They are **not** in per-kind
`🧬️mutations/<kind>/🧪️tests/` leaves the way `brep`/`table`'s are, because those leaf directories do
not exist for these two Pattern-A subsets and creating 10 + 14 of them belongs to the migration ticket
that owns leaf creation, not to this one.

## 5. Findings

**F1 — `InsertSpatialNode`/`InsertElement`/`InsertMaster`/`InsertLayout` carry no position, so the
inverse of a non-terminal remove is wrong.** `apply_named` appends. Removing member *i* of an ID-keyed
collection and applying the mutation's own computed inverse restores the member at the END, not at
*i*. Since `SemioModelSnapshot`/`SemioPresentationSnapshot` compare by `PartialEq` over `Vec` and the
`.dsl.semio` encoding is order-dependent, that difference is observable — the inverse law is simply
false for a non-terminal remove. The affected fixtures therefore remove an APPENDED member (the
before-state is the real artifact after one declared preparatory insert, stated in the feature
description), which is the only shape under which the law holds. Slides and shapes are unaffected:
they are index-addressed and `InsertSlide`/`InsertShape` carry the exact final index.

**F2 — removing the real committed members would leave dangling references, and nothing stops it.**
`storey-1` is referenced by `wall-1.spatialId` and by `rel-1.to`; `master1` by `layout1.masterId`;
`layout1` by `slide1.layoutId`. `MutationDiff::apply` does not check referential integrity — only the
composer's `SemioModelValidator`/`SemioPresentationValidator` do, at compose time. So a mutation can
put the snapshot into a state its own validator rejects. That is a defensible event-sourced design
(you would fix it up in a later step), but it is undocumented and worth a deliberate decision.

**F3 — the TypeScript mutation mirrors have drifted from the Rust wire form.**
`🧬️mutations/🟦️component.ts` declares `parentId`, `spatialId`, `slideIndex`, `shapeIndex`, `layoutId`,
`masterId`. The Rust wire form keeps `parent_id`, `spatial_id`, `slide_index`, `shape_index`,
`layout_id`, `master_id`, because `#[serde(rename_all = "camelCase")]` on an **enum** renames variants
only — field renaming would need `rename_all_fields`. (Struct types are unaffected: `SlideLayout` is
correctly `masterId` on the wire.) A TypeScript implementation written against those mirrors would
emit payloads the Rust decoder rejects. Left as-is and reported: fixing it is a cross-language schema
decision, not this case's business.

**F4 — the composite-subset reachability wall, again, and worked around the way `kit` did.** Both
subsets are unreachable from an owner-root test adapter as shipped: `protocol::Mutation`,
`store::ArtifactDsl` and `store::ArtifactPack` all live behind **private** `extern crate
semio_framework_os_kernel as …` aliases in `📦️glue.rs`, and `store::TextError`/`store::PackError`
cannot even be named in a signature. Six thin wrappers were added in the subsets' own production code,
naming only already-exported types:
`semio_model_mutation_inverse` / `semio_presentation_mutation_inverse`,
`parse_/print_semio_*_dsl`, `encode_/decode_semio_*_pack`.

This also means the existing `mutate-semio-{mesh,graph,table,brep,text,object,kit}` adapters' `use
protocol::Mutation;` **cannot compile** — the generated host crate depends only on
`semio-repo-test-host`, the contributed oracle packages and (behind `sut`) the subject crate; there is
no `protocol` crate in its manifest. Nobody has noticed because the subject phase has never built.
Not fixed here — those cases belong to other owners — but it is the same structural finding the wave-7
results already recorded, now with the precise mechanism.

## 6. Verification — the real output

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`. Full transcript:
`w7-semio-model-presentation/📊️verification.txt`.

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-model
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-presentation
```

Both report **zero breaches attributable to these cases**. The command still exits 1 and prints
`zip-2-0-iso21320`, `svg-1-1-basic`, `svg-1-1-tiny` (and, earlier in the session, docx/pptx/xlsx/ifc
rows) — the unclaimed-catalog and no-adapter checks answer a repository-wide question regardless of
`--case`, and every one of those rows belongs to a peer session's in-flight case. Machine-checked:
filtering the full breach set for `semio-model` / `semio-presentation` / `semio-v1-model` yields
**0 of 10** records.

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-model
[test] not-exercised …/mutate-semio-model (recorded no-oracle decision semio-model-mutation-semantics
       — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-presentation
[test] not-exercised …/mutate-semio-presentation (recorded no-oracle decision
       semio-presentation-mutation-semantics — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
```

**`executed=0` is the honest result, not a pass.** A recorded no-oracle decision means
`oracleDecision` returns no implementation, so the runner has nothing to execute — the identical
output the already-landed `mutate-semio-table` produces today. Reporting these as green would be
precisely the failure the `not-exercised` counter was added to prevent.

Because the runner never builds the adapter for a no-oracle case, the adapter would otherwise have
gone completely unchecked. It was therefore compiled standalone against `semio-repo-test-host` alone
(`w7-semio-model-presentation/adapter-check/`, its own `[workspace]` so the shared root manifest is
untouched):

```
$ cargo run --manifest-path adapter-check/Cargo.toml
model oracle scenarios: 23          # 11 kinds × 2 + identity-round-trip
presentation oracle scenarios: 31   # 15 kinds × 2 + identity-round-trip
```

Zero warnings beyond `unexpected cfg condition value: sut`, which is an artifact of the scratch crate
not declaring the feature the generated host declares. This proves the oracle half compiles and
registers exactly the scenario ids the completeness gate expects, and that the `sut`-gated subject
half at least parses. It does **not** prove the subject half type-checks.

### The subject phase is blocked upstream

```
$ bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-model
error[E0599]: no method named `generation` found for struct `WorkerJobSession<J>` …
error[E0308]: mismatched types    (×4)
error[E0499]: cannot borrow `self.rejected` as mutable more than once at a time
error: could not compile `semio-framework-job` (lib) due to 6 previous errors
[test] …/mutate-semio-model: no result stream at …/📤️results.jsonl
```

Six errors, all in `semio-framework-job`, a framework crate another session is mid-refactor in.
`cargo check -p semio-s-plugin-stdio` fails the same way. Nothing of this case's code is reached, and
consequently the whole subject half — the JSON decoders, the projections, the six new production
wrappers and both `kinds_match_the_enum_and_the_catalog` tests — has never been compiled. That is the
honest limit of this deliverable.

## 7. Files

Production (subset-owned):
* `…/🪆️subsets/✳️model/🧬️schema/🧬️mutations/🦀️component.rs` — `pub const KINDS`, `semio_model_mutation_inverse`, `kinds_match_the_enum_and_the_catalog`
* `…/🪆️subsets/✳️model/🧬️schema/📸️snapshot/🦀️component.rs` — `🔖️ReachableCodecs` region (4 wrappers)
* `…/🪆️subsets/✳️model/🧪️oracle/🔣️component.json` — new
* `…/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/🦀️component.rs` — same three additions
* `…/🪆️subsets/✳️presentation/🧬️schema/📸️snapshot/🦀️component.rs` — `🔖️ReachableCodecs` region
* `…/🪆️subsets/✳️presentation/🧪️oracle/🔣️component.json` — new

Cases (new), under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/`:
* `mutate-semio-model/{component.feature,🦀️component.rs,🧫️fixtures/…}` — 33 fixture files
* `mutate-semio-presentation/{component.feature,🦀️component.rs,🧫️fixtures/…}` — 45 fixture files

Nothing outside these two subsets and their two cases was edited. No shared manifest, no framework
file, no `Cargo.toml`, no `📦️glue.rs`, no `project.json`, no `launch.json`.
