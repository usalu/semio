# 📓️ `s.stdio.dxf@r12/✳️any` — reader-oracle retrofit

Scope: register a genuinely independent READER oracle (`dxf` 0.6, kind `third-party-library`) for
this subset's 19-kind mutation catalog, alongside its existing `dxf-crate-r12-mutate`
(`cross-semio-implementation`, computes what a mutation should produce via
`🧪️oracle/🦀️component.rs`) — left byte-for-byte untouched. Follows the pattern already proven on
`avi`/`bcf`/`docx` (`📓️the-remaining-map.md`, `avi`'s `🏭️generator`/`🔬️probes`/`🧪️oracle/🔣️.json`).
Everything below was run, not asserted.

## What was already in place (verified, not assumed)

A prior agent's session (uncommitted, same working tree, `📓️dxf-r12-any-fixture-corpus.md`) had
already: reclassified `dxf-crate-r12-mutate` to `cross-semio-implementation`; declared 19
`mutationManifests` entries with `oracleRequirements[].qualifyingKind: "third-party-library"` but
`oracle` still wrongly pointing at the cross-semio id (the exact mismatch this ticket closes); built
the `drafting-plate` base fixture and `🏭️generator/🦀️engine` wrapper crate (`dxf 0.6` only); and
per-kind-witnessed all 19 mutations against `drafting-plate` (every kind moves the projection, every
inverse restores it exactly — 0 residual differences). That witnessability work was reused, not
redone.

## Delivered

```
🏭️generator/📜️script.ts                extended: RECIPES table (drafting-plate + 37 new), --only, generate|manifests
🏭️generator/🦀️engine/src/main.rs        extended: base_doc()/recipe()/RECIPE_IDS, build|project|list-recipes|perturb-radius-debug
🔬️probes/📜️script.ts                    NEW: dxf-import / dxf-project / dxf-compare
🧪️oracle/🔣️.json                        + 1 oracle, + pipeline ref, + 3 probes, + 1 comparisonPipeline,
                                        19 oracleRequirements retargeted, + 37 fixtureManifests
🧫️fixtures/<37 new recipe dirs>/        58 files total (1 drafting-plate + 40 before/after pairs + 17 before-only)
```

`🧪️oracle/🦀️component.rs` — **0 diff** against git HEAD (`git diff | wc -l` → `0`). The
`dxf-crate-r12-mutate` oracle entry's `kind`/`rationale` are exactly as the prior session left them.

## Step 0/1 — carrier and witnessability

Inherited from the prior session's work, re-verified rather than re-derived: the subset's serializer
IS the real DXF ASCII group-code stream (`print_dxf_document`/`parse_dxf_document`,
`🧬️schema/📸️snapshot/🦀️component.rs`), not a `print_dsl` stub. All 19 declared kinds are
witnessable via `dxf` 0.6 — none is `-uncarried` at the kind level. The one pre-existing narrowing,
carried forward: `set-header-var`/`remove-header-var` are witnessed **only** for `$INSBASE` — `dxf`'s
`Header` is a fixed generated struct with no arbitrary `$VAR` slot, and no other header variable this
subset could target survives an R12 save/reload through this reference library at all.

## Two NEW findings from this retrofit (verified from the code, not assumed)

**1. `set-header-var` has no reachable `rejected` outcome against a well-formed base document.**
Read directly from `🧬️schema/🧬️mutations/🦀️.rs:239-242` (`DxfMutation::diff`) and
`🧬️schema/🔺️diff/🦀️component.rs:1774-1787` (`diff_set_header_var`) +
`🔺️diff/🦀️component.rs:1523-1578` (`validate_named_targets`):

* if the target name already exists, `diff_set_header_var` emits exactly one `modified` entry
  against that unique name — `validate_named_targets`'s modify-path only rejects an ABSENT or
  DUPLICATED name, and a real document never has either for its own `$INSBASE`;
* if the target name is absent, it emits exactly one `added` entry at `index =
  header_vars.len()` — `index > length` is never true there, so the add-path never rejects it either.

Every reachable branch succeeds. Reaching `invalid-modify-target` needs a BASE document whose
`header_vars` already contains the target name more than once, which no real DXF writer (`dxf`
included — `Header` has one `insertion_base` field, not a repeatable list) can produce. This is
genuinely different from `SetLayer`/`SetStyle`/`SetLinetype`, whose `diff()` ALWAYS emits a
`modified` entry via `.unwrap_or_default()` regardless of presence (`🦀️.rs:247-250` etc.) — an
absent name there fails the modify-path's uniqueness check for real. So `set-header-var` carries
`set-header-var-applied` only in the new corpus; no `set-header-var-rejected-*` — recorded in the new
oracle's own `rationale` field, not silently dropped. `remove-header-var` has no such problem (its
target can simply be an absent name), and does carry `remove-header-var-rejected-missing`.

