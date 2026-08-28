# Canonical Existing-Component Join R45–R46

R45 stopped in the new source fixture join before Cargo: its schema declared draft2020-12 while the existing oracle imports draft7 Ajv. The new schema now declares the same draft7 dialect; no constraints were removed.

R46 canonical command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_ownership_existing_component_ -- --nocapture'`.

Actual exit0: **2 passed, 104 skipped, 0.052s**. These are the original R30/R31 assertions, now using the actual canonical current root rather than the removed production payload map.

```text
[DEBUG] existing-component-refusal rejected=true allocation-before-admission=0 source-unchanged=true
[DEBUG] existing-component-copy turns=35 allocation-ledger=32768 old-unchanged=true
Summary [0.052s] 2 tests run: 2 passed, 104 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

Current production owns UiDocumentLease/Assembly and ID→ordinal metadata, not retained/new_retained payload maps. The old independently applied keyed-diff oracle is now cfg(test)-only and installs its result into a canonical root. The existing-component compare owns the exact old read plus incoming component across turns, debits its full admission/init/root moves, and copies changed bytes under the existing4KiB work/32KiB physical grants.

Still open: actual nine-job/reader replacement gates, complete physical resident census and original inline RED, transaction paired output ownership, full runtime/UI regressions, and all normal/refusal/unwind joins. No whole-cutover/Process-fit/consumed-Wasm claim.

Raw: `🧪️member-runtime-canonical-join-r46-native-2026-08-27.txt`.
