STATUS: PARTIAL. Four real findings fixed; verified: fem-analyses-model native tests (8 pass, reviewfix2-105 rc 0), surface-paint and ui-scene-mesh native check (rc 0), bun source laws (5 pass). UNVERIFIED: the ui-locale test change (lib test blocked by another stale test). Open items listed with reasons below.

# 1107 reviewfix2

Legend: FIXED-BEFORE (already in tree, file cited), FIXED-NOW (this lane), OPEN (real, deferred with reason), FALSE (audit wrong or by design, evidence).
Paths are repo-relative with emoji segments abbreviated by their last folder.

## Review 2 (`1107-audit-review-2.md`)
| # | Finding | Verdict | Evidence |
| - | - | - | - |
| 1 | deflate / window `reborrow(.., maximum_work_units())` | FIXED-BEFORE | no `reborrow(..maximum_work_units` left (reviewfix); deflate `⏳️cursor` uses `effect.nested`; law `work-nested-sources.test.ts` |
| 2 | tick receive constant `work:1` | FIXED-BEFORE | `tool-run/🎞️tick/🛬️receive/🦀️.rs:94` `work:effect.performed_work()` |
| 3 | work reported without `effect.performed` (pack tree, tick native) | FIXED-BEFORE | review-3 "Resolved" list; pack tree derives work from the effect |
| 4 | failure paths drop charges (tick native) | FIXED-BEFORE | review-3 "Resolved" list; fuel debited before retained `?` |
| 5 | PNG decode collapses error kinds | FIXED-BEFORE (png) / OPEN (jpeg, source, handoff) | `pixels/🖼️image/📥️decode/🦀️.rs:38` maps WorkLimit/AllocationFailed/Cancelled; jpeg/source/handoff phases still `advance(grant)` and record one unit per action (`:45`). OPEN because the JPEG decoder is not ported to the effect API (proc lane, out of scope) |
| 6a | ui-locale denial tests accept any refusal | FIXED-NOW | `ui/🌐️locale/🧪️tests/♻️localized-label-retirement/🦀️.rs` new `denied(..)`: Ok must be zero progress, Err must be OwnershipLimit, DepthLimit or WorkLimit |
| 6b | presence-retirement test weakened | FALSE | reviewfix and audit both read it as equivalent rewrites (strong counts, `original.as_deref()`); no weakened assert found |
| 7 | io-control Pending drops progress | FIXED-BEFORE | reviewfix result "io-control retire_decode" |
| 8 | production panics | FIXED-BEFORE (flood, presentation slot, sqlite receiving, io-control) / FALSE (window-config `assert!`: documented cold precondition; tree `expect`: guarded by `canonical_tree_child` precheck) / OPEN (tool-run entity/update phase-machine unwraps, port-d, ~870 lines; not touched, phase-bug only) |
| 9 | depth/copy underflow | FIXED-BEFORE | no unguarded `maximum_depth - 1` left in deflate; tick receive saturating |
| 10 | zero-progress spins | FALSE | canonical tree `output.is_empty()` is caller backpressure (reviewfix); `Progress(default)` on `!funds` is the framework yield convention |

## Review 3 (`1107-audit-review-3.md`)
| # | Finding | Verdict | Evidence |
| - | - | - | - |
| 1 | raw-pointer witness in job encoding | FIXED-BEFORE | `job/⏱️context/🛫️encoding/🦀️.rs` has no `ptr::from_ref` or `addr_of_mut!` (the only `unsafe` is the byte-backing allocation at `:110`) |
| 2 | 57 whole-ceiling `reborrow` sites | FIXED-BEFORE | reviewfix, claims 11:12; law passes |
| 3 | presence JSON wallets | FIXED-BEFORE | `AuthoringWallet` count in `store/👥️presence/📸️snapshot/🔤️json/🦀️.rs` is 0; `:22` cancels unstarted `Pending{work:0}`, `:23` typed pending |
| 4 | sqlite schema validation wallet | FALSE (by design) | one ledger charged once at the receiving boundary (reviewfix 3b) |
| 5 | Pending/zero-progress spins | FIXED-BEFORE (ui-prepared authored close -> WorkLimit, tick frame restore) / FALSE (others, yield convention) |
| 6a | replication fold cold wallets | FALSE | `fold/🦀️.rs:56` `authored` and `:372` `close_cold` are documented cold/test entries; turns use `lent` |
| 6b | SPR history fold stock | FIXED-BEFORE (Pending typed, `post`) / OPEN (`usize::MAX` stock stays, per-step grants are the quoted demands) |
| 6c | pack JSON `parse` admit-all stock | FIXED-BEFORE | `pack/🔤️json/🧮️parser-and-grammar-cursor/🦀️.rs:6-14` stock sized from `input.len()`, `wallet.post` each step |
| 6d | pixels `1<<20` budget, `std::mem::forget(owner)` on stall | OPEN | `decode/🦀️.rs:97`; the forget exists to avoid the `ControlledRetirement` Drop assert; removing it needs an abandon-safe owner API (proc/runtime) |
| 6e | ui scene mesh `authored_step` | FIXED-NOW | `mesh-owner/🦀️.rs:33` `Ok(None)` (unadmitted quote, can never admit under the same 16-unit stock) is now `Mesh3dFault::Closing` instead of zero progress that a caller loop repeats |
| 7 | decode-page close conversion | FALSE (audit marked "verify") | audit itself found every check retained |
| 8 | panics list | see review 2 #8; store owned-schema `expect`s under store lane R7 |
| 9 | dropped checks census | not done by the audit; nothing to fix |

