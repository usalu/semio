# Complete Kernel19 Receiving Diagnostics

Actual unchanged registered full Kernel19 terminated 1 in build. Original all targets, mutation-testing, long-profile deadlines and unfiltered roster were retained. No native test runtime started. The compiler emitted one primary diagnostic, included completely below. The six Kernel18 diagnostics did not recur; disappearance is not runtime acceptance.

Raw output: `🗑️generated/fd/kernel-full19.log`. Current launch snapshot: `🗑️generated/fd/kernel-full19.launch.json`.

## Every Primary Diagnostic

### 1. E0599: no method named `scoped_native` found for mutable reference `&mut io::control::NativeSnapshotDecodeOwner<'_, '_>` in the current scope

Primary span: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs:140:257`. Raw log line 37433.

```text
error[E0599]: no method named `scoped_native` found for mutable reference `&mut io::control::NativeSnapshotDecodeOwner<'_, '_>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🚪️io/../../../../🔨️modules/🚪️io/⏱️control/🧪️tests/🦀️.rs:140:257
    |
140 |    let scoped_events=Cell::new(0usize);let mut scoped=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{assert_eq!(event.owned_bytes,ledger.load(Ordering::Relaxed));scoped_events.set(scoped_events.get()+1);live.get()};let child=original.scoped_native(maximum,&mut scoped,|original|original.receive::<String,String>(|slot,native...
    |                                                                                                                                                                                                                                                                 ^^^^^^^^^^^^^ method not found in `&mut io::control::NativeSnapshotDecodeOwner<'_, '_>`
...
147 |  receiving!(semio_framework_value::NativeDeco...ontrol<'_>|native.copy_text_into(text,slot));
    |  --------------------------------------------...-------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `receiving` (in Nightly builds, run with -Z macro-backtrace for more info)

```
