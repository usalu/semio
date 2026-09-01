# Ordinary Aligned Allocation Law — Proposed Phase-Specific Delta

## Current Source Conflict, Not A Native Result

The existing `resident_admission_all_three_page_backings_use_exact_layout_and_short_grants` (canonical test551+) remains UNCHANGED at30af821b. It probes an ordinary **reserve** call using `ConsumerNode<AlignedResident>.size()-1`, then expects Blocked. The reviewed new reserve phase writes only the retained pending descriptor/counter/charge/prepared pointer; allocation is a later call with its own node-layout work. A large aligned C can therefore make the old allocation-sized probe exceed the reserve-write requirement. No C/alignment compatibility branch or maximum-of-unrelated-phases was added to make it pass.

Root approved staging the following exact narrow correction for review, not applying it. R1 actual compiler diagnostics and all existing test expectations remain untouched. No known-stale native run is requested.

## Proposed Canonical Test Delta

Replace only the consumer grant setup/phase drive and corresponding refusal-array indexing. Preserve the original layouts array, admission and record operations, record grant values, envelope, alignment,3alloc/3free, allocated bytes and all-axis usage assertions.

```diff
@@
-    let mut refused = [ResidentStepKind::Pending; 4];
-    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[0].size() as u64 - 1)].into_iter().enumerate() { refused[index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind; }
-    let short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
-    for _ in 0..4 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
+    let reserve_work = [std::mem::size_of::<Option<super::ConsumerPage>>(), std::mem::size_of::<u64>(), std::mem::size_of::<ResidentResources>(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
+    let allocate_work = [layouts[0].size(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>(), std::mem::size_of::<u64>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
+    assert!(reserve_work > 0 && allocate_work > 0 && reserve_work <= grant.max_bytes() && allocate_work <= grant.max_bytes());
+    let mut refused = [ResidentStepKind::Pending; 8];
+    let mut short_consumer_allocations = 0;
+    let mut consumer_unchanged = true;
+    let mut exact_consumer_work = true;
+    for (phase, bytes) in [reserve_work, allocate_work].into_iter().enumerate() {
+        let before = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
+        for (index, short) in [ResidentGrant::new(0, bytes).unwrap(), ResidentGrant::new(1, 0).unwrap(), admission_grant(bytes.checked_sub(1).unwrap())].into_iter().enumerate() {
+            refused[phase * 3 + index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind;
+            consumer_unchanged &= before == (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
+            short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
+        }
+        let exact = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, admission_grant(bytes)).unwrap();
+        exact_consumer_work &= bytes <= grant.max_bytes() && exact.items == 1 && exact.bytes == bytes && exact.kind == ResidentStepKind::Pending;
+    }
+    for _ in 0..2 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
@@
-    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 2] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
+    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 6] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
@@
-    assert_eq!(refused, [ResidentStepKind::Blocked; 4]); assert_eq!(short_consumer_allocations, 0); assert_eq!(before_record, after_short_record);
+    assert_eq!(refused, [ResidentStepKind::Blocked; 8]); assert_eq!(short_consumer_allocations, 0); assert!(consumer_unchanged && exact_consumer_work); assert_eq!(before_record, after_short_record);
```

The two record slots move from indices2/3 to6/7 only because six consumer refusal probes precede them. Their actual calls, work assumptions and expected outcomes are identical. No record behavior change is proposed.

The current whole-three-layout test's original unwrap/error behavior is not upgraded into a new cleanup guarantee by this narrow patch. New intended grant/refusal assertions are collected as booleans and checked after the existing actual close. The new primary7 bodies separately provide the fuller private snapshot/cleanup laws.

The positive-work/4096 assertion occurs before the first consumer reservation or allocation, while the actual root has no descendants. It prevents an oversized derived grant from being passed to a producer; it does not raise the existing maximum.

## Adjacent Neutral Distinction — Currently Missing, Proposed Only

Read actual `resident/📨️admission/🧪️fixture.json`, its schema and `resident/📜️script.ts`. Existing nativeOwnership.exactAllocation distinguishes four construction states and their actual allocation/deallocation count vectors, but nativeOwnership.grantCases names only zero-items/required-minus-one/required and does not bind required to a particular preparation phase.

Proposed additive native-only declaration in that existing fixture/schema (not mounted):

```json
{
  "preparationGrantFrontiers": [
    {
      "phase": "reserve",
      "required": "Option<ConsumerPage>+u64+ResidentResources+Option<NonNull<ConsumerHeader>>",
      "before": { "reserved": false, "allocations": 0 },
      "afterRefusal": { "reserved": false, "allocations": 0 },
      "afterExact": { "reserved": true, "allocations": 0 }
    },
    {
      "phase": "allocate",
      "required": "ConsumerNode<C>.layout+Option<NonNull<ConsumerHeader>>+u64",
      "before": { "reserved": true, "allocations": 0 },
      "afterRefusal": { "reserved": true, "allocations": 0 },
      "afterExact": { "reserved": true, "allocations": 1 }
    }
  ],
  "preparationGrantCases": ["zero-items", "zero-bytes", "required-minus-one", "required"]
}
```

Both keys must be required const declarations in nativeOwnership's adjacent schema. The required expressions are native Layout symbols, NOT TS physical prices or numbers copied from an observed architecture. Keep the existing generic grantCases and TS admission cell charge unchanged.

The existing adjacent Immer cancellation model can additionally consume this exact two-phase sequence: start unreserved/allocations0; each refusal is the unchanged immutable state; exact reserve installs reservation without allocating; exact allocate requires the preceding reservation and increments allocation once. Compare the resulting states to each literal before/afterRefusal/afterExact row. It proves declared phase ownership transitions, not native grant arithmetic. Actual zero-items/zero-bytes/one-short numerical decisions and native byte sums remain the corrected Rust test's responsibility.

No canonical neutral fixture/schema/controller was modified or executed. These changes require root review and source-owner coordination before mount. They do not claim a native or third-party reference result yet.