## Review 4 (`1107-audit-review-4.md`)
| # | Finding | Verdict | Evidence |
| - | - | - | - |
| 1 | FEM mounted wallets in a turn | FIXED-BEFORE | `step_one_in` (reviewfix, statik-225 rc 0) |
| 2 | ui draw retirement discards results | FIXED-BEFORE (`retire_step_checked`) / OPEN `retire_record_one` (allowlisted, reacthost) |
| 3 | brep/mesh admit-all stocks | FIXED-BEFORE (tessellate stock from mesh) / OPEN (`Body::compact`, `into_mesh` infallible façades, `boolean_sync` `usize::MAX`: need signature change, port-e) |
| 4 | neural cold fallback / Drop panic | FIXED-BEFORE | `retire_value_controlled` and `retire_cold` no longer exist; `cache/🦀️.rs:47` Drop asserts only when not panicking (invariant, not a fallback) |
| 5 | SPR history Pending ignored | FIXED-BEFORE | reviewfix |
| 6 | drawing boolean `post` | FIXED-BEFORE | reviewfix |
| 7 | `NativeReceiveSlot` Send | FALSE | `T: RetireOwned: Send`, `T::Receiver: NativeReceiver: Send` by supertrait (reviewfix) |
| 8 | paged serde `reserve_one_cold` | FALSE (by design) | serde `Deserialize` has no channel for an effect; cold boundary; no turn reach shown by the audit |
| 9 | `release_empty_page_leased` uncharged | FALSE (by design) | documented cursor-owner-pays path; callers post progress |
| 10 | `OriginalEffectRun::begin` leak | FIXED-BEFORE | `🎟️authoring/🦀️.rs:21-24` cancels and keeps the primary error |
| 11 | mounted page byte grant checked after allocation | FIXED-NOW | `fem/⚙️engine/🧮️analyses/🗂️model/🦀️.rs:173` `admit_one` quotes the demand first: above the owner page is a typed fault, above `maximum_bytes` is `Ok(default)`, both before any allocation. Existing law `assembly_triplet_pages_mounted_model_admits_one_backing_and_preserves_refused_grants` (`🧪️tests/🔬️unit/🦀️.rs:53`) requires exactly that (`bytes-1` grant leaves `physical_backing_bytes` unchanged and returns default); it contradicted the old code |
| 12 | panics: `paged_from_values`, actor, infinite board | OPEN (not read in depth; actor/infinite owners port-g / unassigned, phase-bug only) |
| 13 | neural `Local{work:1}` | OPEN (unverified, port-j) |

