# 📓️ `s.stdio.dxf@r12/✳️any` — third-party-generated fixture corpus, mutation manifest, gate validation

Scope: close the one declared gap for `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any`
— it had a qualifying oracle (`dxf-crate-r12-mutate`, `dxf` 0.6) and a 19-kind `mutationCatalogs`
block, but **zero fixtures, no `mutationManifests`, no `fixtureManifests`, no `🏭️generator/`**. The
oracle choice was already settled and was NOT re-litigated or changed.

Structural template: `📓️gif-las-pdf17-findings.md` (same shared oracle crate, same
standalone-generator shape, same one-base-fixture-per-subset conclusion). Everything below was run,
not asserted.

## Delivered

```
✳️any/🏭️generator/📜️script.ts                    generate | manifests, honours SEMIO_FIXTURE_OUT as a ROOT
✳️any/🏭️generator/🦀️engine/{Cargo.toml,src/main.rs}   standalone [workspace], depends on `dxf 0.6` and nothing else
✳️any/🧫️fixtures/drafting-plate/drafting-plate.dxf    9 521 bytes, sha256:18f9470d…d80fbbdb
✳️any/🧪️oracle/🔣️.json                            + mutationManifests (19) + fixtureManifests (1)
```

Scratch verification crate (ticket folder, own `[workspace]`, never joins the repo):
`🔬️dxf-r12-any-oracle-verify/` — links ONLY `✳️any/🧪️oracle/🦀️component.rs` by `#[path]`.

## Step 0 — the carrier is real

The subset's single export serializer
(`🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs:13`) is
`serialize(from) -> TxtSnapshot::from_body(&print_dxf_document(from))`, and
`print_dxf_document` (`🧬️schema/📸️snapshot/🦀️component.rs:1045`) emits the genuine group-code
stream — `HEADER`, `TABLES`, `BLOCKS`, `SECTION/2 ENTITIES`, `ENDSEC`, `EOF` — with
`parse_dxf_document` as its reader. DXF R12 ASCII **is** a text format, so "serialize to `stdio.txt`"
is the real carrier here, not the playbook's `print_dsl` stub shape: the DSL body IS the DXF
document. Carrier recorded as `["dxf"]` on all 19 manifest rows.

## Step 1/2 — outcome classes read off the code, not the doc comments

`grep MutationOutcome:: ✳️any/🧬️schema/🧬️mutations/🦀️.rs` returns exactly two call sites:

* `:235` `protocol::MutationOutcome::new(match self { … })` — the dispatch wraps **every** kind
  uniformly; there is no per-kind `empty`/`error`/`fatal` branch.
* `:225` `protocol::MutationOutcome::error(error.code, …)` inside `apply_dxf_mutation`, reachable by
  every kind when `MutationDiff::apply` fails → generic `rejected`.

One additional site, in the only per-mutation leaf that exists:
`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs:11`
`MutationOutcome::new(DxfDiff::default()).warn("mutation.no-op", …)` when the replacement snapshot
equals the base → `no-op`.

Recorded, therefore: `no-mutation` → `["no-op"]`; `set-snapshot` → `["applied","no-op","rejected"]`;
the other 17 → `["applied","rejected"]`. Identical shape to gif@89a and las@1.0.

Note the file layout: the mutation vocabulary lives in `🧬️schema/🧬️mutations/🦀️.rs`, **not**
`🦀️component.rs` (the rename in commit `d394744295`). `payloadSchema` points at the file that
actually exists, `../🧬️schema/🧬️mutations/🦀️.rs#DxfMutation::<Variant>`; las/gif still carry the
stale `🦀️component.rs` spelling in theirs, which is a separate, cosmetic inaccuracy in their
manifests (not touched here).

## Requirement 5 — witnessability, measured per kind

All 19 kinds were run forward AND inverse against the new fixture through the real
`dxf::Drawing::load`/`save`, and the resulting `project_dxf_r12` surfaces compared under a local
implementation of the `semantic-dxf-r12-v1` profile (tolerance 1e-4, `handle`/`ownerHandle`/
`fileSize`/`byteLength` ignored). Real output:

