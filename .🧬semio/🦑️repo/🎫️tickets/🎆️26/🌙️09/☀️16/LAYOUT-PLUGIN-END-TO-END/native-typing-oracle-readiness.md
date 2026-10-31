# Native Typing Oracle Readiness

Source-only review of actual authored native record emitters and ticket oracles. No build, test, oracle or native execution occurred. Actual native records remain required before claiming equivalence.

## Concrete Coverage Defects

1. typing-edit-oracle.ts lines 20–21 accepts ten records by count without a seen-name set. Ten copies of one valid named record pass, leaving nine native cases unverified. The native harness emits the ten cases at pure EDIT tests line 33. Require unique names and the exact expected key set.
2. typing-original-oracle.ts lines 20–25 has the same defect for eight original commit records: eight copies of one known result pass while seven cases are absent. Native emitter: original commit tests line 32. Require unique names and the exact expected key set.
3. text-splice-native-oracle.ts line 49 requires only fixture and generated counts. It does not require any typing-run native record although the actual native harness emits source typing at tests line 35. Removing every typing record leaves the native log acceptable. Require exact typing-run record coverage using a stable run/step identity emitted by the harness.
4. The composer fixture branch (oracle lines 49–50) does not require unique fixture indexes: repeated valid index records can replace omitted fixture cases while preserving its count. Require the exact fixture index set. Generated records similarly have no stable identity or input coverage comparison: 128 identical valid generated records satisfy the 128 count. Native generated emitter line 37 currently emits no index; exact deterministic generated coverage needs a stable identity and expected input verification.

## Ready Record Comparison Paths

Writer fold oracle lines 37–39 checks record count, unique names, known names, complete expected-key coverage and all result fields. Its ten expected results include complete native splice output for Net and the policy kind for Split/ReplaceIncoming. Ajv validates fixtures; independent fast-json-patch applies sequential and composed text changes. Native emitter: Writer fold tests line 43.

Pure EDIT compares complete steps/committed/open results against SQLite-derived selection and net state, using Ajv and fast-json-patch. Original commit compares windows/leaves/tools/suppress against SQLite-derived ordering and policy with Ajv and fast-json-patch. Their comparison fields are substantive; the concrete gap is exact unique native coverage.

Composer compares each provided actual output with the existing TypeScript algebra using JSON Patch assertions. Its reference phase separately crosschecks an authored bounded algorithm over existing compositions, typing runs and fast-check generated cases. Native cancellation records must match the complete fixture cut sequence; the oracle also requires the exact closed/renewed/unchanged expiry record. Those requirements do not fill the missing source typing or unique main-record coverage.

## Native Test Selectors And Record Prefixes

| Owner | Native test selector | Record prefix |
| --- | --- | --- |
| Pure EDIT | original_edit_matches_all_neutral_outputs_with_real_allocations_and_original_backings | [DEBUG] [original-typing-edit] |
| Original commit | original_typing_policy_and_move_only_rows_match_the_neutral_contract | [DEBUG] [original-typing] |
| Composer | original_text_splice_matches_fixture_typing_runs_and_generated_cold_algebras | [DEBUG] [text-splice-native] |
| Composer cancellation | original_text_splice_cancel_closes_every_partial_owner_without_changing_inputs | [DEBUG] [text-splice-cancel] |
| Composer expiry | original_text_splice_expired_source_closes_without_renewing_its_authority | [DEBUG] [text-splice-expiry] |
| Writer fold | original_writer_fold_matches_neutral_inputs_without_moving_or_cloning_original_nets | [writer-native-fold] |

Composer oracle needs the complete owner log including cancellation and expiry tests, with uncaptured stdout. Native execution remains root's sole lane.

## Oracle Commands

Run from repository root after real native logs exist; substitute the actual ticket-generated log path for LOG. Each command must receive a nonempty log argument to activate native comparison. Commands are documented, not executed:

```sh
repo --service=off tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/typing-edit-oracle.ts LOG
repo --service=off tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/typing-original-oracle.ts LOG
repo --service=off tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/text-splice-native-oracle.ts LOG
repo --service=off tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/writer-fold-oracle.ts LOG
```

All four reject a supplied empty log by count. Omitting LOG intentionally runs reference-only and reports nativeCompared false; that output is not native proof. The commands assume the root-configured tools exec Bun dependency environment and have not been executed in this audit.

## Source Resolution Recheck

All coverage findings above are resolved in actual oracle source. This section supersedes their earlier open status. typing-edit-oracle.ts lines 20–32 and typing-original-oracle.ts lines 18–30 now reject unknown and duplicate native names, compare full actual payloads, and require every expected name. Both also reject duplicate fixture names before constructing the reference map.

text-splice-native-oracle.ts lines 49–53 now reconstructs the entire ordered record tape: indexed fixtures, every composing typing-run pair, and all deterministic generated pairs including actual inputs, context and result. JSON Patch equality against that full array prevents a missing record or different repeated record from replacing an expected record. Its generator uses the same u64 wrapping LCG constants and draw order as the native harness lines 36–37. Cancellation and expiry checks require their full expected arrays at lines 54–59.

The hostile runners require a supplied log and validate the unmodified baseline first. typing-oracle-hostile.ts line 6 calls the actual exact-coverage comparator before deriving five negative corpora, then requires the intended error classes. text-splice-native-hostile.ts line 7 requires the actual baseline oracle to succeed before deriving its five negative corpora from those record lines. Neither constructs positive synthetic native records, permits absent baseline records, nor claims a native pass before baseline validation. No remaining concrete coverage defect was identified in this recheck.

This was read-only source verification; no jobs, tests, native execution or code edits occurred. Native behavior and actual-log hostile execution remain pending the root-managed original framework batch 34369 and the separate original-commit gate. Source resolution does not establish native equivalence.
