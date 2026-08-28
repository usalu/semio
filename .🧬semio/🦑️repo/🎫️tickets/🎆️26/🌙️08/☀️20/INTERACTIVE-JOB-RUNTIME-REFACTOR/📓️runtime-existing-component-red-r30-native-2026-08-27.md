# Existing Component Allocation Before Admission — Native RED R30

Canonical runtime test: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload -- --nocapture'`.

Actual exit 1; 0 passed, 1 failed, 98 skipped; 0.023s nextest. Independent source oracle passed 26 checks. Runtime native output:

```text
[DEBUG] existing-component-refusal rejected=false allocation-before-admission=32768 source-unchanged=true
assertion failed: rejected && source_unchanged
Summary [0.023s] 1 test run: 0 passed, 1 failed, 98 skipped
```

The current existing-record field path allocated the full Surface backing before refusing zero resident credit; the first opportunity did not report a credit fault. The source payload remained unchanged. The fixture closes its retained owners before the assertion. No runtime repair or full regression success is claimed. The canonical-document root replacement remains the required integration, not a second cloned old root.

Raw output: `🧪️member-runtime-existing-component-red-r30-native-2026-08-27.txt`.