**2. `dxf` 0.6's own LOADER (not its writer) resynthesizes a removed-but-still-referenced
LAYER/LTYPE row with default values.** Found empirically while verifying `remove-layer-applied`
(never assumed): `Drawing::load` parses `ENTITIES` via its own `add_entity`, which calls
`ensure_layer_is_present(&entity.common.layer)` for every entity — so a layer removed from `TABLES`
but still named by a surviving entity is silently reinstated on **read** with `Layer::default()`
values (colour 7/BYLAYER, linetype "CONTINUOUS"). Verified two ways:

* the raw bytes are correct — `remove-layer-applied/after.dxf` has exactly two
  `AcDbLayerTableRecord`s ("0", "TEXT"), no "DIMS" anywhere (confirmed by directly scanning the file);
* a locked-in unit test, `reader_resynthesizes_a_removed_but_still_referenced_layer_with_defaults`
  (`🏭️generator/🦀️engine/src/main.rs`), asserts the in-memory table has 2 layers pre-save and the
  reloaded table has 3, the third being `DIMS` with colour 7 / "CONTINUOUS" — not the removed row's
  3 / "DASHED".

The parallel case for `remove-linetype-applied` (the still-referenced `DIMS` layer's
`line_type_name: "DASHED"` resurrects `DASHED` with an empty description on reload) was independently
confirmed the same way via the `project` subcommand. **This does not weaken the gate**: `expected` and
`actual` are read by the SAME loader, so a subject that genuinely fails to remove the row still
differs (its real leftover values vs. the reader's synthesized defaults) — but the projection shows a
default-valued residual row, never a clean absence, and that is recorded rather than hidden (in the
new oracle's `rationale`, and in the engine's own doc comments).

## The 37 new fixtures — one dedicated recipe per witnessable `(mutation, outcome)`

19 kinds × their real outcome classes, read from the CODE exactly as the playbook requires
(`MutationOutcome::{new,empty,error,fatal}` in `🦀️.rs`/`🔺️diff/🦀️component.rs`, not doc comments):
`no-mutation` → `no-op` only; `set-snapshot` → `applied`/`no-op`/`rejected`; `set-header-var` →
`applied` only (finding 1, above); the other 16 → `applied`/`rejected`. Total: 1 + 3 + 1 + 16×2 = 37.
Every `-applied`/`-no-op` `after` touches EXACTLY the field(s) the real dispatch touches; every
`-rejected-*` names, in its own Rust match-arm comment, the exact validation function and branch that
refuses it (`invalid-add-target`/`invalid-remove-target`/`invalid-modify-target`/`invalid-add-index`/
`invalid-remove-index`, all cited with line numbers from `🔺️diff/🦀️component.rs`), and writes ONLY
`before.dxf` — the payload that would be rejected is never itself encoded.

Reproducibility: same `FIXED_STAMP_HEADER` trick the prior session found ($TDCREATE/$TDUPDATE parsed
from a fixed literal header rather than `chrono::Local::now()`), reused unmodified. `drafting-plate`
regenerates **byte-identical** to the previously-committed fixture — `sha256:18f9470d…d80fbbdb`,
9521 bytes, confirmed both by `shasum` and a locked-in Rust test (`drafting_plate_is_unaffected_by_this_retrofit`).

## Verified with the real, unmodified framework commands

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify  --artifact s.stdio.dxf --standard r12 --subset any
[fixture verify] 38 fixture(s), 0 file problem(s)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture audit   --artifact s.stdio.dxf --standard r12 --subset any
[fixture audit] 38 fixture(s), 0 with contract problems
  (every non-drafting-plate row: generator=dxf-crate-r12-mutate-reader(dxf-rs), reproducible=true)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.dxf --standard r12 --subset any
[fixture reproduce] 38 generated fixture(s), 0 problem(s)
```

**Per-fixture** (the check the reproducibility document insists on — a whole-corpus double-run cannot
see order-dependent state): looped `fixture reproduce --mutation <m> --outcome <o>` over all 37 new
`(mutation, outcome)` coordinates individually. **Every single one**: `[fixture reproduce] 1 generated
fixture(s), 0 problem(s)`. Full transcript kept at
`🗑️temp/dxf-r12-any/fixture-reproduce-per-fixture.log`.

## Matrix — real numbers, `dxf` filtered out of the repo-wide JSON

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts matrix --json   (repo-wide; no selector flags)
```

| dimension | repo-wide | dxf rows missing |
| --- | --- | --- |
| subsetOwnershipCoverage | 658/658 (100.0%) | **none** |
| externalOracleCoverage | 431/658 (65.5%) | **none** |
| oracleEvidenceCoverage | 351/658 (53.3%) | **none** |
| oracleCapabilityCoverage | 30/54 (55.6%) | **none** |
| fixtureProvenanceCoverage | 705/705 (100.0%) | **none** |
| fixtureReproducibilityCoverage | 705/705 (100.0%) | **none** |
| runtimeMutationCoverage | 30/68 (44.1%) | `s.stdio.dxf@r12/any (no runtime inventory)` — pre-existing, out of scope (below) |

All five of the dimensions this ticket gates on are **100% clean for `s.stdio.dxf@r12/any`** — zero
entries in any dimension's `missing` array. `s.stdio.dxf` contributes 38 rows (19 mutations × their
declared outcome coordinates: 1+3+1+16×2), each resolving `oracle: dxf-crate-r12-mutate-reader`,
`oracleKind: third-party-library`, `oracleEngineFamily: dxf-rs`, `comparisonProfile:
semantic-dxf-r12-v1` — confirmed by inspecting the raw rows, not just the aggregate.

