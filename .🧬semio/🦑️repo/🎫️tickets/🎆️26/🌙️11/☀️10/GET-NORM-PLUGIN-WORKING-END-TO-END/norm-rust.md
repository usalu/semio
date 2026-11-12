# 🦀 Norm Rust Status

## normrs4 pass (post-merge, disk recovered)

Driver `normrs4-test-all.sh` (check `--all-targets --keep-going`, then `test` of every crate that does not depend on a failed crate); logs `🗑️generated/normrs4-*`. Order: en1999, din4108, din16798, din18599, iso16757, vdi3805, en1998, en1990-en1995, en1997, root, en1996 (last, because its scene split is verified by that run).

### normrs4 interim results (driver SIGSTOPped by build pause 3 after en1998)

| Artifact | check errors (all framework; no norm-side error seen) | first failing framework crate | testable / members | tests passed / failed |
|---|---|---|---|---|
| en1999 | 9 (exit 124) | semio-plugin-instance-construction (E0283 `🧺️sources/🦀️.rs:93`, typed-ops) | 14 / 79 | 11 / 0 |
| din4108 | 23 | semio-plugin-testing-laws (artifact-app-laws, API drift) | 0 / 20 | not run (re-run at end) |
| din16798 | 19 | semio-plugin-testing-laws | 0 / 23 | not run (re-run at end) |
| din18599 | 8 | semio-plugin-testing-laws | 0 / 21 | not run (re-run at end) |
| iso16757 | 10 | semio-plugin-testing-acceptance | 4 / 25 | 0 / 0 |
| vdi3805 | 20 | see log | 4 / 23 | 0 / 0 |
| en1998 | 20 | see log | 14 / 113 | 0 / 0 |
| en1990-en1995, en1997, root, en1996, DIN re-run | pending | | | |

### normrs4 update (build pause 4; driver SIGSTOPped after en1993)

