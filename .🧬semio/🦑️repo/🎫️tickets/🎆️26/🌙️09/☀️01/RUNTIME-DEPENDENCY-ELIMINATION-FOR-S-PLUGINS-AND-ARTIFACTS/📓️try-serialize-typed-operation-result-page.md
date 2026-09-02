# `TypedOperationResultPage::try_serialize` seam — re-bound to `ToValue`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (crate `semio-framework-plugin`,
mounted via `📦️packages/🦀️rust/🦀️.rs`'s `extern crate semio_framework_os_kernel as protocol;`).

Flagged by `📓️queued-pack-bridge-wave.md:29` and `📓️store-file-serde-wave-2026-09-02.md:31-33`
as one of the ten framework seams still bound on `serde::Serialize`.

## Change

- `try_serialize<T: serde::Serialize>` → `try_serialize<T: protocol::ToValue>`.
- Body now calls `protocol::json::to_json_string(value)` (`pack::json`, re-exported at the
  os-kernel crate root as `json` via `os_pack`'s `pub use pack::json;` + the crate root's
  `pub use crate::os_pack::*;`) instead of `serde_json::to_writer` into a custom
  `std::io::Write` adapter.
- The custom `TypedOperationResultPageWriter` struct + its `impl std::io::Write` (its only
  caller) was deleted as dead code rather than left dangling.
- Capacity check preserved exactly: `to_json_string` returns an owned `String`; if its byte
  length exceeds `TYPED_OPERATION_RESULT_PAGE_BYTES` the function returns the same
  `Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), …)`
  it always did (same code as `try_new`'s sibling check), instead of truncating or panicking.
  Otherwise the string's bytes are copied into the fixed page buffer exactly as `try_new` does.

## Callers fixed (5 total, all in this file)

1. `TypedOperationResultPage::try_serialize(token, pending_lane, &receipt)` — `receipt` is
   `store::LaneItemReceipt`, which **already** derives `ToValue, FromValue` alongside serde
   (🏪️store/🦀️.rs `LaneItemReceipt`, comment there anticipates this exact seam). No change
   needed beyond the bound switch.
2. `&emit.ui_scope` — `semio_framework::kernel::UiDirtyScope` **already** derives
   `ToValue, FromValue` (🎠️kernel/🦀️.rs). No change needed.
3 & 4. `&("accepted", self.typed_effect_outbox.len())` / `&("accepted",
   self.typed_event_outbox.len())` — `(&str, usize)` 2-tuples; `ToValue` has a built-in 2-tuple
   impl (🌱️value/🔁️codec/🦀️.rs). No change needed.
5. The Download-lane call: was `&(download.filename.as_str(), download.mime_type.as_str(),
   download.encoding.as_deref(), download.chunks.bytes())` — a 4-tuple
   `(&str, &str, Option<&str>, usize)`. `ToValue`'s tuple impls stop at 3 elements (no shared
   4-tuple impl exists), and adding one would mean editing
   `🌱️value/🔁️codec/🦀️.rs`, outside this ticket's stated scope
   (`🔌️plugin/🦀️.rs` plus type definitions needing derives). Instead added a small
   **local, hand-written** type in `🔌️plugin/🦀️.rs` right above `impl TypedOperationResultPage`:

   ```rust
   struct DownloadResultPayload<'a> {
       filename: &'a str,
       mime_type: &'a str,
       encoding: Option<&'a str>,
       bytes: usize,
   }
   impl protocol::ToValue for DownloadResultPayload<'_> {
       fn to_value(&self) -> protocol::DslValue {
           protocol::DslValue::Array(vec![
               protocol::ToValue::to_value(&self.filename),
               protocol::ToValue::to_value(&self.mime_type),
               protocol::ToValue::to_value(&self.encoding),
               protocol::ToValue::to_value(&self.bytes),
           ])
       }
   }
   ```

   Hand-written (not `#[derive(ToValue)]`) because the derive only ever emits a
   `DslValue::Object` for a named-field struct (checked `🌱️value/✨️derive/🦀️.rs`'s
   `expand_to_value`) — the wire shape needed here is the same flat 4-element JSON array
   `serde_json` produced for the old tuple, not an object. The call site now builds
   `DownloadResultPayload { .. }` instead of the raw tuple literal.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` only. No edits to `🏪️store/🦀️.rs`,
  no edits under `✏️s/🔌️plugins/`, no edits to `🌱️value/🔁️codec/🦀️.rs` or any `Cargo.toml`.

## Verification

Two foreground `cargo check -p semio-framework-os-kernel` runs were attempted, each given ~10
minutes; both produced zero output and were moved to background by the harness's own timeout
(the second was later reported `killed`). `ps aux` while the second was pending showed roughly
40 concurrent `cargo check` invocations already in flight system-wide (other agents' sessions:
`semio-s-plugin-*`, `semio-framework-os-flow`, a `--workspace --keep-going` run alive since
7:14AM at ~0% CPU, etc.) plus one long-idle `cargo check -p semio-framework-os-kernel` from a
7:38AM session — genuine, heavy build-lock contention, not a hang caused by this edit. Per the
task's own instruction ("if verification is genuinely blocked by contention, finish the edit and
say plainly that verification was blocked"): **verification did not complete in this session.**
The edit itself was checked by hand against the actual trait/derive surface (see above) rather
than by a passing compiler run — do not treat this as a passing-check claim.


## Verification, final status

The second `cargo check -p semio-framework-os-kernel` attempt terminated (exit 144) with only:

    Blocking waiting for file lock on build directory

confirming this was pure build-directory lock contention (≈40 concurrent `cargo check`
invocations observed via `ps aux` at the time, across many other agents' sessions), not a hang
introduced by this edit. Two good-faith foreground attempts (~10 min and ~15 min) were made per
the task's "be patient, do not kill and retry" instruction; neither was killed by me — the first
was moved to background by the harness's own timeout and later reported `killed`, the second
exited on its own with 144 while still blocked on the lock. Stopping further retries here rather
than looping, per the "do not retry failing commands in a sleep loop" rule.

**Net result: this edit has not been compiler-verified in this session.** It should be re-checked
(`cargo check -p semio-framework-plugin`, the crate that actually contains the diff — see
"Verification" above for why `-os-kernel` is not the right target) once build contention clears.
