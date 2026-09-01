# Resident Primary R1 Complete Compiler Diagnostics

Rendered from the preserved compiler JSONL. All77 records retained:65 coded errors,9 warnings,one abort summary andtwo failure notes.

## Diagnostic 1 — error E0432

```text
[1m[91merror[E0432][0m[1m: unresolved imports `crate::ResidentPrimaryAnchor`, `crate::ResidentPrimaryBacking`, `crate::ResidentRecoveryCursor`, `crate::ResidentRecoveryMode`, `crate::ResidentRecoveryPin`[0m
 [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:5:57
  [1m[94m|[0m
[1m[94m5[0m [1m[94m|[0m [1m[94m...[0mrPage, ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecoveryCursor, ResidentRecoveryMode, ResidentRecoveryPin, ResidentR[1m[94m...[0m
  [1m[94m|[0m           [1m[91m^^^^^^^^^^^^^^^^^^^^^[0m  [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m  [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m  [1m[91m^^^^^^^^^^^^^^^^^^^^[0m  [1m[91m^^^^^^^^^^^^^^^^^^^[0m [1m[91mno `ResidentRecoveryPin` in the root[0m
  [1m[94m|[0m           [1m[91m|[0m                      [1m[91m|[0m                       [1m[91m|[0m                       [1m[91m|[0m
  [1m[94m|[0m           [1m[91m|[0m                      [1m[91m|[0m                       [1m[91m|[0m                       [1m[91mno `ResidentRecoveryMode` in the root[0m
  [1m[94m|[0m           [1m[91m|[0m                      [1m[91m|[0m                       [1m[91mno `ResidentRecoveryCursor` in the root[0m
  [1m[94m|[0m           [1m[91m|[0m                      [1m[91mno `ResidentPrimaryBacking` in the root[0m
  [1m[94m|[0m           [1m[91mno `ResidentPrimaryAnchor` in the root[0m
  [1m[94m|[0m
[1m[96mhelp[0m: a similar name exists in the module
  [1m[94m|[0m
[1m[94m5[0m [91m- [0muse crate::{ConsumerHeader, ConsumerNode, ConsumerPage, ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecoveryCursor, [91mResidentRecoveryMode[0m, ResidentRecoveryPin, ResidentRelease, ResidentReleaseStage};
[1m[94m5[0m [92m+ [0muse crate::{ConsumerHeader, ConsumerNode, ConsumerPage, ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecoveryCursor, [92mResidentRecord[0m, ResidentRecoveryPin, ResidentRelease, ResidentReleaseStage};
  [1m[94m|[0m
[1m[96mhelp[0m: a similar name exists in the module
  [1m[94m|[0m
[1m[94m5[0m [91m- [0muse crate::{ConsumerHeader, ConsumerNode, ConsumerPage, ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecoveryCursor, ResidentRecoveryMode, [91mResidentRecoveryPin[0m, ResidentRelease, ResidentReleaseStage};
[1m[94m5[0m [92m+ [0muse crate::{ConsumerHeader, ConsumerNode, ConsumerPage, ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecoveryCursor, ResidentRecoveryMode, [92mResidentRecord[0m, ResidentRelease, ResidentReleaseStage};
  [1m[94m|[0m
```

## Diagnostic 2 — warning unused_qualifications

```text
[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:128:108
    [1m[94m|[0m
[1m[94m128[0m [1m[94m|[0m [1m[94m...[0mcations={allocations} permit_mounted=false", std::mem::size_of::<ResidentCapacity>());
    [1m[94m|[0m                                                 [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: requested on the command line with `-W unused-qualifications`
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m128[0m [91m- [0m    eprintln!("[DEBUG] native resident capacity header={} allocations={allocations} permit_mounted=false", [91mstd::mem::[0msize_of::<ResidentCapacity>());
[1m[94m128[0m [92m+ [0m    eprintln!("[DEBUG] native resident capacity header={} allocations={allocations} permit_mounted=false", size_of::<ResidentCapacity>());
    [1m[94m|[0m
```

## Diagnostic 3 — warning unused_qualifications

```text
[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:198:35
    [1m[94m|[0m
[1m[94m198[0m [1m[94m|[0m     assert_eq!(layout.root_bytes, std::mem::size_of::<ResidentLedgerRoot>() as u64);
    [1m[94m|[0m                                   [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m198[0m [91m- [0m    assert_eq!(layout.root_bytes, [91mstd::mem::[0msize_of::<ResidentLedgerRoot>() as u64);
[1m[94m198[0m [92m+ [0m    assert_eq!(layout.root_bytes, size_of::<ResidentLedgerRoot>() as u64);
    [1m[94m|[0m
```

