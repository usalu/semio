# Post-Build Plan (W3-POSTBUILD)

Scope: what must happen after the first green `:build` of each Eurocode artifact (en1990..en1999), per artifact, and whether `post-build.sh` covers it. Nothing was built, tested or edited. Runner, `post-build.sh` and all artifact sources are untouched.

Sources read: `fleet-wave3-preamble.md`, `📌️important/post-build.md`, `🗑️generated/build-runner/post-build.sh` (113 lines), `runner-next.sh` (finish_run), all 17 `🔍️audits/wave3/*.md`, the wave2 reports containing golden/regen/pack/showcase/bless/latency terms (about 300 files, keyword-extracted), and `wave2/post-build-pipeline.md`. The file `🔍️audits/post-build-pipeline.md` named in the brief does not exist; its wave2 copy is `🔍️audits/wave2/post-build-pipeline.md`.

## 1. Gaps in the pipeline (verified in code)

| # | Gap | Evidence | Effect |
|---|---|---|---|
| G1 | The post-build hook fires only when a `:build` finishes with rc 0. A finished `:test` never calls `post-build.sh`. | `runner-next.sh:230`: `if [ "$t" = build ] && [ "$rc" = 0 ]; then ... post-build.sh "$dir" auto`. | After the first green build `auto` only prints "waiting for the runner's :test". Bless, acceptance and oracle never run on their own. Red tests are never classified. Fix: also call the hook when `t=test` (any rc). This is in runner-next.sh, not in post-build.sh. |
| G2 | `bless-allow.txt` does not exist at `$TICKET/📌️important/bless-allow.txt`. | `ls 📌️important` lists no such file. | Every non-empty golden, DSL and pack change is restored from the tar and written to `golden-diff-n.txt`. Only empty files are kept automatically. |
| G3 | A pack is kept only when its sibling DSL is unchanged (rule c). A changed DSL is itself rejected unless listed. | `stage_bless`: `*.pack.semio` branch, and `ok` is set only for `empty`, `listed`, or `pack-from-unchanged-dsl`. | The stale binary packs and the DSL canonical rewrites cannot be written without a per-file allow-list. Packs are stale for every DSL touched (`showcase-refresh.md`, `coordinator.md`). |
| G4 | Bless budget is 2 passes per code. | `BLESS_COUNT >= 2` refuses. | Pass 1 rejects real diffs, a human adds allow-list lines, pass 2 writes them. A third pass needs a human decision. The budget has no slack for the semantic.json second run (see section 3, rule R-2). |
| G5 | `bless_env` for en1995 omits `EN1995_REGEN_MUTATION_VECTORS=1`. | Switch exists in `en1995/.../🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`. The script sets only `EN1995_REGEN_PACKS=1 SEMIO_UPDATE_GOLDEN=1`. | The mutation-vector regen test asserts byte equality and fails instead of blessing. Fix: add the switch to the en1995 case of `bless_env`. |
| G6 | `EN1991_REGEN_ASSETS=1` and `EN1992_REGEN_ASSETS=1` (named in several wave2 reports and in the brief) do not exist in the source. | `grep -rl` over `✏️s` returns nothing. `regen-tests.md` replaced them with `SEMIO_UPDATE_GOLDEN=1` and `-E 'test(/regen/)'`. | Do not use them. The checklist below uses the unified switch. The brief's "EN1991_REGEN_ASSETS=1 etc." is stale. |
| G7 | `.spr.semio` and `.op.semio` are never regenerated. | `post-build.md` row (c) says they are protocol fixtures for the fixture sweep. en1995-v4/v6/v11 and en1994 reports still ask for them to be re-blessed after the build. 16 such files exist across the Eurocode artifacts. | Add a step: `@semio-tech/s-fixture-sweep-rs:test` (or a regen), or name the owner. Not covered anywhere. |
| G8 | Showcase DSL assets and palette examples have no stage. | No `*.dsl.semio` with `showcase` in its path exists for en1991, en1993, en1994, en1995, en1997 (only `ground-showcase`), en1998 or en1999. Existing showcase DSLs: en1990 `road-bridge-showcase`, en1992 `rc-building-showcase`, en1996 `masonry-showcase`, en1997 `ground-showcase`. Coordinator open item: "en1998 full_showcase DSL asset plus palette example, then pack regeneration". | The regen test rewrites only existing DSL files. A new showcase asset has to be authored (Rust palette entry plus DSL file) before the regen runs. Nothing in `post-build.sh` creates it. |
| G9 | Post-build does not run schema-catalog or generated schema index regeneration. | en1990-v8 and en1990-v7 ("schema-catalog.json not regenerated"), en1993-v8 (`generate-schema-index`), en1999-v5 (`verify build-owners sync`), en1994-v6/en1995-v6 (Cargo.lock sync). | Build-time or pre-test items, not post-build. Add them to the pre-test list, or accept as manual. |
| G10 | The script only accepts `en19xx`. | `case "$CODE" in en19[0-9][0-9])`. | The din, iso and vdi artifacts with `semantic.json` and packs get no post-build. Out of scope for this ticket, but the gap is real. |
| G11 | Latency targets are unmeasured. | `SUMMARY.md:140`: "targets ... unmeasured". The latency bench exists in all ten editors. `latency-report.py` was smoke-tested only on fake JSON. | First real acceptance run is the first measurement. Release build cost is noted in `post-build.md`. |
| G12 | Unverified CLI forms. | `post-build.md` lists `repo run ... -- -E` forwarding and `-c release` for `:test` as unverified. `repo` MCP server failed to connect in this session, so the CLI could not be checked here. | First live run may fail on syntax. The single fix point is `post-build.sh`. |
| G13 | Failure classification may not match the nextest output format. | `failing()` greps `^ *(FAIL|SIGABRT|SIGSEGV|TIMEOUT) ` from `--output=static`. Format not verified against a real log. | If no FAIL lines match, the script logs "exit without FAIL lines" and never blesses. |
| G14 | Non-golden failures block every bless in the same run. | `OTHER` non-empty means no bless for that test run. | Any unrelated red test (for example a law test) prevents the golden pass. Expected after a first green build, so the human must clear those first. |
| G15 | Golden scope is restricted to `$SUBSET` fixtures and assets. | `golden_files` uses `find "$SUBSET"` with `*🧪️tests/🧫️fixtures/*` or `*🖼️assets/*`. | Covers scene goldens, semantic.json, packs and DSL in the current layout. Any future golden outside those folders is invisible to the bless rules. |
| G16 | Re-baseline of hand-pinned numbers is not automated. | Wave2 reports pin values in `catalogue-bridge-v2-en1990/1991/1992/1996`, `en1994-v10`, `en1997-v8`. The bless rule only touches golden-like files, not test source literals. | Those tests need manual re-baseline. Listed per artifact below. |

