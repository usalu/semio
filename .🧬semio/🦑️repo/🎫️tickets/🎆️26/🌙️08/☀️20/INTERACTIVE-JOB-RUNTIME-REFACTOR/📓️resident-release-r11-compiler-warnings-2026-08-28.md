# Resident Release R11 Compiler Warnings

Original fingerprint: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/.fingerprint/semio-framework-value-resident-ca9d6776d76d4aa3/output-test-lib-semio_framework_value_resident

Read after successful native R11; timestamp 2026-08-28T02:57:04.787Z. Eleven warning diagnostics plus one summary record; no errors. No source changes or warning suppression.

```text
[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:123:108
    [1m[94m|[0m
[1m[94m123[0m [1m[94m|[0m [1m[94m...[0mcations={allocations} permit_mounted=false", std::mem::size_of::<ResidentCapacity>());
    [1m[94m|[0m                                                 [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: requested on the command line with `-W unused-qualifications`
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m123[0m [91m- [0m    eprintln!("[DEBUG] native resident capacity header={} allocations={allocations} permit_mounted=false", [91mstd::mem::[0msize_of::<ResidentCapacity>());
[1m[94m123[0m [92m+ [0m    eprintln!("[DEBUG] native resident capacity header={} allocations={allocations} permit_mounted=false", size_of::<ResidentCapacity>());
    [1m[94m|[0m


[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:193:35
    [1m[94m|[0m
[1m[94m193[0m [1m[94m|[0m     assert_eq!(layout.root_bytes, std::mem::size_of::<ResidentLedgerRoot>() as u64);
    [1m[94m|[0m                                   [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m193[0m [91m- [0m    assert_eq!(layout.root_bytes, [91mstd::mem::[0msize_of::<ResidentLedgerRoot>() as u64);
[1m[94m193[0m [92m+ [0m    assert_eq!(layout.root_bytes, size_of::<ResidentLedgerRoot>() as u64);
    [1m[94m|[0m


[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:195:43
    [1m[94m|[0m
[1m[94m195[0m [1m[94m|[0m     assert!(layout.consumer_move_bytes >= std::mem::size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m                                           [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m195[0m [91m- [0m    assert!(layout.consumer_move_bytes >= [91mstd::mem::[0msize_of::<Option<ResidentDropProbe>>() as u64);
[1m[94m195[0m [92m+ [0m    assert!(layout.consumer_move_bytes >= size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m


[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:196:40
    [1m[94m|[0m
[1m[94m196[0m [1m[94m|[0m     assert!(layout.shell_move_bytes >= std::mem::size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m                                        [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m196[0m [91m- [0m    assert!(layout.shell_move_bytes >= [91mstd::mem::[0msize_of::<Option<ResidentDropProbe>>() as u64);
[1m[94m196[0m [92m+ [0m    assert!(layout.shell_move_bytes >= size_of::<Option<ResidentDropProbe>>() as u64);
    [1m[94m|[0m


[1m[33mwarning[0m[1m: unnecessary qualification[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🧪️tests/🦀️.rs:197:45
    [1m[94m|[0m
[1m[94m197[0m [1m[94m|[0m     assert!(layout.descriptor_move_bytes >= std::mem::size_of::<Vec<u8>>() as u64);
    [1m[94m|[0m                                             [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: remove the unnecessary path segments
    [1m[94m|[0m
[1m[94m197[0m [91m- [0m    assert!(layout.descriptor_move_bytes >= [91mstd::mem::[0msize_of::<Vec<u8>>() as u64);
[1m[94m197[0m [92m+ [0m    assert!(layout.descriptor_move_bytes >= size_of::<Vec<u8>>() as u64);
    [1m[94m|[0m


[1m[33mwarning[0m[1m: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:652:22
    [1m[94m|[0m
[1m[94m652[0m [1m[94m|[0m [1m[94m...[0m   node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    [1m[94m|[0m                    [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(deprecated)]` on by default
[1m[96mhelp[0m: replace the use of the deprecated method
    [1m[94m|[0m
[1m[94m652[0m [91m- [0m        node.aliases.[91mfetch_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
[1m[94m652[0m [92m+ [0m        node.aliases.[92mtry_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    [1m[94m|[0m


[1m[33mwarning[0m[1m: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:560:24
    [1m[94m|[0m
[1m[94m560[0m [1m[94m|[0m [1m[94m...[0m   header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Cou[1m[94m...[0m
    [1m[94m|[0m                      [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
[1m[96mhelp[0m: replace the use of the deprecated method
    [1m[94m|[0m
[1m[94m560[0m [91m- [0m        header.aliases.[91mfetch_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
[1m[94m560[0m [92m+ [0m        header.aliases.[92mtry_update[0m(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    [1m[94m|[0m


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


[1m[33mwarning[0m[1m: field `released_layout` is never read[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:156:13
    [1m[94m|[0m
[1m[94m156[0m [1m[94m|[0m     Clear { released_layout: Option<Layout> },
    [1m[94m|[0m     [1m[94m-----[0m   [1m[33m^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m     [1m[94m|[0m
    [1m[94m|[0m     [1m[94mfield in this variant[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default


[1m[33mwarning[0m[1m: field `origin` is never read[0m
   [1m[94m--> [0m🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:159:5
    [1m[94m|[0m
[1m[94m158[0m [1m[94m|[0m struct ResidentRelease {
    [1m[94m|[0m        [1m[94m---------------[0m [1m[94mfield in this struct[0m
[1m[94m159[0m [1m[94m|[0m     origin: ResidentReleaseOrigin,
    [1m[94m|[0m     [1m[33m^^^^^^[0m


[1m[33mwarning[0m[1m: 11 warnings emitted[0m


```

