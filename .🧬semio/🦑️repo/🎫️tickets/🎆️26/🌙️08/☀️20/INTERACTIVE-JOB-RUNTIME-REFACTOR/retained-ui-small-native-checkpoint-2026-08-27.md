# Retained UI Small Native Checkpoint

## Executed Boundaries

The shared compiler lease ran only the existing small UI/async targets; no plugin, native host, WGPU, or Wasm graph was launched. All commands used Bun/Nx, the master `🧱️cargo-target-cad`, and retained `🧪️native-artifacts` metadata. No cleanup occurred.

- Initial document-descendant law:0PASS/1FAIL,102skipped,.021s summary, `🧪️member-ui-document-descendants-red-r1-native-2026-08-27.txt`.
- After DAG's in-place typed/document ownership repair:6PASS/0FAIL,102skipped,.134s summary, `🧪️member-ui-document-typed-green-r2-native-2026-08-27.txt`.
- Full UI regression:108PASS/0FAIL,0skipped,2.742s summary, `🧪️member-ui-full-green-r6-native-2026-08-27.txt`.

The six-law command was `bun x nx run @semio-tech/ui-contract-rs:test --skip-nx-cache --args='--lib -E "test(instance_lifetime_ui_document_) | test(instance_lifetime_ui_typed_)" -- --nocapture'`. Full regression used `--args='--lib -- --nocapture'`. These are actual native UI-contract tests, not app close, renderer, browser, or hard-latency certificates.

## Subsequent Drop-Contention RED

Four new laws then held each actual arena mutex across owner Drop and observed whether the Drop returned before release. They recovered and drained the exact root/descendants before asserting, avoiding a cleanup panic hiding the primary failure. All four returned `waits=true` while the neutral law requires false:

- Document (unstarted/claimed variants):0PASS/1FAIL,111skipped,.293s summary; `🧪️member-ui-document-drop-red-r3-native-2026-08-27.txt`.
- Ordinary UiValue:0PASS/1FAIL,111skipped,.243s; `🧪️member-ui-value-drop-red-r3-native-2026-08-27.txt`.
- Strict unstarted guard:0PASS/1FAIL,111skipped,.230s; `🧪️member-ui-value-unstarted-guard-red-r3-native-2026-08-27.txt`.
- Strict claimed guard:0PASS/1FAIL,111skipped,.222s; `🧪️member-ui-value-claimed-guard-red-r3-native-2026-08-27.txt`.

Each used the existing UI task with its exact function-name filter. Production remained unchanged through these RED cases; DAG received the completed results and source window for the handback repair. No GREEN result is claimed for this newer boundary.

Two earlier grouped invocations did not execute tests: r1 had an unquoted shell filter expression; r2 showed that `partitionNextestExecutionFilters` forwards `--no-fail-fast` to the metadata-build command rather than the execution command. Both retained `🧪️member-ui-drop-contention-red-r{1,2}-native-2026-08-27.txt` logs are infrastructure failures, not semantic RED evidence. Four exact invocations avoided changing that shared helper during this packet; flag-routing remains a scoped test-infrastructure follow-up.

## Handback Repair Gate

After DAG's atomic handback correction, the four Drop laws plus three fixed-bitset helper laws passed7/0,108skipped,.181s summary: `🧪️member-ui-drop-handback-green-r4-native-2026-08-27.txt`.

The subsequent full115 run did not pass: `🧪️member-ui-full-green-r7-native-2026-08-27.txt` contains16PASS/1FAIL with98not run due fail-fast,.247s summary. The filename describes the attempted GREEN run, not its outcome. The sole failing regression was `instance_lifetime_ui_value_retirement_nested_shared_alias_keeps_external_payload`, retirement test line111, actual item/byte total `(3,10)` versus expected `(2,10)`. The seven focused laws had passed and strict caught-Drop panic laws passed; there was no SIGABRT. DAG received this exact accounting regression and the source window for diagnosis. Full UI remains RED at this newer source boundary until rerun.
