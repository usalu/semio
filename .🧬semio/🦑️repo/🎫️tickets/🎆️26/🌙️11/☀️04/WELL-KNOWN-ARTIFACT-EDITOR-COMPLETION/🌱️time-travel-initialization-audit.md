# Time Travel Final Address Initialization Audit

Permanent neutral fixture and schema added under time-travel/tests/fixtures/initialization, with a Bun structural/default oracle under tests/initialization. Exact emoji paths: ⏪️time-travel/🧪️tests/🧫️fixtures/🌱️initialization/{🔣️.json,📋️schema.json}, and ⏪️time-travel/🧪️tests/🌱️initialization/🟦️.ts. Rust production source was not edited.

The neutral roster covers all27TimeTravelLedger fields and41TimeTravelStoreState fields. Independent Ajv validates inputs; the TS oracle compares every declared Rust field with exactly one final-address target reference, derives actual written initial values, and compares both those values and existing Default/new constructors against the neutral corpus. It checks callbacks, mutation label, nested document state, allNone options, falseflags, zerocounters, emptycollections, emptyretirementmetadata and slotwise arrays8/2.

Actual repo Bun execution:3pass0fail104expectations121ms. Prior structural-only run3pass32expectations was strengthened to check constructor-default parity; latest result supersedes it.

No omitted field, undefined initializer, fullSelf write or value mismatch found. Quotes retain typed Option discriminants and2times metadata size; nested document quote is composed into ledger quote. ManuallyDrop array casts preserve the original array layout and slotwise writes avoid an intermediate full array.

This is source coverage and independent neutral-value validation. It does not execute unsafe Rust initialization, certify padding/stack-copy behavior, or measure original retained receipts. Native construction must still prove properly aligned final storage, initialized exactly once, unchanged label/clock callbacks and complete funded retirement. TS is the independent test oracle, not a second runtime implementation of the Rust internal ledger.

Path check: the five changed initialization source/input files have maximum136absoluteUTF16codeunits; none exceeds256.

## Paged Port Oracle Update

The old aggregate initialize_empty_at/initial_copy_bytes ports are replaced. Updated only the permanent TS structural oracle; neutral 27-ledger/41-store default corpus remains unchanged. Actual Bun rerun exited 0: 3 passed, 0 failed, 338 expectations, 144 ms. Checks actual struct fields and Default/new parity, exact destination field writes once, exact demand/initialize arm indices, unchanged neutral initial values, bounded absent field/array templates for every None, no write(None) aggregate, explicit 8/2 array extents and ManuallyDrop casts, atomic full-admission guards, mutation label and default clock preservation. Ledger owns 26 direct arms plus 41 delegated document arms: slots 0..9, 10..50, 51..66 cover exactly 67 positions; both quote and write dispatch use field-10. Absent quotes require body.max(1), preventing zero-byte cursor-only completion. This remains source structural evidence with independent Ajv validation, not native unsafe execution or constructor lifecycle proof. No Rust source changed.