## Diagnostic 4 — warning unused_qualifications

```text
[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:200:43
    [1m[94m|[0m
[1m[94m200[0m [1m[94m|[0m     assert!(layout.consumer_move_bytes >= std::mem::size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m                                           [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m200[0m [91m- [0m    assert!(layout.consumer_move_bytes >= [91mstd::mem::[0msize_of::<Option<ResidentDropProbe>>() as u64);
[1m[94m200[0m [92m+ [0m    assert!(layout.consumer_move_bytes >= size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m
```

## Diagnostic 5 — warning unused_qualifications

```text
[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:201:40
    [1m[94m|[0m
[1m[94m201[0m [1m[94m|[0m     assert!(layout.shell_move_bytes >= std::mem::size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m                                        [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m201[0m [91m- [0m    assert!(layout.shell_move_bytes >= [91mstd::mem::[0msize_of::<Option<ResidentDropProbe>>() as u64);
[1m[94m201[0m [92m+ [0m    assert!(layout.shell_move_bytes >= size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m
```

## Diagnostic 6 — warning unused_qualifications

```text
[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:202:45
    [1m[94m|[0m
[1m[94m202[0m [1m[94m|[0m     assert!(layout.descriptor_move_bytes >= std::mem::size_of::<Vec<u8>>() as u64);
    [1m[94m|[0m                                             [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m202[0m [91m- [0m    assert!(layout.descriptor_move_bytes >= [91mstd::mem::[0msize_of::<Vec<u8>>() as u64);
[1m[94m202[0m [92m+ [0m    assert!(layout.descriptor_move_bytes >= size_of::<Vec<u8>>() as u64);
    [1m[94m|[0m
```

## Diagnostic 7 — warning deprecated

```text
[1m[33mwarning[0m[1m: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:560:24
    [1m[94m|[0m
[1m[94m560[0m [1m[94m|[0m [1m[94m...[0m   header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Cou[1m[94m...[0m
    [1m[94m|[0m                      [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(deprecated)]` on by default
