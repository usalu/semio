# Complete Kernel18 Receiving Diagnostics

Actual registered full Kernel18 terminated 1 in build. The original all-target roster and mutation-testing feature were retained, with the original long profile and 600000 ms build deadline. No test runtime started. Actual compiler emitted six primary diagnostics, all included below. Library compilation had advanced beyond the previous Store210 floor; this is not runtime acceptance.

Raw output: `🗑️generated/fd/kernel-full18.log`. Exact current launch snapshot: `🗑️generated/fd/kernel-full18.launch.json`.

## Family Counts

| Actual Family | Primary Diagnostics | Owner |
|---|---:|---|
| Store native snapshot allocation fixture | 1 | Draw |
| Canonical native snapshot control receiving producer | 5 | Draw / current original nested-custody producer |

## Every Primary Diagnostic

### 1. E0593: closure is expected to take 4 arguments, but it takes 3 arguments

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/../../../🧪️tests/💰️allocation/🦀️.rs:126:46`. Raw log line 24168.

```text
error[E0593]: closure is expected to take 4 arguments, but it takes 3 arguments
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/../../../🧪️tests/💰️allocation/🦀️.rs:126:46
    |
126 | ...r>=store::decode_sqlite_snapshot_record_native(...},&mut control,&mut snapshot_decoding_owner);
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^...^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |       |                                                                                                                 |
    |       |                                                                                                                 takes 3 arguments
    |       expected closure that takes 4 arguments

```

### 2. E0592: duplicate definitions with name `receive_nested`

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:45:2`. Raw log line 37333.

```text
error[E0592]: duplicate definitions with name `receive_nested`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:45:2
   |
43 |  pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<O,ValueError>)->Result<O,ValueError>{let...
   |  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- other definition for `receive_nested`
44 |  /// 🪆️ Keeps child construction and its exact receipts inside the original admitted parent frame.
45 |  pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_, '_>)->Result<O,ValueError>)->Result<O,ValueError>{se...
   |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `receive_nested`

[cargo:build] running elapsedMs=80018
```

### 3. E0277: expected an `FnOnce(&mut std::option::Option<_>, &mut semio_framework_dsl::NativeDecodeControl<'control>, &mut io::control::NativeSnapshotBodyWallet)` closure, found `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:41:422`. Raw log line 37343.