```
[witness] no-mutation        moved=0   first=[]
[witness] set-snapshot       moved=9   first=["$.insertionBase[0]", "$.insertionBase[1]", "$.layers", "$.entities"]
[witness] set-header-var     moved=2   first=["$.insertionBase[0]", "$.insertionBase[1]"]
[witness] remove-header-var  moved=2   first=["$.insertionBase[0]", "$.insertionBase[1]"]
[witness] insert-layer       moved=7   first=["$.layers", "$.layers[1].name", "$.layers[1].color", "$.layers[1].linetype"]
[witness] remove-layer       moved=5   first=["$.layers[1].name", "$.layers[1].color", "$.layers[1].linetype", "$.layers[2].name"]
[witness] set-layer          moved=2   first=["$.layers[1].color", "$.layers[1].linetype"]
[witness] insert-style       moved=5   first=["$.styles", "$.styles[1].name", "$.styles[1].font", "$.styles[2].name"]
[witness] remove-style       moved=3   first=["$.styles", "$.styles[1].name", "$.styles[1].font"]
[witness] set-style          moved=1   first=["$.styles[1].font"]
[witness] insert-linetype    moved=9   first=["$.linetypes", "$.linetypes[1].name", …]
[witness] remove-linetype    moved=4   first=["$.linetypes[3].name", "$.linetypes[3].description", …]
[witness] set-linetype       moved=1   first=["$.linetypes[3].description"]
[witness] insert-entity      moved=28  first=["$.entities", "$.entities[2].center[0]", …]
[witness] remove-entity      moved=20  first=["$.entities", "$.entities[3].center", …]
[witness] set-entity         moved=4   first=["$.entities[5].position[0]", "$.entities[5].position[1]", "$.entities[5].height", "$.entities[5].value"]
[witness] insert-block       moved=5   first=["$.blocks", "$.blocks[1].name", "$.blocks[1].basePoint[0]", …]
[witness] remove-block       moved=1   first=["$.blocks"]
[witness] set-block          moved=8   first=["$.blocks[0].basePoint[0]", "$.blocks[0].basePoint[1]", "$.blocks[0].entities", …]
```

Every non-identity kind moves the projection and **every inverse restores it exactly** (0 residual
differences, asserted per kind). So: **19 of 19 witnessable, 0 `-uncarried` exemptions.** All 19
carry the real `oracleRequirement` `{capability: dxf-r12-mutate, qualifyingKind: third-party-library,
oracle: dxf-crate-r12-mutate}`. There is nothing here analogous to pdf@1.7's `insert-object`.

**One honest narrowing, recorded in the manifest rather than glossed.** `set-header-var` /
`remove-header-var` are carried **only for `$INSBASE`**: `dxf::Header` is a fixed generated struct
with no arbitrary `$VAR` slot, and its own generated writer emits `$INSUNITS` only for
`version >= R2000`, so no other header variable survives an R12 save/reload through this reference
library at all. Both rows therefore carry the extra local invariant
`only-insbase-is-representable-in-r12` alongside the substantive one. This is a genuine narrowing of
the mutation's payload space under this oracle, not a missing capability — the kind itself IS
applied, re-serialized and read back by `dxf`, and its projection moves (2 differences, above).

## Requirement 4 — the gate validated in BOTH directions, with numbers

`semantic-dxf-r12-v1` reimplemented against its own registration (tolerance `0.0001`, `ignoreKeys`
`handle`/`ownerHandle`/`fileSize`/`byteLength`) in `🔬️dxf-r12-any-oracle-verify`, then run three ways.

**ACCEPT (known-good).** The oracle's own `no-mutation` re-encode — a real `dxf` load/save round
trip, which is byte-different from the input, asserted so the test is not vacuous — against the
committed fixture:

```
[gate accept] bytes 9521 vs 9521 (differ), projection differences = 0
```

**REJECT (known-bad, and NOT a hand-invented document).** The *same* `set-entity` payload applied to
the *wrong* index — 5 (the `TEXT`) is correct, 3 (the `ARC`) is wrong. Both documents are produced by
the reference library itself. 13 differences:

```
$.entities[3].center      present vs absent      $.entities[3].radius     present vs absent
$.entities[3].startAngle  present vs absent      $.entities[3].endAngle   present vs absent
$.entities[3].entityKind  "arc" vs "text"        $.entities[3].layer      "DIMS" vs "TEXT"
$.entities[3].position    absent vs present      $.entities[3].height     absent vs present
$.entities[3].value       absent vs present
$.entities[5].position[0] 200 vs 80    (delta 120)
$.entities[5].position[1] 260 vs 720   (delta 460)
$.entities[5].height      80 vs 35     (delta 45)
$.entities[5].value       "PLATE REVISION B" vs "DRAFTING PLATE"
```

**The 1e-4 tolerance is a real threshold, not decoration.** Same `set-entity` payload, `radius`
perturbed below and above it, both written by `dxf`:

```
[tolerance] 1e-5 perturbation → 0 difference(s);
            1e-2 perturbation → 1 difference(s) [("$.entities[2].radius", 0.009999999999990905)]
```

150.00001 vs 150.0 (delta 1.0e-5) is accepted; 150.01 vs 150.0 (measured delta 9.999999999990905e-3)
is rejected, at exactly one coordinate. The gate discriminates.

## Reproducibility — one real trap found, in `dxf` itself

The brief asked whether `dxf::Drawing::save` embeds a process-global counter, timestamp or similar,
and required proof either way. Both directions were checked.

**Handles are NOT process-global.** `Drawing::next_handle` (`drawing.rs:654`) reads and bumps
`self.header.next_available_handle` — per-`Drawing` state, derived entirely from what was added to
that drawing. Nothing `static`/`Atomic`/`thread_local` exists in the writer path (`grep` over
`dxf-0.6.1/src/*.rs`: the only `'static` hits are encoding references). So no OCCT-style
`NEXT_ASSEMBLY_USAGE_OCCURRENCE` analogue. Handles are also in the profile's `ignoreKeys` and absent
from the projection.

**But there IS a wall-clock stamp, and it was caught empirically, not by reading.** Five generator
runs in one shell loop were byte-identical; a sixth run ~100 s later was not. Diff of the two
outputs, the whole difference:

```
336c336
< 2461281.56462962972       ($TDCREATE)
---
> 2461281.565798610914
340c340
< 2461281.56462962972       ($TDUPDATE)
---
> 2461281.565798610914
```

`Header::default()` sets `creation_date`/`update_date` to `chrono::Local::now()` (generated
`header.rs:709`/`:711`) and `Drawing::save` writes both fields verbatim
(`header.rs:2405`/`:2415`). Second granularity — which is exactly why five runs inside one second
agreed and looked "reproducible". This is the reproducibility document's lesson in a different
costume: a batch that finishes fast enough cannot see a clock.

**Fix, without taking a second dependency.** Both fields are `chrono::DateTime<Local>` and every
`f64`↔`DateTime` helper in `dxf` is `pub(crate)`, so the generator obtains a fixed stamp by having
**`dxf`'s own reader** parse a literal three-variable R12 header (`$ACADVER`/`$TDCREATE`/`$TDUPDATE`
= `2461281.0`) and copying the two parsed values across. `as_datetime`/`as_double` share the same
local 1899-12-30 epoch, so the conversion cancels and the emitted double is time-zone independent —
verified, not assumed:

```
$ generate a.dxf ; sleep 2 ; generate b.dxf ; TZ=UTC generate c-utc.dxf ; TZ=Pacific/Auckland generate d-nz.dxf
18f9470d4a545fcca0ad6fb7ce9d0721c098f1012a45dc4eed4e28cad80fbbdb  a.dxf
18f9470d4a545fcca0ad6fb7ce9d0721c098f1012a45dc4eed4e28cad80fbbdb  b.dxf
18f9470d4a545fcca0ad6fb7ce9d0721c098f1012a45dc4eed4e28cad80fbbdb  c-utc.dxf
18f9470d4a545fcca0ad6fb7ce9d0721c098f1012a45dc4eed4e28cad80fbbdb  d-nz.dxf
```

Output paths of different lengths were used deliberately; the bytes do not depend on them. The
fixture file is hashed once and never rewritten afterwards (playbook step 5.4).

`$TDINDWG`/`$TDUSRTIMER` are `Duration::default()` = 0, and `$TDUCREATE`/`$TDUUPDATE` are R2000+ and
never emitted for R12 — confirmed by the diff above showing exactly two differing lines and nothing
else.

## The fixture — `drafting-plate`, one base document for all 19 kinds

Per `📓️gif-las-pdf17-findings.md`, one well-designed base fixture per subset is what the four gated
coverage dimensions actually measure (re-read `measureCoverage`, `📦️index.ts:5472-5480`: a mutation
is evidenced iff a qualifying oracle discharges it AND *some* fixture targets
`artifact@standard/subset` — any fixture counts for every mutation in the subset). Nineteen bundles
would not raise those numbers. Its own projection, printed by the oracle rather than described:

