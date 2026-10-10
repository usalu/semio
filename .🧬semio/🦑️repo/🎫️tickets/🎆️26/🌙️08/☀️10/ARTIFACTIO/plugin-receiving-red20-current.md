# Plugin Receiving Native Retry 20

The original complete Plugin launch settled terminal 1 before Plugin compilation or native assertions. Shared Job dependency stopped at its one actual compiler floor:

```text
error[E0308]: mismatched types
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../👷️worker/🚪️admission/🦀️.rs:88:95
   |
88 | ...)->Option<usize>{let owner=self.destination_owner();owner.admission_fault_source.as_ref().map(JobPayloadPageSource::backing_identity).or_else(||owner.preadmitted_fault.original().and_then(|payload|payload.pages[0].as_ref().map(|page|page.source.backing_identity())))}
   |       -------------                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<usize>`, found `Option<*const MaybeUninit<u8>>`
   |       |
   |       expected `std::option::Option<usize>` because of return type
   |
   = note: expected enum `Option<usize>`
              found enum `Option<*const std::mem::MaybeUninit<u8>>`

warning: `semio-framework-pack-error` (lib) generated 12 warnings (run `cargo fix --lib -p semio-framework-pack-error` to apply 11 suggestions)
warning: `semio-framework-diagnostic` (lib) generated 44 warnings (run `cargo fix --lib -p semio-framework-diagnostic` to apply 44 suggestions)
warning: `semio-framework-pack-json` (lib) generated 31 warnings (run `cargo fix --lib -p semio-framework-pack-json` to apply 29 suggestions)
[cargo:build] running elapsedMs=10003
warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../⚙️codec/🦀️.rs:435:56
    |
435 |             fn admit(&mut self,bytes:usize)->Result<(),crate::value::ValueError>{self.control.charge(bytes)}
    |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
435 -             fn admit(&mut self,bytes:usize)->Result<(),crate::value::ValueError>{self.control.charge(bytes)}
435 +             fn admit(&mut self,bytes:usize)->Result<(),ValueError>{self.control.charge(bytes)}
    |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../⚙️codec/🦀️.rs:436:102
    |
436 | ...   fn checkpoint(&mut self,event:semio_framework_deflate::DeflateEncodeProgress)->Result<(),crate::value::ValueError>{if self.ph...
    |                                                                                                ^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
436 -             fn checkpoint(&mut self,event:semio_f
```

The newly authored Media fault projection has parse evidence only. Its runtime and compiler acceptance remain unproved. Physical evidence: `🗑️generated/plugin/media-original-signature-oct10-red20.log`. Limits, roster, and original launch environment were unchanged.