## `test contract` — filtered, zero NEW problems

`contract` scans the whole repository regardless of selector flags (1711 lines total, exit 1,
overwhelmingly pre-existing/unrelated — other subsets, e.g. `s.semio.semio@v1/mesh`, have real
unrelated fixture-profile gaps). Filtering for `s.stdio.dxf`: **exactly 20 lines**, unchanged in kind
and count from the prior session's own filtered count:

* 1 × `"No runtime inventory has been produced for s.stdio.dxf@r12/any"` — the production crate is
  out of scope (below).
* 19 × `"Mutation <kind> is owned by 'any' and s.stdio.dxf@r12 declares no narrower subset at all"` —
  the benign `WILDCARD_SUBSET_IDS` flavour, precedented 414 times repo-wide (`gif@89a` 21, `las@1.0`
  15), not touched by the earlier wave for the same reason, not touched here either.

## Gate validated BOTH ways, real numbers, quoted JSON

**ACCEPT (known-good).** `no-mutation-no-op`'s `before.dxf` vs `after.dxf` (byte-identical — both are
`base_doc()` encoded independently, matching AVI's own precedent for this case):

```json
{ "equal": true, "diffCount": 0, "diffs": [] }
```

**REJECT (known-bad, real single-field diff, both sides real `dxf`-produced bytes).**
`set-style-applied`'s `after.dxf` (expected) vs its own `before.dxf` (actual) — one field genuinely
differs (`NOTES`' font):

```json
{ "equal": false, "diffCount": 1, "diffs": ["$.styles[name=NOTES].font: \"arial.ttf\" ≠ \"romans.shx\""] }
```

**Tolerance discriminates at the declared 1e-4** (perturbation applied through `dxf`'s own typed
`Circle.radius` field via a scratch-only `perturb-radius-debug` subcommand, never a byte-level text
edit — output kept in `🗑️temp/dxf-r12-any/`, never in `🧫️fixtures/`):

```
1e-5 perturbation → { "equal": true,  "diffCount": 0 }
1e-2 perturbation → { "equal": false, "diffCount": 1, "diffs": ["$.entities[2].radius: 150 ≠ 150.01 (delta 0.009999999999990905)"] }
```

**Name-keyed vs order-significant, demonstrated on real fixtures.** `insert-layer-applied`
(before vs after) → `{"equal":false,"diffCount":1,"diffs":["$.layers[name=MARKERS]: absent in
expected, present in actual"]}` — a name ADDED shows as exactly one diff, not a positional shift
cascade. `insert-entity-applied` (before vs after) → `diffCount: 28`, every subsequent entity's
fields shifted — entities are genuinely order-significant (production's own `validate_indexed_targets`
addresses them by index), matching `semantic-dxf-r12-v1`'s own description.

All four probe commands were invoked exactly as registered in `🧪️oracle/🔣️.json`'s `probes[].command`
arrays (`bun …/🔬️probes/📜️script.ts dxf-import|dxf-project|dxf-compare`), not through a shortcut.

## What could NOT be verified, and why

`semio-s-plugin-stdio` was **not needed and not built** — both crates this work depends on are
standalone `[workspace]` roots (the engine crate depends only on `dxf 0.6`). Consequence, unchanged
from the prior session: the production-side runtime mutation inventory for this subset cannot be
produced, so `runtimeMutationCoverage` for `s.stdio.dxf@r12/any` stays unmeasured — a property of the
in-flight `protocol::Mutation`/`DESCRIPTORS` peer refactor (`d394744295`, after this ticket's
baseline), not of anything touched here. Not chased, per the repo's own rule for a peer's in-flight
work.

`cargo test --lib --features oracles` on the shared oracle crate (which houses the untouched
`component.rs`) was not re-attempted here — the prior session already established it fails for
unrelated reasons (`pdf@1.4`, `step@ap214`, `dwg@ac1024`, `mp3@mpeg1-layer3`, none `dxf`) and nothing
in `component.rs` changed.

## Cleanup

`🗑️temp/dxf-r12-any/` holds: `all-manifests.json` (the generator's own `manifests` output, input to
the patch script), `fixture-reproduce-wholecorpus.log` + `fixture-reproduce-per-fixture.log` (37
individual coordinate runs), `matrix.json` + `matrix.err.log`, `contract-full.log`, `gate-accept.json`
+ `gate-reject.json`, and the two `perturb-radius-debug` outputs used only for the tolerance
demonstration. `🏭️generator/🦀️engine/target/` is `.gitignore`d; its `Cargo.lock` is force-tracked
(`!**/🔖️*/**`), matching every sibling generator crate.