## 2. Per-artifact checklist (after the first green build)

Current state is from file inventory: scene golden `🎬️scene/🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` is 0 bytes in all ten artifacts. The showcase scene golden is also 0 bytes where it exists (en1994 `🎪️showcase`, `🌉️bridge`). "Packs" means `🎒️.pack.semio`. "semantic" means `🎛️semantic.json`.

Common steps for every artifact, in order:
1. `:build` green (runner).
2. `:test` run (runner). Classify failures. Non-golden failures stop the pass for a human.
3. Bless pass: `SEMIO_UPDATE_GOLDEN=1` plus regen switch, filter `test(/golden|regen|pack|asset|semantic/)`. Scene goldens are empty, so they are kept as "empty".
4. Review `golden-diff-n.txt` for any rejected real diff. Add to `bless-allow.txt`. Run bless again (pass 2).
5. Second `:test` run (semantic.json `include_str!`). Expect green.
6. Release acceptance: `repo run '<proj>:test' -c release -- -E 'test(latency_bench) | test(live_acceptance)'` with `SEMIO_LATENCY_REPORT_DIR`. Output `latency.md`.
7. Oracle stage: python syntax, feature dry run, Rust oracle and third-party tests.

Per artifact:

| Code | Scene goldens to bless (0 bytes today) | Packs and DSL to regenerate | Semantic contract | Showcase and palette work | Tests to re-baseline (hand-pinned) | Notes from reports |
|---|---|---|---|---|---|---|
| en1990 | `🏛️scene/🔣️.json`; new view for permanents per span (gamma bands) and sls-combinations diagram | Packs 2 (`default-office`, `high-consequence-office`); DSL 9 incl. `road-bridge-showcase` | `semantic.json` (768 bytes) regenerated by regen test | Examples registry entry `road-bridge-showcase` not added (Rust, needs build: `en1990-showcase.md`). Full showcase builder `🧰️scripts/showcase/en1990.py` exists | Add sqlite extent case with `cases` (fixture has `cases: []`, `catalogue-bridge-v2-en1990.md`). Bridge-groups numbers may change (`en1990-v3`). Rail/override handle tests (`en1990-v4`) | Open from audits: trafficGroups/factorOverrides grips, `en1990.6` checks unbound (`wave3/en1990.md`) |
| en1991 | `🏛️scene/🔣️.json` (0 bytes); deformation and decision diagrams | Packs 2 (`multi-fail-noncompliant`, `de-office-compliant`); DSL 2 in assets. Reports claim 4 palette examples, the assets count does not match | `semantic.json` (1275 bytes), reports expect 27 sqlite tables and schemaBytes 23302 (`en1991-v6`) | Showcase DSL for the full 28-collection builder (`en1991-diagrams.md`, `🧰️scripts/showcase/en1991.py`) must be authored. Hazard collections example (`en1991-v7`) | sqlite contract (table count), `oracle-cases` snow drift (mu_w 0.8 vs 0.4 open question, `en1991-v3`) | `EN1991_REGEN_ASSETS=1` in reports is stale, use `SEMIO_UPDATE_GOLDEN=1`. Wave3 has no en1991 report; the wave3 items come from wave2 `en1991-v12` (`EN1991_REGEN_ASSETS`, scene golden, Cargo.lock for 5 crates) |
| en1992 | `🏛️scene/🔣️.json` (0 bytes); office-frame golden (`en1992-v5`) and crack-width, torsion, second-order, deep-beam diagrams (wave3) | Packs 3 (`liquid-retaining-fem-anchor`, `failing-under-reinforced`, `compliant-office-frame`); DSL 6 incl. `rc-building-showcase` (42 KB) | No semantic.json (by design). sqlite `🚪️io/…/🧫️fixtures/🔣️.json` changes if scene/catalogue changes | Showcase exists (`rc-building-showcase`). `spr/op` 1 each | Oracle values T-02/A-10 (1e-6..1e-9 vs Rust), catalogue `e19..e22` (`wave3/en1992.md`). Scene tests for the new diagrams (`🧪️tests/🔭️design-views`) | `EN1992_REGEN_ASSETS` in `en1992-v14` is stale. Wave3: 5 scene modules, first compile risks |
| en1993 | `🏛️scene/🔣️.json` (0 bytes); showcase golden (new entities: `en1993-v14`); joint-ep and tower-nm diagrams (`en1993-v16/v17`) | Pack 1 (`high-strength-connection`); DSL 2 (`heb240-compliant`, `high-strength-connection`); `spr` and `op` 1 each. Showcase DSL: none | `semantic.json` (1408 bytes), sqlite contract hash (`en1993-v9`, `en1993-v7`) | Full showcase example (`showcase-docs.md`): palette entry in Rust, DSL asset missing. Confirm with asset-laws test | Generated twin tests may need re-bless (`en1993-v9`). `schema-catalog` and generated index (`en1993-v8`). Oracle case ids in `requests.md` (`en1993-vids`) | Wave3 `en1993.md` lists 6 gaps; no bless item given there |
| en1994 | Scene golden 0 bytes: `🏛️scene`, `🎪️showcase` (94 KB fixture, golden 0), `🌉️bridge`. Interaction and mn series diagrams (`en1994-v5/v6`) | Packs 2 (`composite-floor-beam`, `composite-bridge-girder`); DSL 3; `spr`/`op` for `composite-bridge-girder`. DSL regen adds action fields (`en1994-v6`) | `semantic.json` 591 bytes. Reports disagree on values (`en1994-v6`: 9814 vs 10395-byte DDL; `en1994-v7`: 10804 schemaBytes, 2189 valueBytes). Trust the regen test, do not hand-pin | Full showcase palette entry (`showcase-docs.md`), DSL asset missing | Cargo.lock sync (2 new crates). Vibration, U-frame LTB, creep deflection, protected steel: oracle rows (`wave3/en1994.md`, 16 rows) | Wave3 en1994 says "ready for rebuild". Stud work (`en1994-v10`) needs test run to re-bless |
| en1995 | Scene golden 0 bytes (`🏛️scene`): response curves, case-kser, TCC joint profile, bracing demo | Packs 4 (`glulam-floor-beam`, `glulam-footbridge`, `multi-fail-timber`, `overloaded-footbridge`); DSL 4; `spr`/`op` for `glulam-footbridge`. `.spr/.op` not regenerated (G7) | `semantic.json` present, regen switch in place | No palette change required beyond the existing entries (verify) | Mutation vectors: needs `EN1995_REGEN_MUTATION_VECTORS=1` (G5). Gamma and psi factors (`gamma-unify.md`: crowd gamma_Q 1.5 to 1.35 changes blessed outputs of footbridge assets) | `EN1995_REGEN_PACKS=1`, `EN1995_REGEN_MUTATION_VECTORS=1` and `SEMIO_UPDATE_GOLDEN=1` (bless_env sets two of three). Wave3 round 2: TCC, plywood flag |
| en1996 | Scene golden 0 bytes (`en1996-v12`: chase cut into wall face, 5 handles, Warning status); yield-line and tie entities (wave3 `en1996.md` says already done) | Packs 2 (`loadbearing-wall`, `multi-fail-masonry`); DSL 3 incl. `masonry-showcase` | `semantic.json` present (1) | `masonry-showcase` DSL exists. Flanged-wall M-20 change needs rows in showcase if fixtures move | Citation goldens if NDP reads grow (`en1996-v12` risk: `normative_limit` reads log into ndp_reads). `ndp.en1996.*` citation output | `regenerate_example_assets` requested (`en1996-v2`). Wave3 flange cap: `the_flange_overhang_is_capped_by_the_flange_length_of_every_flange` |
| en1997 | Scene golden 0 bytes. Entity counts move: one ghost per pile group (`group-G1:equivalent-raft`), up to 3 diagrams per document (consolidation, raft section, Spencer) (`wave3/en1997.md`) | Packs 3 (`demo`, `noncompliant`, `compliant`); DSL 4 incl. `ground-showcase`; regen test in `📸️snapshot` | `semantic.json` present | `ground-showcase` exists. Palette: 7 entries | Fixture `⏳️consolidation/🔣️.json` was blessed by a hand script `bless_consolidation.py` (not in post-build; re-run if the PDE changes). Entity-count tests (`en1997-v11/v12`) | Source budget for `en1997-checks` over (`wave3/en1997.md`), not a bless item |
| en1998 | Scene golden 0 bytes. Changes: `adrs:<building>` diagram, hoop entities in assessment scenes (`wave3/en1998.md`: one re-bless). Full-showcase digest written when missing (`showcase-docs.md`) | Packs 1 (`seismic-rc-frame`); DSL 4. Example pack twin in `SEMIO_UPDATE_GOLDEN` path. Showcase DSL asset and palette example missing (coordinator open item) | No `semantic.json` (by design) | Create `full_showcase` DSL asset + palette example, then regenerate packs for all new showcase assets (`en1998-showcase.md`, coordinator `[open]`) | Goldens pinned to the old showcase (periods, check counts), `en1998-showcase.md`. Golden-pinned `fundamental_period_s` signature mismatch (`en1998-v14`) | NA-DE T_R change affects capacity outcomes (`wave3/en1998.md` unverified row). Mutation fixtures ~595 stale (`showcase-refresh.md`, requests.md) |
| en1999 | Scene golden 0 bytes. New layer `mode-torsional-flexural`, third mode entity per member with tb check; `:zone`, `patch-resistance` diagrams; 3 more stiffener plates in showcase (`wave3/en1999.md`, `en1999-v9/v12`) | Packs 2 (`noncompliant-multi-fail`, `aluminium-roof-purlin`); DSL 2. Showcase DSL: none | `semantic.json` present | Full showcase palette example (per `showcase-docs.md`), DSL asset missing. Check `REGEN_EN1999_ASSETS=1` for the pack regen | Oracle section constants: 20 % tolerance on N_cr,T may fail if the FE torsion differs (`wave3/en1999.md`). `i-extruded-200x100` blessed values (`en1999-v7`) | `en1999-stiffeners`, `en1999-v4`: scene unit tests that enumerate ids may need `:` children in `owner_of` |

