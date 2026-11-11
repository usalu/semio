# 🦀 Norm Rust Status

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
