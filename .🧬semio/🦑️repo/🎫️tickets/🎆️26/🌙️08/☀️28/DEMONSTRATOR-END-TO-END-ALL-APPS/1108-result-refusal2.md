# 1108 refusal2: ⚠️refusal port onto the reply-shaped failure model

File: `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/⚠️refusal/🦀️.rs` (owned crate).

## State (W-refusal2-2)
- Hand-written `NativeRetirementFailureReceiver` is fully ported; zero errors remain at its lines (76-165).
- `OriginalFailureStorage` and `refill_original_failure_storage` were not removed in 733; they just left the owned crate root. The file now imports them from `semio_framework_value_retirement` (retirement-contract re-export).
- advance_nested: `fail` and the phase 2/4/5 closure now return `NativeReceivingFailure` (was `OriginalReceivingPrimary`).
- Close side: old `close_step(effect)->Poll`, `close_original_step(original,effect)` replaced by one `close_dispatch(original: Option<loan>, body, effect, reply, recover)` serving `close_step`, `recover_close_step`, `close_original_step`, `recover_close_original_step` (same shape as 👥️tuple). Child slot begins in the parent close path (`begin_close_into` before `close_into`), `advance_pending` delegates to the slot, close demand/work quotes are begin-aware. No adapter, no poll-to-reply shim.
- Unchanged semantics: original birth (`close_original_birth_step`, `refill_original_failure_storage`) and original-control gates.

## Remaining errors attributed to this file (not hand code)
Only at derive sites (lines 19, 38, 55, 60, 73, 182, 187: `#[derive(NativeReceive)]`): expansion still emits the pre-733 `NativeRecordMatcher` API (`demands/advance/advance_work_demand`, 5-arg advance), `RetainedCloneRef::try_project_owned` (value_codec clone side) and `NativeVariantSelector::close_step` (line 55/182). Owners: derive receive (receivers2) and 🔀️variant selector (nested).

## Rerun
`"$C/1105b-tools/chk.sh" W-refusal2-<n> 🧰️framework/Cargo.toml native -p semio-framework-value-native-owned` then filter `🌳️value/⚠️refusal/🦀️.rs` with line >= 75.


## Coordinator check (W-macros-24)
Hand-written receiver has no ValueError vs OriginalReceivingPrimary mismatch; the only remaining errors in the module are at the derive sites (lines 55 and 182: E0282/E0283 from derive expansion), owned by derive/matcher.

## Task 2: value-persistent `--tests` port (W-refusal2-3 .. W-refusal2-34)
- `--tests` compiles (W-refusal2-12, rc 0; 143 errors -> 0).
- Tests (W-refusal2-44): lib 24/24, `original-ranked-updates-and-reads` 2/2, `persistent-builder-resident` 3/3, `original-vector-concat` 5/5, rc 0.
- New shared harness `📋️list/🔗️persistent/🧪️tests/reply.rs`: `poll` (drives each reply-shaped turn through a genuine `OriginalEffectAuthority` invocation, no allocation, work measured inside the callback), `run`, `take`, `begin`, `read`, `admit_source`, `banked`. Included as `tests::reply` (lib) and `wallet` (owning.rs).
- Ported: cursor `advance/take/finish/adopt/begin_close/close_step` to `*_into`/`begin_close_paid` in splice, build (3 files), read (2), update, observation, traversal, root tests, sealed-shared-list; source admission to `EffectBank` (`admit_source`/`wallet::admit`), `source.close_into`, `ControlledRetirement::step_into(body,..)`, clone test via `PersistentList::project` (Clone no longer exists), resident.rs borrow fix.
- Laws changed by 733 (updated to the new exact value, with reason): projection quote = duplicate copy + 2 `Option<PersistentList>` headers; adopt/finish/take copy exactly their quote; builder `demand()` made exact per phase (Scan branches, carry, leaf/branch staging) in `📥️build/🦀️.rs` because quote exceeded the actual copy; `admit_funded` funds `source_admission_demand`; build preparation cuts 0..=368 (finish now first reachable at turn 345; ready at 368); wallet `largeCounts` += 4097 (+schema) because fixture admission and sealed births cost more fuel; close work bounded by the selected `close_work_demand` instead of 1.
- Multi-turn driver: `reply::Driver<E>` keeps one `OriginalEffectAuthority` plus its pending `OriginalEffectAction` across turns of a shared `EffectBank`, with the bank's Source identity as producer source (the cursor's projection step rejected the sourceless per-turn harness: "projection changed original action"). The callback gets `recover=true` on a resumed action. The sealed-shared-list cursor test uses one driver for advance/close/begin_close (`OriginalRetirementFailure`) and one for take (`ValueError`); it now passes.
- Why the cursor needs 10+ turns (old law <=3): deliberate per-turn granularity of the 733 projection cursor, not a regression. The neutral law `📽️projection/🧬️clone/🧪️tests/law.json` declares `cancellationCuts` 0..13 and `maximumTurns` 128. The persistent test now bounds turns by that law (<=14); its own `cuts` stay [0,1,2] (sampled early frontiers; the full frontier sweep is the neutral law). Also deliberate: the first close step of a Row list quotes copy 6224 / capacity 3328 (733 prepays failure-storage births), so "quote <= authored body 4096" was replaced by the selected-event law `next_demand(grant.copy) == d`.
- `debit_admission` is exact again: staged fixture admission is 17 turns charging 37 Work (per-turn quotes 5,3,1,4,4,1,1,1,2,1,1,4,1,4,1,1 plus one); pinned as data in `🎟️update/📐️.json` (`admission.turns`, `admission.work`, schema const) and asserted with equality. Fixture admission work is charged to the case fuel again (`wallet::admit_progress`).
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-refusal2-<n> 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml native`

STATUS: FIXED-verified W-refusal2-44 (persistent --tests 24/24 lib + 10 integration); ⚠️refusal hand receiver FIXED-verified W-refusal2-2