## 3. Rules the pipeline must respect (applies to every artifact)

- R-1 Use `SEMIO_UPDATE_GOLDEN=1` for every regen test (plus the artifact's own switch: en1995 `EN1995_REGEN_PACKS=1` and `EN1995_REGEN_MUTATION_VECTORS=1`, en1999 `REGEN_EN1999_ASSETS=1`). Do not use `EN1991_REGEN_ASSETS`, `EN1992_REGEN_ASSETS` or `EN1997_REGEN`, which do not exist.
- R-2 Run the test twice after a bless: the first run rewrites `semantic.json` and packs, the second run lets `include_str!` tests see them (`post-build.md`, `regen-tests.md`). The runner's "ready for test" line already triggers the second run.
- R-3 Packs and DSL are written only when allow-listed (G2, G3). Each DSL changed by regen needs its pack listed too, unless the DSL is unchanged.
- R-4 A stale pack fails the ordinary test run (`regen-tests.md`, intended). Expect a red `:test` until the bless pass is done.
- R-5 Scene goldens that are 0 bytes are blessed automatically (empty rule). Non-empty goldens need allow-list entries.
- R-6 Do not hand-pin numbers in tests from wave2 reports; take them from the regen output and the independent oracle.

## 4. post-build.sh coverage

| Step | Covered | Where |
|---|---|---|
| Build hook | Partly | Only on build rc 0 (G1) |
| Test run and red classification | Yes | `stage_bless` path, `failing()` (G13, G14) |
| Scene golden bless | Yes | Filter `golden` and `SEMIO_UPDATE_GOLDEN=1`; empty goldens only, or allow-listed (G2) |
| Pack and DSL regen | Partly | Filter `pack|asset|regen`; rejected unless allow-listed (G2, G3) |
| semantic.json regen | Partly | Filter `semantic`/`regen`; second run needed (R-2) |
| en1995 mutation vectors | No | G5 |
| en1995 `.spr/.op` | No | G7 |
| Showcase DSL and palette creation | No | G8 (authoring, not automatable) |
| Re-baseline of pinned test numbers | No | G16 |
| Schema catalog and index, Cargo.lock | No | G9 |
| Release acceptance and latency | Yes | `stage_acceptance` (G11, G12) |
| Oracle stage | Yes | `stage_oracle` (G12) |
| Non-Eurocode artifacts | No | G10 |

## 5. Recommended changes (not applied, runner owner decides)

1. runner-next.sh: call `post-build.sh <dir> auto` also when `t=test` completes (any rc), not only on build success (G1).
2. Create `📌️important/bless-allow.txt` with a first entry set, or a documented rule that accepts regen-test outputs (G2, G3). Decide before the first green build.
3. post-build.sh `bless_env`: add `EN1995_REGEN_MUTATION_VECTORS=1` to the en1995 case (G5).
4. Decide the owner of `.spr/.op` regeneration (fixture sweep) and add a step (G7).
5. Author the missing showcase DSL assets and palette entries for en1991, en1993, en1994, en1998 and en1999 before the first regen (G8).
6. Raise or document the bless budget (2) for the first green run (G4).
7. Verify the `repo run` CLI forms once, on the first run, then fix only `post-build.sh` (G12).

Output: this file. No files were edited except this plan. Temporary keyword extract was created and removed.