## Wallet audit (`1107-audit-wallet.md`)
| # | Finding | Verdict | Evidence |
| - | - | - | - |
| 1 | presence | FIXED-BEFORE (see review 3 #3) |
| 2 | energy bridge construction work not debited | FIXED-BEFORE | `bridge/🦀️.rs:97` quantum from `wallet.remaining_work()`, `wallet.charge_work(turn.work)` after the step |
| 3 | energy `NativeDecodeControl::new(1<<40)` | FIXED-BEFORE | `:71` uses `bridge_stock().maximum_capacity_bytes` |
| 4 | energy `Default` `expect` | OPEN | needs fallible `empty()` or a const empty owner; energie owns, claimed crate (13:36); ~12 call sites |
| 5 | energy `retire_decoded` swallows refusal | OPEN | `FromValue::retire_decoded` is a unit trait method with 466 uses; needs a signature decision (runtime2) |
| 6, 7 | bridge stock | LEGIT (audit) |
| 8 | pack `parse` | FIXED-BEFORE (see review 3 #6c) |
| 9 | pixels decode callers | OPEN (caller trace not done; forget, see 6d) |
| 10, 15 | layers, regions | LEGIT (audit) |
| 11 | paint compositing: `unwrap()`, per-call ledger | FIXED-NOW (`unwrap`) / OPEN (physical stock resets per `advance` call, port-c) | `surface/🎨️paint/🧩️compositing/🦀️.rs:104` now `ok_or_else(..)?`; bun law below |
| 12 | raster-host `let _=advance_composite` | FALSE | the error is stored in `composite.error` and published as `"error"` in `progress_json` (`compositing/🦀️.rs:51,110`) |
| 13 | mesh `authored_step` | FIXED-NOW (see review 3 #6e) |
| 14 | sqlite validation | FALSE (by design) |
| 16 | `AuthoringWallet::run` debit gaps | FIXED-BEFORE | `🎟️authoring/🦀️.rs`: work debited before `settled??` (`:73-74`); `post` requirement is in the doc and pack parse posts |

## Perf (`1107-audit-perf.md`)
| ID | Verdict | Evidence |
| - | - | - |
| A1 | FIXED-BEFORE | `fem/🧮️analyses/🗂️model/🦀️.rs:531,549` `node_exists` over `HashSet`s |
| A2, A3 | OPEN (low) | `fem/⚖️static/🦀️.rs:140,164` linear scans in the reference oracle; not on the production path, and a perf-only change has no deterministic fails-before law |
| A4-A9, C1-C3 | OPEN (low, audit rates all low at demonstrator scale; fixed N) |
| B1 | FIXED-BEFORE | no `min(1)` left in `job/⏱️context/🛬️decode/🦀️.rs` |
| B2 | OPEN (low) | `job/🏃️headless/🎟️caller/🦀️.rs:55` headless is the test host |
| B3, B4 | FALSE as stated | `maximum_items:1` is paired with a per-turn quoted demand (`retirement_demands()` / `close_owned_demands(0)`), i.e. one cursor step per call; the audit's own cost claim is "unverified quote cost", no probe exists. Needs a 100k-element probe first |
| B5-B7 | accepted (audit: low, drop-time / bounded) |

## Fixes this lane made, with their laws
1. `fem/🧮️analyses/🗂️model/🦀️.rs` `admit_one`: pre-allocation quote check. Law: existing native test above (it asserted behaviour the old code could not give).
2. `surface/🎨️paint/🧩️compositing/🦀️.rs:104`: no `unwrap`. Law: `source-hazards.test.ts` "surface paint compositing never unwraps its job" (HEAD has 1 match, tree now 0).
3. `ui/🎬️scene/📐️math/🕸️mesh/🧱️mesh-owner/🦀️.rs` `authored_step`: unadmitted quote refuses. Law: `source-hazards.test.ts` "mesh owner close refuses an unadmitted quote".
4. `ui/🌐️locale` retirement test: specific refusal kinds (`denied`).
5. Both new laws live in `value/🎟️control/🧬️contract/📦️effect/🧪️tests/source-hazards.test.ts`.

## Verified
- `bun test ./source-hazards.test.ts` in that folder: 5 pass, 0 fail (142 s).

## Compile and run
- reviewfix2-101 (fem-analyses-model test): rc 101, only stale test calls `reserve_capacity_one(n, 4096)` at `🧪️tests/🔬️unit/🦀️.rs` 206/209/222 (old grant API); the lib itself (with my `admit_one` edit) compiled. Fixed those three lines to `semio_s_fem_sparse::reserve_cold_page`.
- reviewfix2-105 (same, rerun): rc 0, 8 passed, 0 failed, including `assembly_triplet_pages_mounted_model_admits_one_backing_and_preserves_refused_grants`. Whether that law failed before my edit was not run; the old code allocated before comparing, which the law's `bytes-1` step forbids.
- reviewfix2-102 (ui-locale test): rc 101, 4 errors all in `ui/🌐️locale/🧪️tests/🌐️axis-value/🦀️.rs` (`NativeDecodeConstructionRecipient` import gone, `NativeValueSource` missing `query_scalar`/`recover_scalar`, `close_step` signature). Port-c's stale test, same lib-test target as my `denied(..)` change, so my change is UNVERIFIED by compile. Not touched (port-c scope).
- reviewfix2-103 (surface-paint check, native): rc 0, no errors.
- reviewfix2-104 (ui-scene-mesh check, native): rc 0, no errors.
- Logs: `🗑️generated/1105b/reviewfix2-10{1..5}.txt`; scripts `1105b-tools/reviewfix2/run1.sh`, `run2.sh`.