```text
error[E0277]: expected an `FnOnce(&mut std::option::Option<_>, &mut semio_framework_dsl::NativeDecodeControl<'control>, &mut io::control::NativeSnapshotBodyWallet)` closure, found `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:41:422
    |
 41 | ...fault();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,_pending...
    |                       -----------------                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnOnce(&mut std::option::Option<_>, &mut semio_framework_dsl::NativeDecodeControl<'control>, &mut io::control::NativeSnapshotBodyWallet)` closure, found `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`
    |                       |
    |                       required by a bound introduced by this call
    |
    = help: the trait `for<'a, 'b, 'c> FnOnce(&'a mut std::option::Option<_>, &'b mut semio_framework_dsl::NativeDecodeControl<'control>, &'c mut io::control::NativeSnapshotBodyWallet)` is not implemented for `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`
note: required by a bound in `receive`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:112:172
    |
112 | ...impl FnOnce(&mut Option<T>,&mut N,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `receive`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACTIO/🗑️generated/k/o/semio_framework_os_kernel-da6532544f5bfbd9.long-type-17742027246060340411.txt'
    = note: consider using `--verbose` to print the full type name to the console

```

### 4. E0061: this function takes 4 arguments but 5 arguments were supplied

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:41:367`. Raw log line 37360.

```text
error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:41:367
    |
 41 | ...lt=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,_pending|operation(slot,native,body));s...
    |       ^^^^^^^^^^^^^^^^^                                                                  ------------------------------------------------------ unexpected argument #5 of type `{closure@🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:41:450: 41:477}`
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:112:15
    |
112 | pub(super) fn receive<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(native:&mut N,original_grant:RetainedCloneGrant,performed:&m...
    |               ^^^^^^^
help: remove the extra argument
    |
 41 -  pub fn receive<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeDecodeControl<'_>,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,_pending|operation(slot,native,body));self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
 41 +  pub fn receive<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeDecodeControl<'_>,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut());self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
    |

```

### 5. E0277: expected an `FnOnce(&mut std::option::Option<_>, &mut semio_framework_dsl::NativeDecodeControl<'control>, &mut io::control::NativeSnapshotBodyWallet)` closure, found `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:43:408`. Raw log line 37377.

```text
error[E0277]: expected an `FnOnce(&mut std::option::Option<_>, &mut semio_framework_dsl::NativeDecodeControl<'control>, &mut io::control::NativeSnapshotBodyWallet)` closure, found `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:43:408
    |
 43 | ...fault();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,pending|...
    |                       -----------------                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnOnce(&mut std::option::Option<_>, &mut semio_framework_dsl::NativeDecodeControl<'control>, &mut io::control::NativeSnapshotBodyWallet)` closure, found `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`
    |                       |
    |                       required by a bound introduced by this call
    |
    = help: the trait `for<'a, 'b, 'c> FnOnce(&'a mut std::option::Option<_>, &'b mut semio_framework_dsl::NativeDecodeControl<'control>, &'c mut io::control::NativeSnapshotBodyWallet)` is not implemented for `Option<&mut Option<Box<dyn ErasedSnapshotRetirement>>>`
note: required by a bound in `receive`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:112:172
    |
112 | ...impl FnOnce(&mut Option<T>,&mut N,&mut NativeSnapshotBodyWallet)->Result<O,ValueError>)->Result<O,ValueError>{
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `receive`
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACTIO/🗑️generated/k/o/semio_framework_os_kernel-da6532544f5bfbd9.long-type-17742027246060340411.txt'
    = note: consider using `--verbose` to print the full type name to the console

```

### 6. E0061: this function takes 4 arguments but 5 arguments were supplied

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:43:353`. Raw log line 37394.

```text
error[E0061]: this function takes 4 arguments but 5 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🦀️.rs:43:353
    |
 43 | ...lt=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,pending|{let mut original=...ginal child custody remains"))}else{result}});s...
    |       ^^^^^^^^^^^^^^^^^                                                                  --------------------------------------------...-------------------------------------------- unexpected argument #5
    |
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:112:15
    |
112 | pub(super) fn receive<T:RetireOwned,O:RetireOwned,N:SnapshotAdmission>(native:&mut N,original_grant:RetainedCloneGrant,performed:&m...
    |               ^^^^^^^
help: remove the extra argument
    |
 43 -  pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut(),|slot,native,body,pending|{let mut original=NativeSnapshotDecodeOwner{native,grant:body.remaining_grant(),progress:Default::default(),wallet:None,pending:Some(pending)};let result=operation(slot,&mut original);let progress=original.progress();let retained=original.pending.as_ref().is_some_and(|slot|slot.is_some());drop(original);body.record_progress(progress)?;if retained&&result.is_ok(){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"nested snapshot cannot publish while original child custody remains"))}else{result}});self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
 43 +  pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned>(&mut self,operation:impl FnOnce(&mut Option<T>,&mut NativeSnapshotDecodeOwner<'_,'_>)->Result<O,ValueError>)->Result<O,ValueError>{let remaining=self.remaining_grant();let mut performed=RetainedCloneProgress::default();let result=snapshot::receive(self.native,remaining,&mut performed,self.pending.as_deref_mut());self.record_progress(performed).map_err(|error|error.with_retained_progress(performed))?;result.map_err(|error|error.with_retained_progress(performed))}
    |

```

## Physical Owned Path Census

Current owned generated k/f/ng/fd census: 8680 files, maximum absolute UTF16 length 247, 0 paths above 256. Generated detail: `🗑️generated/fd/kernel18-paths.json`.