```json
{"acadVersion":"R12","insertionBase":[12.5,-7.25,0],
 "layers":[{"name":"0","color":7,"linetype":"CONTINUOUS"},{"name":"DIMS","color":3,"linetype":"DASHED"},{"name":"TEXT","color":5,"linetype":"CONTINUOUS"}],
 "styles":[{"name":"STANDARD","font":"txt"},{"name":"NOTES","font":"romans.shx"},{"name":"TITLES","font":"italicc.shx"}],
 "linetypes":[{"name":"BYLAYER",…},{"name":"BYBLOCK",…},{"name":"CONTINUOUS","description":"Solid line"},{"name":"DASHED","description":"Dashed __ __ __ __"},{"name":"HIDDEN","description":"Hidden - - - - - -"}],
 "blocks":[{"name":"SHELTER_POST","basePoint":[0,0,0],"entities":[line,circle]},{"name":"BENCH","basePoint":[15,-5,0],"entities":[line]}],
 "entities":[line,line,circle,arc,solid,text,insert]}
```

Design decisions, each for a stated reason:

* `$INSBASE` deliberately non-origin `(12.5, -7.25, 0)` — `remove-header-var` resets it to the
  origin, so an origin base point would make that kind an unwitnessable no-op.
* Five `LTYPE` rows including `BYLAYER`/`BYBLOCK` stated explicitly rather than inherited: entities
  default to `line_type_name = "BYLAYER"` and `Drawing::add_entity` silently `ensure_*`-appends any
  missing table row, so leaving them implicit would have made table ORDER — which every
  index-addressed insert/remove is measured against — an accident of `normalize`'s alphabetical sort.
  Every one of the five tables is cleared and re-added in declared order for the same reason.
* Seven order-significant top-level entities covering all six typed kinds the subset models, so
  `set-entity`/`remove-entity`/`insert-entity` have distinct middle and end targets.
* `units.length = "unitless"`: DXF R12 declares no drawing unit at all (`$INSUNITS` is R2000+ and
  `dxf` never emits it for R12 — the oracle module's own `🔖️HeaderVar` note, independently confirmed
  in the generated writer). `units.angle = "degree"`: DXF group codes 50/51 are degrees.
* `family: "mechanical"`, matching the las/gif/pdf base fixtures.
* `mutation`/`outcome` deliberately left unset: this is a base INPUT document, not the expected
  result of one mutation × outcome. Claiming otherwise would be a false declaration; the las, gif and
  pdf base fixtures do the same. Consequence, stated rather than hidden: the 38 `mutation × outcome`
  matrix rows for this subset stay `status: "missing" — no fixture declares this mutation × outcome`,
  which is `expectedOutcomeCoverage`, not one of the four gated dimensions in scope.

## Verified with the real, unmodified framework commands

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify    --artifact s.stdio.dxf --standard r12 --subset any
[fixture verify] 1 fixture(s), 0 file problem(s)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture audit     --artifact s.stdio.dxf --standard r12 --subset any
[fixture audit] third-party-generated    s.stdio.dxf@r12/any / licence=public-domain (synthetic, …) reproducible=true generator=dxf-crate-r12-mutate(dxf-rs)
[fixture audit] 1 fixture(s), 0 with contract problems

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.dxf --standard r12 --subset any
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

`fixture reproduce` is the PER-FIXTURE check the reproducibility document insists on; it re-ran the
recorded `generator.command` from the repo root with `SEMIO_FIXTURE_OUT` pointed at a fresh cache
directory and re-hashed the bytes. It passed first time because the generator honours that variable
as a fixtures ROOT (`<root>/<recipe>/<file>`) — the mistake the gif/las/pdf wave had to fix in all
three of theirs.

Isolated oracle verification, real output, offline:

```
$ cd .🧬semio/…/🔬️dxf-r12-any-oracle-verify && cargo test --offline -- --nocapture
running 7 tests
test tests::projects_the_declared_structure ... ok
test tests::gate_accepts_a_known_good_pair ... ok
test tests::gate_rejects_the_same_mutation_on_the_wrong_target ... ok
test tests::gate_tolerance_discriminates_at_1e_minus_4 ... ok
test tests::all_19_kinds_mutate_and_invert_on_this_fixture ... ok
test dxf_r12_any::smoke_tests::identity_round_trip_is_not_byte_identical ... ok
test dxf_r12_any::smoke_tests::all_kinds_mutate_and_invert_cleanly ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
```

The last two are the oracle module's OWN committed `smoke_tests`, which had never been executed
(`cargo test --lib` on the shared crate still cannot compile — see below); they run here for the
first time and pass against the pre-existing `bus-shelter` example.

## Coverage — read out of the real matrix, not asserted