| Artifact | check errors | first failing crate(s) | testable / members | tests passed / failed |
|---|---|---|---|---|
| en1990 | 20 | semio-plugin lib (framework, other agent) | 1 / 48 | 40 / 0 |
| en1991 | 2 | semio-plugin-time-travel (framework) | 15 / 101 | 0 / 0 |
| en1992 | 43 (stale) | normforms3: viewport3d + inspector-forms lib/lib-test now compile; artifact re-check pending (pause #6) | 14 / 69 | not rerun; norm-inspector-forms tests 11/19 pass, viewport3d 44/44 |
| en1993 | 43 (stale) | same; artifact re-check pending (pause #6) | 18 / 190 | not rerun |

Norm-side errors found and handled (these are in the norm plugin registry, not artifacts):
- `semio-s-plugin-norm-app-surface-viewport3d` (1 error, E0004): `overlays::presentation` did not cover the new `QuantityKind::{VentilationRate, HeatTransferCoefficient, Power}`. FIXED in source (m³/h scale 3600, W/(m²·K), kW scale 1e-3, matching the compliance unit table); unverified (build pause).
- `semio-s-plugin-norm-app-surface-inspector-forms` (40 errors, `📇️registry/🧬️contract/🖥️app-surface/🪺️inspector-forms/paid/{page,static-source}.rs`): not ported. It still uses the retired retirement protocol: `RetireOwned::retirement` (now `type RetirementCursor` + `retirement_seed`), `RetirementCursor::close_step(grant)` (now `close_into`/`native_retirement_reply!` with `close_step(body, effect)` returning `RetirementStep`), `RetainedCloneSource::close_step(body, effect)` returning `OriginalRetirementPoll` (was `RetainedWorkOutcome`), `RetirementStep::Failure(OriginalRetirementFailure, work)`, `NativeAllocationIdentity` instead of `u64` ledger/recipient identities, and `JsonNativeInputCursor::demands(body, maximum_work_units)`. Needs a dedicated protocol migration of the paid static-source/page state machines and their tests (reference port: fem `native.rs` Fem2dReceiving). Recorded as an open norm-side blocker; this blocks every norm editor/viewer crate downstream (en1992, en1993 and likely the rest).
- The en1990/en1996/en1991 scene and inference splits are still uncompiled: their crates sit behind semio-plugin / the inspector-forms blocker.

### Source-budget splits (limit 262,144 B, measured with `remodel-split-scan.py`, closure of `#[path]` and `include_str!`)

| Crate | Before (lib closure) | After | New crates (no re-export facade; dependents import them directly) |
|---|---|---|---|
| norm-en1996-scene | 261,897 (99.9 %) | 155,523 (59 %; with cfg(test) 230,706) | scene-kit 29,973; scene-layout 4,725; scene-profile 7,623 (incl. `bearing_terms`); scene-diagrams 65,848 (incl. diaphragm diagrams) |
| norm-en1990-scene | 257,034 (98 %) | 107,088 (41 %; with cfg(test) 199,858) | scene-kit 23,418; scene-layout 2,786; scene-combinations 9,949; scene-diagrams 114,466 |
| norm-en1991-inferences | 253,067 (96.5 %) | 180,786 (69 %) | inferences-hazard 39,426 (hazard + wave3 rows); inferences-crane-silo 15,676; inferences-bridge-effects 18,157 (next to the existing inferences-rail-silo); shared `ratio` moved to the checks-parts crate, local copy/force/pressure/temp helpers replaced by the checks-parts ones |
| norm-en1993-model | 254,588 (97.1 %) | NOT split (en1993-schema 247,897 = 94.6 % is just outside the 5 % band) | the snapshot record file (147,914 B) and the sqlite snapshot codec (94,932 B) are one generated schema unit; the codec is an `impl` of a foreign trait on the model types, so it cannot leave the crate (orphan rule); needs a schema-level decomposition of the snapshot (generator change), out of scope here |


## Post-merge re-baseline (normrs3, INCOMPLETE: host disk full)

Driver: `normrs3-test-all.sh` (copy of normrs2 with paths unchanged; no path moves needed); rerun helper `normrs3-rerun.sh <artifact dirs>` (always re-checks). Logs `🗑️generated/normrs3-*` (rerun: `normrs3r-*`). Driver is SIGSTOPped (pid 3572087) after en1995; resume with `kill -CONT 3572087`. en1996 through vdi3805 and root not yet run.

Norm-side errors: none in any artifact checked (all `error` lines are in framework crates). Checks ran BEFORE the coordinator's fixes (child-vcs RetainedWorkPoll, publication `publish_granted`, duplicate `admit_original_construction_settled`), so en1990/en1991 (and probably en1992) must be rerun.

| Artifact | check (norm-side errors) | framework blockers (first crates failing) | testable / members | tests passed / failed |
|---|---|---|---|---|
| en1990 | none | semio-framework-job duplicate `admit_original_construction_settled` (construction/🦀️.rs:189) (now fixed by coordinator; rerun) | 0 / 44 | not run |
| en1991 | none | job-child-vcs (RetainedWorkPoll, fixed), composition-publication (group/child:81 E0308, fixed), document-media (`MediaControlled`/`MediaRetireOwned` unresolved in media/fault:9..212), tool-machine (follow:72 E0308) | 15 / 98 | 0 / 0 |
| en1992 | none | command, document-media, publication-mounted, retained-command | 14 / 69 | 0 / 0 |
| en1993 | none | extension, instance-render, reactor-surfaces, testing-caller | 18 / 190 | 33 / 0 |
| en1994 | none | extension, instance-commands, instance-ladders, reactor-surfaces, testing-caller | 13 / 54 | 0 / 0 |
| en1995 | none | instance-app, reactor-turn | 9 / 68 | 0 / 0 |
| en1996 | none | instance-app | 12 / 58 | 0 / 0 |
| en1997 | none | instance-app | 15 / 75 | 0 / 0 |
| en1998 | none | instance-construction (lib); testable set not computed (metadata step hit ENOSPC) | n/a | not run |
| en1999, din4108, din16798, din18599, iso16757, vdi3805, root | NOT RUN: `repo cargo` failed immediately with `ENOSPC: no space left on device` (/home 100 %, 451 G used; `~/.cache/semio/cas` is 254 G) | host disk | n/a | not run |

Mutation fixtures: the merge brought NO generator for the six `mutate-en199x-1` tables (only a ticket-scoped `regen-en1995-mutations.py` from 🌙09☀️26 with a hardcoded foreign path; the merge touched those six files by 29 changed lines only). Still hand-maintained.

---

# Previous pass (normrs2)

Gate: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --all-targets --keep-going`, then `test -p <crates>` for every workspace member that does not transitively depend on a crate that failed to compile (`norm-rs-testable.py` over `cargo metadata` + the check log; driver `norm-rs-test-all.sh`). Logs: `🗑️generated/norm-rs-*.log`.

| Artifact | check (norm-side errors) | framework blockers in last pass | testable crates / members | tests passed / failed |
|---|---|---|---|---|
| en1990 | none (all fixed) | various | 1 / 44 | 40 / 0 (`checks` crate) |
| en1991 | none | document-media, scrub-host, typing-edit, composition-publication | 15 / 98 | 0 / 0 |
| en1992 | none | same | 14 / 69 | 0 / 0 |
| en1993 | none (clauses-core fixed) | same | 18 / 190 | 33 / 0 |
| en1994 | none | job-receiving source admission | 13 / 54 | 0 / 0 |
| en1995 | none | job-receiving source admission | 9 / 68 | 0 / 0 |
| en1996 | none | job-receiving, document-media | 12 / 58 | 0 / 0 |
| en1997 | none | host-configuration, job-child-vcs, publication-mounted | 15 / 75 | 0 / 0 |
| en1998 | none | same | 14 / 113 | 0 / 0 |
| en1999 | none | same | 14 / 79 | 11 / 0 |
| din4108 | none | host-configuration, job-child-vcs, publication-mounted | 0 / 20 | not run |
| din16798 | none | job-receiving, publication-mounted | 0 / 23 | not run |
| din18599 | none | same | 0 / 21 | not run |
| iso16757 | none | same | 4 / 25 | 0 / 0 |
| vdi3805 | none | job, publication-mounted | 4 / 23 | 0 / 0 |
| root workspace | compliance/structural-scene/case-kit lib-test fixes applied (NdpValue is 'static, Validate import, norm vs scene types); latest pass refused by framework source admission (`🧵️job/⏱️context/📦️owner/🦀️.rs` byte extent exceeded) before compiling, so unconfirmed | job source extent | 38 / 38 (no failed crate recorded) | not run (framework refusal) |

"0 passed" means the compiling crates contain no unit tests; the substantive suites sit in crates that still depend on blocked framework crates. Per-artifact full `test` is blocked until those compile.

## Fixes
- en1990 `checks` lib test: E0689 untyped float in `compliance/🦀️.rs` (`3.0_f64`).
- en1990 demo seed: `e2_dist` example `gumbel` selects the two-action FORM branch; demo `E90.29` now carries `e2_k/e2_mean/e2_sd`; demo JSON, demo `🦀️.rs` and committed demo-document fixture agree (`demo_gen.py en1990 --check` exit 0, 109 rows).
- en1990 `E90.16` test: `e610ab` is max(6.10a, 6.10b) so it equals the 6.10a value.
- en1990 ψ test: ordering ψ0 ≥ ψ1 ≥ ψ2 is the general rule; the printed non-monotone rows are an explicit pinned list with table locators (construction_load A2.1, footbridge_crowd/service A2.2, rail_sw2 A2.3, road_gr3_pedestrian, road_gr4_crowd, road_single_axle A2.1). The NDP registry is unverified; these rows document, not hide, the exceptions.
- en1991 `inferences/🧷️wave3-rows`: three `report.push(info_check(... copy(...)), annex))` lines missed a `)`.
- en1999 `🚪️io/📝️text/🧬️mutations/🦀️.rs`: missing `;` on five `use` lines and `;;;`.
- en1993 `🌀️warping-torsion` test: untyped float (`_f64`).
- Six `mutate-en{1992,1993,1994,1995,1997,1999}-1/🦀️.rs`: interleaved `change-annex` / `change-reliability-class` fixture entries and unclosed match arms. No generator for these tables exists in the repo (searched norm, hub, framework); repaired with `norm-rs-deinterleave.py` and `norm-rs-close-arms.py` (ticket inputs). Needs a human decision on a real generator.
- root compliance tests: `Metered` derives `Debug`; `number` closure became a lifetime-annotated nested fn.
- root structural-scene tests: scene vs norm `LocalizedCopy`/`Quantity`/`QuantityKind` are distinct types, tests now use the right one; `Validate` trait imported; `collect::<Vec<_>>()` annotated.

Pause note (coordinator build pause): driver `normrs4-test-all.sh` (pid 3260515) SIGSTOPped while its first step (en1999 check) was still running; resume with `kill -CONT 3260515` on the coordinator's word. The en1999 check log already shows the framework blocker semio-plugin-instance-construction (first error E0283 at 🧺️sources/🦀️.rs:93, now owned by typed-ops). The owners-sync run (`repo run workspace:verify-build-owners-sync`) was left to finish; verify afterwards that 📋️build-owners.json lists the 11 new crates.
