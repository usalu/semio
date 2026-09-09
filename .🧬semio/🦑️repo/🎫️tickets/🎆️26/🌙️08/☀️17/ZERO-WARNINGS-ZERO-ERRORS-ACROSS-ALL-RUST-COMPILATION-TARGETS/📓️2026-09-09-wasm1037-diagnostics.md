# WASI 1037 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":2,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T11:15:24.435Z","finishedAt":"2026-09-09T11:17:39.036Z","packages":160}

## unused_qualifications — unnecessary qualification

error: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs:37:12
   |
37 | ...   if std::mem::size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || std::mem::size_of::<M>() > ARTIFACT_EPHEMERA...
   |          ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-qualifications` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
   |
37 -         if std::mem::size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || std::mem::size_of::<M>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES {
37 +         if size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || std::mem::size_of::<M>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES {
   |


## unused_qualifications — unnecessary qualification

error: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs:37:91
   |
37 | ...   if std::mem::size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || std::mem::size_of::<M>() > ARTIFACT_EPHEMERA...
   |                                                                                         ^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
37 -         if std::mem::size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || std::mem::size_of::<M>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES {
37 +         if std::mem::size_of::<P>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES || size_of::<M>() > ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES {
   |