`bun 🧰️framework/…/🧪️test/📜️script.ts matrix --json` (repo-wide; the command does not honour
selector flags), `missing` arrays filtered for `dxf`:

```
subsetOwnershipCoverage    645/658 repo-wide — dxf missing: []
externalOracleCoverage     436/658 repo-wide — dxf missing: []
oracleEvidenceCoverage     262/658 repo-wide — dxf missing: []
oracleCapabilityCoverage    35/48  repo-wide — dxf missing: []
fixtureProvenanceCoverage  419/444 repo-wide — dxf missing: []
```

The manifest is genuinely in the denominator, not silently undiscovered: the matrix emits **38 rows**
for `s.stdio.dxf` = 19 mutations × their declared outcome coordinates (1 + 3 + 17×2), each already
resolving `oracle: dxf-crate-r12-mutate`, `oracleKind: third-party-library`,
`oracleEngineFamily: dxf-rs`. All 19 dxf@r12 mutations are at 100 % on all four gated dimensions in
scope.

## `test contract` — filtered, and what remains

`contract` scans the whole repository regardless of `--artifact/--standard/--subset` (1 604 lines,
exit 1, overwhelmingly pre-existing and unrelated). Filtering for this subset leaves exactly two
classes, both expected:

* `"No runtime inventory has been produced for s.stdio.dxf@r12/any"` — a runtime inventory is emitted
  by the production crate, which is out of scope here (below).
* 19 × `"Mutation <kind> is owned by 'any' and s.stdio.dxf@r12 declares no narrower subset at all"` —
  the **benign** flavour of `WILDCARD_SUBSET_IDS`, not pdf@1.7's real conflict. `🪆️subsets/` for
  dxf@r12 contains exactly one entry, `✳️any`, and its own `🔣️component.json` declares
  `"subsets": {"*": {…}}` — there is no sibling subset for the wildcard to collide with, and
  `isWildcardSubsetFor` (`📦️index.ts:2708`) correctly returns false for it, which is why
  `subsetOwnershipCoverage` lists nothing for dxf and `fixture audit` reports 0 contract problems.
  This line appears **414 times repo-wide** across 15 artifacts, gif@89a (21) and las@1.0 (15)
  included — both left as-is by the earlier wave for the same reason. No rename is warranted here.

## What could NOT be verified, and why

`semio-s-plugin-stdio`, the production crate, was **not needed at all** and was not built. Both crates
this work depends on are standalone `[workspace]` roots that do not reference it:

* the shared oracle crate — `cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && cargo check
  --features oracles --offline` → `Finished dev profile in 0.22s`, 3 warnings, 0 errors;
* the new `🏭️generator/🦀️engine`, whose only dependency is `dxf 0.6`.

The one consequence: the **production-side runtime mutation inventory** for this subset cannot be
produced, so `runtimeMutationCoverage` / `productionBridgeCoverage` for `s.stdio.dxf@r12/any` remain
unmeasured and `contract` keeps reporting "No runtime inventory has been produced". That is a
property of the in-flight `protocol::Mutation`/`DESCRIPTORS` peer refactor already documented in
`📓️session-close.md`, not of anything added here. The manifest's `productionDispatch.variant` values
were name-checked directly against the `enum DxfMutation` definition (`🧬️mutations/🦀️.rs:78-181`)
rather than against a compiled inventory.

Separately, `cargo test --lib --features oracles` on the shared oracle crate still fails to compile,
for **unrelated, out-of-scope** reasons — the failing modules, listed from the real compiler output,
are `pdf@1.4` (`base` ×10, `a`, `x`), `step@ap214` (`cc1`–`cc6`, 2 each), `dwg@ac1024` (4) and
`mp3@mpeg1-layer3` (2); none is `dxf`, and pdf@1.4's is the genuinely-restructured fixture breakage
the gif/las/pdf wave already flagged as `spawn_task task_c6c27918`. Nothing in this ticket's scope
was touched to work around it; the isolated
`🔬️dxf-r12-any-oracle-verify` crate sidesteps it exactly as `🔬️gif-las-pdf17-oracle-verify` did.

## Cleanup

`🗑️temp/dxf-r12-any/` (contract/matrix dumps and the determinism probe outputs) deleted after use.
`🔬️dxf-r12-any-oracle-verify/` source kept, its `target/` and `Cargo.lock` removed.
`🏭️generator/🦀️engine/target/` is `.gitignore`d (`.gitignore:337`); its `Cargo.lock` is force-tracked
by `!**/🔖️*/**` (`.gitignore:188`), matching the las generator.
