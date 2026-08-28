# Existing Component Comparison and Copy — Native RED R31

Canonical runtime command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_ownership_existing_component_retains_comparison_and_copy_between_turns -- --nocapture'`.

Actual exit 1; 0 passed, 1 failed, 98 skipped, 0.023s nextest. The Node Buffer/schema oracle passed 26 checks. Existing Surface records differed only at the final byte of their 32-KiB payloads.

```text
[DEBUG] existing-component-copy turns=1 allocation-ledger=0 old-unchanged=true
assertion failed: turns > data["minimumTurns"].as_u64().unwrap()
Summary [0.023s] 1 test run: 0 passed, 1 failed, 98 skipped
```

The actual old-record route compared and copied the payload within a single turn, with zero allocation debit. Source content remained unchanged and all fixture owners were closed before the final assertion. This confirms the missing retained comparison/copy join; standalone component and document-reader passes do not establish active runtime correctness.

Raw log: `🧪️member-runtime-existing-component-red-r31-native-2026-08-27.txt`. Canonical root integration must also eliminate the current duplicate document-producer clone, retain exact source/candidate owners on failure, and account the comparison-owner admission. No limit change or Process acceptance is claimed.