[1m[96mhelp[0m: replace the use of the deprecated method
    [1m[94m|[0m
[1m[94m560[0m [91m- [0m        header.aliases.[91mfetch_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
[1m[94m560[0m [92m+ [0m        header.aliases.[92mtry_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    [1m[94m|[0m
```

## Diagnostic 8 — warning deprecated

```text
[1m[33mwarning[0m[1m: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:569:60
    [1m[94m|[0m
[1m[94m569[0m [1m[94m|[0m [1m[94m...[0m   unsafe { source.pointer.as_ref().header.admissions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_ad[1m[94m...[0m
    [1m[94m|[0m                                                          [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: replace the use of the deprecated method
    [1m[94m|[0m
[1m[94m569[0m [91m- [0m        unsafe { source.pointer.as_ref().header.admissions.[91mfetch_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
[1m[94m569[0m [92m+ [0m        unsafe { source.pointer.as_ref().header.admissions.[92mtry_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    [1m[94m|[0m
```

## Diagnostic 9 — warning deprecated

```text
[1m[33mwarning[0m[1m: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:652:22
    [1m[94m|[0m
[1m[94m652[0m [1m[94m|[0m [1m[94m...[0m   node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    [1m[94m|[0m                    [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: replace the use of the deprecated method
    [1m[94m|[0m
[1m[94m652[0m [91m- [0m        node.aliases.[91mfetch_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
[1m[94m652[0m [92m+ [0m        node.aliases.[92mtry_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    [1m[94m|[0m
```

## Diagnostic 10 — warning deprecated

```text
[1m[33mwarning[0m[1m: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:732:43
    [1m[94m|[0m
[1m[94m732[0m [1m[94m|[0m [1m[94m...[0m   unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_|[1m[94m...[0m
    [1m[94m|[0m                                         [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: replace the use of the deprecated method
    [1m[94m|[0m
[1m[94m732[0m [91m- [0m        unsafe { pointer.as_ref().aliases.[91mfetch_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
[1m[94m732[0m [92m+ [0m        unsafe { pointer.as_ref().aliases.[92mtry_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    [1m[94m|[0m
```

## Diagnostic 11 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:385:22
    [1m[94m|[0m
[1m[94m385[0m [1m[94m|[0m                 root.reserve_primary_consumer::<u64>(partition, full())?;
    [1m[94m|[0m                      [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m385[0m [91m- [0m                root.[91mreserve_primary_consumer[0m::<u64>(partition, full())?;
[1m[94m385[0m [92m+ [0m                root.[92mprepare_consumer[0m::<u64>(partition, full())?;
    [1m[94m|[0m
```

## Diagnostic 12 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:645:88
    [1m[94m|[0m
[1m[94m645[0m [1m[94m|[0m [1m[94m...[0mspawn(|| observed(None, || root.begin_primary_recovery::<u64>(ResidentRecoveryMode::Forward, full()))).join());
    [1m[94m|[0m                                    [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_recovery` not found for this struct[0m
```

## Diagnostic 13 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:572:28
    [1m[94m|[0m
[1m[94m572[0m [1m[94m|[0m                     shared.begin_primary_recovery::<u64>(ResidentRecoveryMode::Forward, full())?;
    [1m[94m|[0m                            [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `&ResidentLedgerRoot`[0m
```

## Diagnostic 14 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:326:14
    [1m[94m|[0m
[1m[94m326[0m [1m[94m|[0m         root.reserve_primary_consumer::<Tagged>(ResidentPartition::Data, full())?;
    [1m[94m|[0m              [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m326[0m [91m- [0m        root.[91mreserve_primary_consumer[0m::<Tagged>(ResidentPartition::Data, full())?;
[1m[94m326[0m [92m+ [0m        root.[92mprepare_consumer[0m::<Tagged>(ResidentPartition::Data, full())?;
    [1m[94m|[0m
```

## Diagnostic 15 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:471:18
    [1m[94m|[0m
[1m[94m471[0m [1m[94m|[0m             root.reserve_primary_consumer::<u64>(partition, full())?;
    [1m[94m|[0m                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m471[0m [91m- [0m            root.[91mreserve_primary_consumer[0m::<u64>(partition, full())?;
[1m[94m471[0m [92m+ [0m            root.[92mprepare_consumer[0m::<u64>(partition, full())?;
    [1m[94m|[0m
```

## Diagnostic 16 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:267:87
    [1m[94m|[0m
[1m[94m267[0m [1m[94m|[0m [1m[94m...[0m   let mut exact = short_then_exact(&root, work::<u64>(Work::Reserve)?, |g| root.reserve_primary_consumer::<u64>(ResidentPartiti[1m[94m...[0m
    [1m[94m|[0m                                                                                     [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m267[0m [91m- [0m        let mut exact = short_then_exact(&root, work::<u64>(Work::Reserve)?, |g| root.[91mreserve_primary_consumer[0m::<u64>(ResidentPartition::Data, g))?;
[1m[94m267[0m [92m+ [0m        let mut exact = short_then_exact(&root, work::<u64>(Work::Reserve)?, |g| root.[92mprepare_consumer[0m::<u64>(ResidentPartition::Data, g))?;
    [1m[94m|[0m
```

## Diagnostic 17 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:517:84
    [1m[94m|[0m
[1m[94m517[0m [1m[94m|[0m [1m[94m...[0m::<u64>(Work::Begin)?, |g| root.begin_primary_recovery::<u64>(ResidentRecoveryMode::Forward, g))?;
    [1m[94m|[0m                                    [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_recovery` not found for this struct[0m
```

## Diagnostic 18 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:386:42
    [1m[94m|[0m
[1m[94m386[0m [1m[94m|[0m                 for _ in 1..calls { root.prepare_primary_consumer::<u64>(full())?; }
    [1m[94m|[0m                                          [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 19 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:333:30
    [1m[94m|[0m
[1m[94m333[0m [1m[94m|[0m         for _ in 0..3 { root.prepare_primary_consumer::<Tagged>(full())?; }
    [1m[94m|[0m                              [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 20 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:522:29
    [1m[94m|[0m
[1m[94m522[0m [1m[94m|[0m         let overflow = root.advance_primary_recovery::<u64>(grant(work::<u64>(Work::Advance)?)?);
    [1m[94m|[0m                             [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `advance_primary_recovery` not found for this struct[0m
```

## Diagnostic 21 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:574:110
    [1m[94m|[0m
[1m[94m574[0m [1m[94m|[0m [1m[94m...[0mnd.is_some() { break; } shared.advance_primary_recovery::<u64>(full())?; }
    [1m[94m|[0m                                   [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `&ResidentLedgerRoot`[0m
```

## Diagnostic 22 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:275:78
    [1m[94m|[0m
[1m[94m275[0m [1m[94m|[0m                     let (result, events) = observed(context(&root)?, || root.prepare_primary_consumer::<u64>(refused));
    [1m[94m|[0m                                                                              [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 23 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:474:69
    [1m[94m|[0m
[1m[94m474[0m [1m[94m|[0m             let (result, events) = observed(Some(selected), || root.prepare_primary_consumer::<u64>(full()));
    [1m[94m|[0m                                                                     [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 24 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:651:26
    [1m[94m|[0m
[1m[94m651[0m [1m[94m|[0m         let wrong = root.begin_primary_recovery::<u8>(ResidentRecoveryMode::Forward, full());
    [1m[94m|[0m                          [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_recovery` not found for this struct[0m
```

## Diagnostic 25 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:388:22
    [1m[94m|[0m
[1m[94m388[0m [1m[94m|[0m                 root.begin_primary_consumer_close(full())?;
    [1m[94m|[0m                      [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_consumer_close` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 26 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `ResidentAccessGuard<'_, LedgerState>`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:395:31
    [1m[94m|[0m
[1m[94m395[0m [1m[94m|[0m                         state.primary.as_ref().is_some_and(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Releasing))
    [1m[94m|[0m                               [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `access`, `thread`
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 27 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:527:79
    [1m[94m|[0m
[1m[94m527[0m [1m[94m|[0m         good &= short_then_exact(&root, work::<u64>(Work::Advance)?, |g| root.advance_primary_recovery::<u64>(g))?;
    [1m[94m|[0m                                                                               [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `advance_primary_recovery` not found for this struct[0m
```

## Diagnostic 28 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:352:14
    [1m[94m|[0m
[1m[94m352[0m [1m[94m|[0m         root.begin_primary_consumer_close(full())?;
    [1m[94m|[0m              [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_consumer_close` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 29 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:579:70
    [1m[94m|[0m
[1m[94m579[0m [1m[94m|[0m                     let (attempt, events) = observed(None, || shared.advance_primary_recovery::<u64>(full()));
    [1m[94m|[0m                                                                      [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `&ResidentLedgerRoot`[0m
```

## Diagnostic 30 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:278:74
    [1m[94m|[0m
[1m[94m278[0m [1m[94m|[0m                 let (result, events) = observed(context(&root)?, || root.prepare_primary_consumer::<u64>(grant(bytes)?));
    [1m[94m|[0m                                                                          [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 31 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:652:27
    [1m[94m|[0m
[1m[94m652[0m [1m[94m|[0m         let replay = root.reserve_primary_consumer::<u64>(ResidentPartition::Data, full());
    [1m[94m|[0m                           [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m652[0m [91m- [0m        let replay = root.[91mreserve_primary_consumer[0m::<u64>(ResidentPartition::Data, full());
[1m[94m652[0m [92m+ [0m        let replay = root.[92mprepare_consumer[0m::<u64>(ResidentPartition::Data, full());
    [1m[94m|[0m
```

## Diagnostic 32 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:480:18
    [1m[94m|[0m
[1m[94m480[0m [1m[94m|[0m             root.begin_primary_consumer_close(full())?;
    [1m[94m|[0m                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_consumer_close` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 33 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:531:77
    [1m[94m|[0m
[1m[94m531[0m [1m[94m|[0m         good &= short_then_exact(&root, work::<u64>(Work::Match)?, |g| root.advance_primary_recovery::<u64>(g))?;
    [1m[94m|[0m                                                                             [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `advance_primary_recovery` not found for this struct[0m
```

## Diagnostic 34 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:354:14
    [1m[94m|[0m
[1m[94m354[0m [1m[94m|[0m         root.begin_primary_recovery::<Tagged>(ResidentRecoveryMode::Closing, full())?;
    [1m[94m|[0m              [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_recovery` not found for this struct[0m
```

## Diagnostic 35 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:281:71
    [1m[94m|[0m
[1m[94m281[0m [1m[94m|[0m             } else { exact &= short_then_exact(&root, bytes, |g| root.prepare_primary_consumer::<u64>(g))?; }
    [1m[94m|[0m                                                                       [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 36 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:654:14
    [1m[94m|[0m
[1m[94m654[0m [1m[94m|[0m         root.begin_primary_consumer_close(full())?;
    [1m[94m|[0m              [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_consumer_close` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 37 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `capture_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:535:29
    [1m[94m|[0m
[1m[94m535[0m [1m[94m|[0m         let overflow = root.capture_primary_consumer::<u64>(ResidentRecoveryMode::Forward, grant(bytes)?);
    [1m[94m|[0m                             [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `capture_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m535[0m [91m- [0m        let overflow = root.[91mcapture_primary_consumer[0m::<u64>(ResidentRecoveryMode::Forward, grant(bytes)?);
[1m[94m535[0m [92m+ [0m        let overflow = root.[92mprepare_consumer[0m::<u64>(ResidentRecoveryMode::Forward, grant(bytes)?);
    [1m[94m|[0m
```

## Diagnostic 38 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:429:14
    [1m[94m|[0m
[1m[94m429[0m [1m[94m|[0m         root.begin_primary_consumer_close(full())?;
    [1m[94m|[0m              [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_consumer_close` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 39 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:241:48
    [1m[94m|[0m
[1m[94m241[0m [1m[94m|[0m     short_then_exact(root, bytes, |grant| root.begin_primary_consumer_close(grant))
    [1m[94m|[0m                                                [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 40 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `ResidentAccessGuard<'_, LedgerState>`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:246:24
    [1m[94m|[0m
[1m[94m246[0m [1m[94m|[0m     let anchor = state.primary.as_ref().ok_or(ResidentFault::Identity)?;
    [1m[94m|[0m                        [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `access`, `thread`
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 41 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `ResidentAccessGuard<'_, LedgerState>`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:254:24
    [1m[94m|[0m
[1m[94m254[0m [1m[94m|[0m     let anchor = state.primary.as_ref().ok_or(ResidentFault::Identity)?;
    [1m[94m|[0m                        [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `access`, `thread`
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 42 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `capture_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:544:63
    [1m[94m|[0m
[1m[94m544[0m [1m[94m|[0m                 let (result, events) = observed(None, || root.capture_primary_consumer::<u64>(ResidentRecoveryMode::Forward, short));
    [1m[94m|[0m                                                               [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `capture_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m544[0m [91m- [0m                let (result, events) = observed(None, || root.[91mcapture_primary_consumer[0m::<u64>(ResidentRecoveryMode::Forward, short));
[1m[94m544[0m [92m+ [0m                let (result, events) = observed(None, || root.[92mprepare_consumer[0m::<u64>(ResidentRecoveryMode::Forward, short));
    [1m[94m|[0m
```

## Diagnostic 43 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:357:64
    [1m[94m|[0m
[1m[94m357[0m [1m[94m|[0m         let (revoked, revoked_events) = observed(None, || root.advance_primary_recovery::<Tagged>(full()));
    [1m[94m|[0m                                                                [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `advance_primary_recovery` not found for this struct[0m
```

## Diagnostic 44 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:655:28
    [1m[94m|[0m
[1m[94m655[0m [1m[94m|[0m         let forward = root.begin_primary_recovery::<u64>(ResidentRecoveryMode::Forward, full());
    [1m[94m|[0m                            [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_recovery` not found for this struct[0m
```

## Diagnostic 45 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:185:30
    [1m[94m|[0m
[1m[94m185[0m [1m[94m|[0m     let begin = checked(root.begin_primary_recovery::<C>(mode, full())?, full())?;
    [1m[94m|[0m                              [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `&ResidentLedgerRoot`[0m
```

## Diagnostic 46 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `capture_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:549:37
    [1m[94m|[0m
[1m[94m549[0m [1m[94m|[0m         let (step, consumer) = root.capture_primary_consumer::<u64>(ResidentRecoveryMode::Forward, grant(bytes)?)?;
    [1m[94m|[0m                                     [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `capture_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m549[0m [91m- [0m        let (step, consumer) = root.[91mcapture_primary_consumer[0m::<u64>(ResidentRecoveryMode::Forward, grant(bytes)?)?;
[1m[94m549[0m [92m+ [0m        let (step, consumer) = root.[92mprepare_consumer[0m::<u64>(ResidentRecoveryMode::Forward, grant(bytes)?)?;
    [1m[94m|[0m
```

## Diagnostic 47 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:304:18
    [1m[94m|[0m
[1m[94m304[0m [1m[94m|[0m         oversize.reserve_primary_consumer::<[u8; 4096]>(ResidentPartition::Data, full())?;
    [1m[94m|[0m                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m304[0m [91m- [0m        oversize.[91mreserve_primary_consumer[0m::<[u8; 4096]>(ResidentPartition::Data, full())?;
[1m[94m304[0m [92m+ [0m        oversize.[92mprepare_consumer[0m::<[u8; 4096]>(ResidentPartition::Data, full())?;
    [1m[94m|[0m
```

## Diagnostic 48 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `ResidentAccessGuard<'_, LedgerState>`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:130:14
    [1m[94m|[0m
[1m[94m130[0m [1m[94m|[0m     Ok(state.primary.as_ref().and_then(|anchor| match &anchor.backing {
    [1m[94m|[0m              [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `access`, `thread`
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 49 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:448:24
    [1m[94m|[0m
[1m[94m448[0m [1m[94m|[0m     let forward = root.prepare_primary_consumer::<u64>(full());
    [1m[94m|[0m                        [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 50 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:174:18
    [1m[94m|[0m
[1m[94m174[0m [1m[94m|[0m     checked(root.reserve_primary_consumer::<C>(partition, full())?, full())?;
    [1m[94m|[0m                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m174[0m [91m- [0m    checked(root.[91mreserve_primary_consumer[0m::<C>(partition, full())?, full())?;
[1m[94m174[0m [92m+ [0m    checked(root.[92mprepare_consumer[0m::<C>(partition, full())?, full())?;
    [1m[94m|[0m
```

## Diagnostic 51 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `registration` on type `&ConsumerHeader`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:103:57
    [1m[94m|[0m
[1m[94m103[0m [1m[94m|[0m [1m[94m...[0m   nodes[index] = Some(Node { registration: header.registration.get(), type_id: header.type_id, address: pointer.as_ptr() as usi[1m[94m...[0m
    [1m[94m|[0m                                                       [1m[91m^^^^^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `aliases`, `admissions`, `closing`, `next`, `type_id`
```

## Diagnostic 52 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `recovery_pins` on type `&ConsumerHeader`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:103:151
    [1m[94m|[0m
[1m[94m103[0m [1m[94m|[0m [1m[94m...[0m address: pointer.as_ptr() as usize, pins: header.recovery_pins.load(Ordering::Acquire), aliases: header.aliases.load(Ordering::[1m[94m...[0m
    [1m[94m|[0m                                                      [1m[91m^^^^^^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `aliases`, `admissions`, `closing`, `next`, `type_id`
```

## Diagnostic 53 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:660:22
    [1m[94m|[0m
[1m[94m660[0m [1m[94m|[0m     let stale = root.begin_primary_recovery::<u64>(ResidentRecoveryMode::Forward, full());
    [1m[94m|[0m                      [1m[91m^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `ResidentLedgerRoot`[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `begin_primary_recovery` not found for this struct[0m
```

## Diagnostic 54 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:106:42
    [1m[94m|[0m
[1m[94m106[0m [1m[94m|[0m     let (backing, pending) = match state.primary.as_ref().map(|anchor| &anchor.backing) {
    [1m[94m|[0m                                          [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 55 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `capture_primary_consumer` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:189:41
    [1m[94m|[0m
[1m[94m189[0m [1m[94m|[0m             let (step, consumer) = root.capture_primary_consumer::<C>(mode, full())?;
    [1m[94m|[0m                                         [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m189[0m [91m- [0m            let (step, consumer) = root.[91mcapture_primary_consumer[0m::<C>(mode, full())?;
[1m[94m189[0m [92m+ [0m            let (step, consumer) = root.[92mprepare_consumer[0m::<C>(mode, full())?;
    [1m[94m|[0m
```

## Diagnostic 56 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `last_consumer_registration` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:114:21
    [1m[94m|[0m
[1m[94m114[0m [1m[94m|[0m [1m[94m...[0m   last: state.last_consumer_registration, primary: state.primary.as_ref().map(|anchor| (anchor.stamp.generation.get(), anchor.s[1m[94m...[0m
    [1m[94m|[0m                   [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 57 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:114:64
    [1m[94m|[0m
[1m[94m114[0m [1m[94m|[0m [1m[94m...[0m   last: state.last_consumer_registration, primary: state.primary.as_ref().map(|anchor| (anchor.stamp.generation.get(), anchor.s[1m[94m...[0m
    [1m[94m|[0m                                                              [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 58 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `primary` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:115:34
    [1m[94m|[0m
[1m[94m115[0m [1m[94m|[0m [1m[94m...[0m   primary_partition: state.primary.as_ref().map(|anchor| anchor.partition), prepared: state.prepared_consumer.map(|pointer| poi[1m[94m...[0m
    [1m[94m|[0m                                [1m[91m^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 59 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `registration` on type `&ConsumerPage`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:116:76
    [1m[94m|[0m
[1m[94m116[0m [1m[94m|[0m [1m[94m...[0m   ordinary_pending: state.pending_consumer.as_ref().map(|page| (page.registration.get(), page.pointer.map(|pointer| pointer.as_[1m[94m...[0m
    [1m[94m|[0m                                                                          [1m[91m^^^^^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `pointer`, `layout`, `partition`, `charge`, `initialized` ... and 4 others
```

## Diagnostic 60 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `recovery` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:117:39
    [1m[94m|[0m
[1m[94m117[0m [1m[94m|[0m         backing, pending, next: state.recovery.as_ref().and_then(|cursor| cursor.next.as_ref().map(|pin| pin.registration.get())),
    [1m[94m|[0m                                       [1m[91m^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 61 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `recovery` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:118:22
    [1m[94m|[0m
[1m[94m118[0m [1m[94m|[0m         found: state.recovery.as_ref().and_then(|cursor| cursor.found.as_ref().map(|pin| pin.registration.get())),
    [1m[94m|[0m                      [1m[91m^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 62 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `recovery` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:119:24
    [1m[94m|[0m
[1m[94m119[0m [1m[94m|[0m         revoked: state.recovery.as_ref().is_some_and(|cursor| cursor.revoked), cursor: state.recovery.is_some(),
    [1m[94m|[0m                        [1m[91m^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 63 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `recovery` on type `&LedgerState`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:119:94
    [1m[94m|[0m
[1m[94m119[0m [1m[94m|[0m         revoked: state.recovery.as_ref().is_some_and(|cursor| cursor.revoked), cursor: state.recovery.is_some(),
    [1m[94m|[0m                                                                                              [1m[91m^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 64 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:175:34
    [1m[94m|[0m
[1m[94m175[0m [1m[94m|[0m     for _ in 0..3 { checked(root.prepare_primary_consumer::<C>(full())?, full())?; }
    [1m[94m|[0m                                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 65 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:306:59
    [1m[94m|[0m
[1m[94m306[0m [1m[94m|[0m         let (result, events) = observed(None, || oversize.prepare_primary_consumer::<[u8; 4096]>(full()));
    [1m[94m|[0m                                                           [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `prepare_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 66 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:676:57
    [1m[94m|[0m
[1m[94m676[0m [1m[94m|[0m         let (primary, pe) = observed(None, || full_root.reserve_primary_consumer::<u64>(ResidentPartition::Data, full()));
    [1m[94m|[0m                                                         [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m676[0m [91m- [0m        let (primary, pe) = observed(None, || full_root.[91mreserve_primary_consumer[0m::<u64>(ResidentPartition::Data, full()));
[1m[94m676[0m [92m+ [0m        let (primary, pe) = observed(None, || full_root.[92mprepare_consumer[0m::<u64>(ResidentPartition::Data, full()));
    [1m[94m|[0m
```

## Diagnostic 67 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `advance_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:193:22
    [1m[94m|[0m
[1m[94m193[0m [1m[94m|[0m         checked(root.advance_primary_recovery::<C>(full())?, full())?;
    [1m[94m|[0m                      [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91mmethod not found in `&ResidentLedgerRoot`[0m
```

## Diagnostic 68 — error E0109

```text
[1m[91merror[E0109][0m[1m: type arguments are not allowed on local variable[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:690:20
    [1m[94m|[0m
[1m[94m690[0m [1m[94m|[0m         ordinary::<u8>(&exhausted, ResidentPartition::Control)?;
    [1m[94m|[0m         [1m[94m--------[0m   [1m[91m^^[0m [1m[91mtype argument not allowed[0m
    [1m[94m|[0m         [1m[94m|[0m
    [1m[94m|[0m         [1m[94mnot allowed on local variable[0m
```

## Diagnostic 69 — error E0618

```text
[1m[91merror[E0618][0m[1m: expected function, found `Result<ResidentStep, _>`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:690:9
    [1m[94m|[0m
[1m[94m179[0m [1m[94m|[0m fn ordinary<C: Send + 'static>(root: &ResidentLedgerRoot, partition: ResidentPartition) -> Result<(), ResidentFault> {
    [1m[94m|[0m [1m[94m--------------------------------------------------------------------------------------------------------------------[0m [1m[94mthis function of the same name is available here, but it's shadowed by the local binding[0m
[1m[94m...[0m
[1m[94m682[0m [1m[94m|[0m     let (before, after, primary, ordinary, pe, oe) = capacity_observation?;
    [1m[94m|[0m                                  [1m[94m--------[0m [1m[94m`ordinary` has type `Result<ResidentStep, _>`[0m
[1m[94m...[0m
[1m[94m690[0m [1m[94m|[0m         ordinary::<u8>(&exhausted, ResidentPartition::Control)?;
    [1m[94m|[0m         [1m[91m^^^^^^^^^^^^^^[0m[1m[94m----------------------------------------[0m
    [1m[94m|[0m         [1m[94m|[0m
    [1m[94m|[0m         [1m[94mcall expression requires function[0m
```

## Diagnostic 70 — error E0609

```text
[1m[91merror[E0609][0m[1m: no field `last_consumer_registration` on type `ResidentAccessGuard<'_, LedgerState>`[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:691:57
    [1m[94m|[0m
[1m[94m691[0m [1m[94m|[0m         exhausted.access()?.ok_or(ResidentFault::Busy)?.last_consumer_registration = last;
    [1m[94m|[0m                                                         [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[91munknown field[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: available fields are: `access`, `thread`
    [1m[94m= [0m[1mnote[0m: available fields are: `capacity`, `data`, `control`, `allocated_bytes`, `head` ... and 9 others
```

## Diagnostic 71 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `begin_primary_consumer_close` found for reference `&ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:201:48
    [1m[94m|[0m
[1m[94m201[0m [1m[94m|[0m     if before.primary.is_some() { checked(root.begin_primary_consumer_close(full())?, full())?; } else { root.begin_close()?; }
    [1m[94m|[0m                                                [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name, but with different arguments
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:512:5
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
    [1m[94m|[0m     [1m[96m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
```

## Diagnostic 72 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:693:84
    [1m[94m|[0m
[1m[94m693[0m [1m[94m|[0m [1m[94m...[0m   let (first, first_events) = observed(None, || if primary_first { exhausted.reserve_primary_consumer::<u64>(ResidentPartition:[1m[94m...[0m
    [1m[94m|[0m                                                                                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m693[0m [91m- [0m        let (first, first_events) = observed(None, || if primary_first { exhausted.[91mreserve_primary_consumer[0m::<u64>(ResidentPartition::Data, full()) } else { exhausted.prepare_consumer::<u64>(ResidentPartition::Data, full()) });
[1m[94m693[0m [92m+ [0m        let (first, first_events) = observed(None, || if primary_first { exhausted.[92mprepare_consumer[0m::<u64>(ResidentPartition::Data, full()) } else { exhausted.prepare_consumer::<u64>(ResidentPartition::Data, full()) });
    [1m[94m|[0m
```

## Diagnostic 73 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:703:76
    [1m[94m|[0m
[1m[94m703[0m [1m[94m|[0m [1m[94m...[0m   let primary = if !primary_first { Some(observed(None, || exhausted.reserve_primary_consumer::<u64>(ResidentPartition::Data, f[1m[94m...[0m
    [1m[94m|[0m                                                                          [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m703[0m [91m- [0m        let primary = if !primary_first { Some(observed(None, || exhausted.[91mreserve_primary_consumer[0m::<u64>(ResidentPartition::Data, full()))) } else { None };
[1m[94m703[0m [92m+ [0m        let primary = if !primary_first { Some(observed(None, || exhausted.[92mprepare_consumer[0m::<u64>(ResidentPartition::Data, full()))) } else { None };
    [1m[94m|[0m
```

## Diagnostic 74 — error E0599

```text
[1m[91merror[E0599][0m[1m: no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs:707:34
    [1m[94m|[0m
[1m[94m707[0m [1m[94m|[0m         let no_reset = exhausted.reserve_primary_consumer::<u64>(ResidentPartition::Data, full());
    [1m[94m|[0m                                  [1m[91m^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
   [1m[94m::: [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:148:1
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
    [1m[94m|[0m [1m[94m-----------------------------[0m [1m[94mmethod `reserve_primary_consumer` not found for this struct[0m
    [1m[94m|[0m
[1m[96mhelp[0m: there is a method `prepare_consumer` with a similar name
    [1m[94m|[0m
[1m[94m707[0m [91m- [0m        let no_reset = exhausted.[91mreserve_primary_consumer[0m::<u64>(ResidentPartition::Data, full());
[1m[94m707[0m [92m+ [0m        let no_reset = exhausted.[92mprepare_consumer[0m::<u64>(ResidentPartition::Data, full());
    [1m[94m|[0m
```

## Diagnostic 75 — error

```text
[1m[91merror[0m[1m: aborting due to 65 previous errors; 9 warnings emitted[0m
```

## Diagnostic 76 — failure-note

```text
[1mSome errors have detailed explanations: E0109, E0432, E0599, E0609, E0618.[0m
```

## Diagnostic 77 — failure-note

```text
[1mFor more information about an error, try `rustc --explain E0109`.[0m
```
