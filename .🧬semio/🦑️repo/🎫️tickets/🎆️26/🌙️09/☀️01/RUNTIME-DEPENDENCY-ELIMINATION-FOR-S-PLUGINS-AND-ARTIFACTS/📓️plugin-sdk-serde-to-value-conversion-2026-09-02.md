# `semio-framework-plugin` (🔌️plugin/🦀️.rs) — serde → ToValue/FromValue, wave N

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` only. Continues the file's dual-derive
and `serde_json` cleanup after `try_serialize` (already done before this pass — see
📓️try-serialize-typed-operation-result-page.md).

**No `cargo` command was run — by explicit instruction (a peer session held the exclusive
build-directory lock).** Verification substitutes used instead:
- A hand-written brace/paren/bracket depth scanner (skips string/char/comment content correctly,
  including raw strings and Rust's nestable block comments) over the whole file: final depth 0/0/0
  after every edit — no unbalanced delimiters introduced.
- `rustfmt --check --edition 2021` against the crate root: it reached and diffed the target file
  (diff headers up to line ~37499, near EOF) with zero parse errors anywhere in the run — only
  formatting-style diffs (this repo doesn't run default rustfmt line width, pre-existing). A parse
  error would have hard-failed rustfmt instead of producing a clean diff, so this is a real (if
  partial) syntax check, not just my own bracket counter.
- Manual argument-parity check for every `#[value(…)]`/`#[serde(…)]` pair before deleting the serde
  half (see table below).
- Manual cross-reference grep for every type I stripped serde from, confirming no other production
  call site still needs `Serialize`/`Deserialize` on it (e.g. via a generic `T: Serialize` helper)
  before touching it — this caught a real blocker (see "Deferred" below).

## Converted: 20 production dual-derive types

All had `#[serde(…)]` args already 1:1 with their `#[value(…)]` twin (confirmed before deleting —
no format-changing arg drift). Removed `serde::Serialize`/`serde::Deserialize` (or bare
`Serialize`/`Deserialize` from a shared `use serde::{Deserialize, Serialize};` that has other live
consumers elsewhere in the file, so the import itself was left in place — see "Import cleanup"):

- **ArtifactInferenceWire region** (~line 1554): `WireArtifactInferenceMetadata`,
  `WireArtifactInferenceRequest`, `WireArtifactInferenceBudget`, `WireArtifactInferenceCacheMode`,
  `WireArtifactInferenceDiagnostic`, `WireArtifactInferenceProvenance`, `WireArtifactInferenceResult`
  (7 types — this is the literal "9 types" the assignment named, minus the 2 below which it also
  meant. The assignment's count came from a `serde::`-qualified-only grep and missed the next 13).
- **TypedOperationResultLane**, **TypedOperationResultToken** (~line 12997) — the other 2 of the
  named "9".
- `WireMutationRosterEntry` (~4433), `WireArtifactMutationPlanRequest`,
  `WireArtifactMutationPlanResult` (~4515-4528).
- `NoConfig`/`NoConfigMutation`, `NoPresence`/`NoPresenceMutation`, `NoTransient`/
  `NoTransientMutation` (~9519-9721) — these are `ArtifactApp::{Config,Presence,Transient}`'s
  defaults; the trait bound itself was already flipped to `ToValue`/`FromValue` in an earlier wave
  (Cargo.toml's own comment says so) so these six derives were pure leftover weight.
- `MediaArtifactDescriptor` (~11488) — outer struct-level serde only; its two foreign-type fields
  (`media_type: Option<MediaType>`, `wire: MediaWireFormat`) keep their existing
  `#[value(serialize_with = …, deserialize_with = …)]` bridge through `media_value_bridge` (that
  bridge's own docstring already explains the orphan-rule reason and was correct, so left as-is).
- `ExtensionManifest` (~31824) — outer struct-level serde only. Its `capabilities`/`dependencies`/
  `contributions`/`execution` fields keep the `foreign_kernel_value_bridge` bridge (still-foreign
  types). Its `capability_requests: Vec<CapabilityRequest>` field's bridge was **removed** (not just
  serde) — `CapabilityRequest` (`🎠️kernel/🦀️.rs:2171`, outside this file) now derives
  `ToValue`/`FromValue` directly upstream, so the field's `#[value(…)]` no longer needs
  `serialize_with`/`deserialize_with` at all. Updated the module's docstring to drop the now-stale
  `CapabilityRequest` mention and note why `capability_requests` differs from its siblings.

## Companion call-site conversions (required — these types are used through raw `serde_json`, not
just derived)

Without these the file would not compile once the types above lost `Serialize`/`Deserialize`:

- `wire_list_artifact_inference_services_with_routes`, `wire_artifact_infer_from`,
  `wire_execution_completes` (the `to_vec`/`from_slice` trio around ArtifactInferenceWire, plus their
  4 test call sites in `artifact_inference_wire_tests`) → `protocol::json::to_json_string(&x)
  .into_bytes()` / `protocol::json::from_json_str(std::str::from_utf8(bytes)?)`. Byte-identical wire
  format (`pack::json` is the same module as `protocol::json`, documented byte-identical to
  `serde_json` for floats — confirmed in the assignment).
- `plugin_consume_media`/`plugin_produce_media`'s `MediaArtifactDescriptor` JSON round-trip → same
  `protocol::json::{from_json_str, to_json_string}` swap; `to_json_string` is infallible so the
  encode side dropped its `.map_err(...)？`.
- The **second `serde_json::to_writer` site** (`admit_command_json_with_proof`, ~line 19049): was
  `let wire_args = args.map(serde_json::Value::from); serde_json::to_writer(&mut writer, &(verb,
  wire_args))…`. Converted to build the JSON text via `protocol::json::to_json_string(&(verb,
  args.cloned()))` then `std::io::Write::write_all(&mut writer, encoded.as_bytes())` — same
  `TypedCommandWireWriter` bound-capacity `Write` impl, same Fault on overflow, same 2-element flat
  array wire shape (`(&str, Option<DslValue>)` is a 2-tuple; ToValue tuple support goes up to 3 per
  the file's own comment near `DownloadResultPayload`). **Known behavioral difference, flagged, not
  silently accepted**: the old code streamed into the bounded writer incrementally and could fail
  fast without ever building a full buffer for an oversized payload; the new code builds the whole
  `String` first, then `write_all`s it — so a pathological huge `args` now allocates fully before the
  capacity check trips, where before it didn't. I could not find a streaming/incremental
  `T: ToValue` writer in `pack::json` (only `to_json_string`/`from_json_str` exist there, and I was
  told not to add to that file). Flagging this explicitly rather than passing it off as
  equivalent — someone with authority to touch `pack/🔤️json/🦀️.rs` should decide if a streaming
  variant is worth adding.
  - Updated the file's own self-referential structural test
    (`fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers`,
    ~17859) which did `admission.find("serde_json::to_writer").expect("incremental wire encoder")` —
    it now searches for `"protocol::json::to_json_string(&(verb, wire_args))"` /
    `.expect("bounded wire encoder")`. This is a structural/shape assertion on the file's own source
    text, not a serde-vs-first-party oracle test, so updating it to track the real implementation is
    normal maintenance, not "deleting a test to reduce a count."
- The **sibling** `args.map(serde_json::Value::from)` / `serde_json::to_vec(&(action, wire_args))`
  in `dispatch_framework_reserved_action` (~21606) → same `protocol::json::to_json_string(...)
  .into_bytes()` swap, infallible so `.map_err(...)?` dropped too. (No bounded-writer capacity
  concern here — plain `Vec<u8>` builder, so no behavioral-difference flag needed.)
- `submit_owned_media_export`'s `serde_json::to_vec(&(tool_id.as_str(), port))` (~21336) → same
  `to_json_string(...).into_bytes()`.
- The clipboard byte-budget accumulator `bytes.saturating_add(serde_json::to_vec(effect)…len())`
  (~21786, `effect: &Effect`) → `protocol::json::to_json_string(effect).len()`. `Effect` already
  derives `ToValue`/`FromValue` upstream (`🎠️kernel/🦀️.rs:302`) — confirmed before converting.
- Two `serde_json::json!({ "readOnly": true }).to_string()` sites (~25053, ~25731) → the literal
  `"{\"readOnly\":true}".to_string()`. It's a fixed 1-field literal, so hardcoding the exact
  `serde_json` compact-object output (no spaces, matches `to_string()`'s format) is simpler and
  zero-risk versus routing through any JSON API at all.
- Two **bridge sites that were routing an already-`FromValue`-capable type through
  `serde_json::Value` "to satisfy the compiler"** (the exact trap named in the assignment), now using
  `DslValue`/`FromValue` directly instead:
  - `history_command_authors`'s `serde_json::from_value::<Vec<vcs::Author>>(serde_json::Value::from(value))`
    — **found already fixed by a concurrent peer session** when I re-read the file (this repo has
    multiple agents live-editing this same file per the ticket; I did not touch this site, just
    confirming it's resolved).
  - `framework_reserved_work_items`'s `"paste"` arm:
    `serde_json::from_value::<ClipboardFragment>(serde_json::Value::from(value)).ok()` (~21597) →
    `<ClipboardFragment as protocol::FromValue>::from_value(value.clone()).ok()`.
    `ClipboardFragment` derives `ToValue`/`FromValue` directly (`🎠️kernel/🦀️.rs:241`) — confirmed
    before converting. Used the fully-qualified `<T as Trait>::method` form rather than
    `ClipboardFragment::from_value(...)` since only the `semio_framework_value_derive` **macros**
    (not the `protocol::FromValue` **trait**) are `use`d in this scope; fully-qualifying sidesteps
    any doubt about trait-method resolution without adding an import to a shared scope.

## Deferred — investigated, NOT converted, with the specific reason

Listing these because the assignment requires listing every uncertain site, not because they're
equally close to done — several are genuinely blocked, not just unexamined.

1. **`owned_abi` module (~line 114-176): `PollInput`, `StartJobInput`, `StepJobInput`,
   `CancelJobInput`, `RestoreInput`, `JobStep`.** This is the actual WASM-guest ABI boundary
   (`take_json<T: serde::de::DeserializeOwned>` / `return_json<T: Serialize>`, raw pointer+length
   memory functions called from `#[unsafe(no_mangle)] extern "C"` exports). `return_json` is called
   with `Result<(), Vec<u8>>` and `Result<JobStep, Vec<u8>>` — `std::result::Result<T, E>` has a
   blanket `serde::Serialize`/`Deserialize` impl but **no blanket `protocol::ToValue`/`FromValue`
   impl exists** (checked `🌱️value/` — not present). Converting `take_json`/`return_json` to
   `T: FromValue`/`T: ToValue` would need that impl added to the value-derive/codec crate, which is
   outside this file. Until that lands, these 6 types **must** keep their serde derives or the ABI
   entry points stop compiling — this is a real architectural blocker, not an oversight.
   `PollInput`'s own hand-written `protocol::ToValue`/`FromValue` impl (bridging via `serde_json`
   because its fields are foreign `semio_framework::kernel::*` types with no `ToValue` of their own)
   is correctly documented as necessary and was left untouched for the same reason.
2. **`WorldSunConfig`, `WorldProjectionConfig`** (`world3d_host` module, ~36266-36420+). Both are
   clean dual-derives I could strip mechanically, but `world3d_environment_json` interpolates
   `WorldSunConfig` into a `serde_json::json!` call, and the surrounding module threads
   `serde_json::Value` as its general dynamic-JSON currency through many function signatures
   (`apply_world3d_sun_action`, `world3d_sun_measures`'s `action: impl Fn(&str, Option<Value>) -> …`,
   etc.) that also cross into `ui_wgpu::wgpu::ActionDescriptor`/`WindowMeasure`/`MeasureSelectItem` —
   types owned by a different crate I didn't have time to fully audit for whether they themselves
   require `serde_json::Value` specifically. Converting just the two structs without following every
   downstream `Value` call site risked a partial, wrong migration I couldn't verify by compiling.
   Left alone; flagging as the next well-scoped sub-task.
3. **`PastePlacement`** bridge (`serde_json::from_value::<PastePlacement>(args)`, ~line shifted near
   32750 after other agents' edits) — genuine foreign-type bridge, `PastePlacement`
   (`🎠️kernel/🦀️.rs:227`) only derives `Serialize`/`Deserialize` upstream, no `ToValue`. Same
   category as the `foreign_kernel_value_bridge`/`media_value_bridge` bridges I kept — correctly
   un-fixable from inside this file alone.
4. A **second, test-only** `ClipboardFragment` bridge inside `TestClipboardReservedJob` (a
   `#[cfg(test)]` fixture, ~32700s) still routes through `serde_json::Value` even though
   `ClipboardFragment` itself is fixed. Low-priority — it's test scaffolding, not production, and its
   own local `args` type wasn't investigated closely enough to convert with confidence in the time
   available.
5. **Not investigated at all** (found via grep, out of time budget for this pass — each needs the
   same "trace every call site + check upstream type derives" treatment given to the ones above
   before touching):
   - `InteractionState` cluster (`serde_json::to_string`/`from_str`/`to_vec`/`from_slice`, ~4 sites
     near where `set-interaction-state`/`interaction-config-mutation` live).
   - The `ManifestActionInvocation`/`ManifestCommandInvocation`/`ViewModel`/`WindowRenderInput`/
     `RefreshRequest`/`ContextMenuWireRequest`/`ContextMenuResponse` cluster (~29880-30360) — a large
     block of host-boundary JSON-string glue functions.
   - `relay_opening_command`'s three `serde_json::json!({…})` call sites (~30400s) — likely
     convertible to direct `DslValue` construction, not investigated.
   - `serde_json::to_value`/`to_string`/`from_str` around descriptor/fingerprint handling (~31300s).
   - `interaction_target_args`'s `serde_json::to_string(&vec![protocol::InteractionTarget{…}])`
     (~34200s) — `InteractionTarget` may already derive `ToValue`, not checked.
   - The `world3d` mesh helper functions (~36700s+) sharing `world3d_host`'s deferred `Value` plumbing
     (see #2).
   - `pub use serde_json::Value;` (~37200 pre-edit) — a **public re-export**. If any other crate in
     the tree names this path, removing it is a breaking API change beyond this file; not
     investigated.
   - `foreign_kernel_value_bridge`'s and `media_value_bridge`'s own generic helper bodies
     (`to_value<T: serde::Serialize>`/`from_value<T: DeserializeOwned>`) are intentionally kept — they
     exist specifically because their target types are foreign and don't implement `ToValue` upstream
     (documented in both modules' docstrings, one of which I updated for accuracy — see above). These
     are not bugs to fix from inside this file.

## Import cleanup

Checked but did **not** remove `use serde::{Deserialize, Serialize};` anywhere: every occurrence in
this file (7 total, e.g. line ~116/owned_abi, ~287) sits at the top of a module-or-larger scope that
still has other live serde consumers after this pass (owned_abi's `PollInput` etc., the deferred
`world3d_host` types, deferred test fixtures). None of the types I converted had their own
narrower/local `use serde::{…}` to clean up — they all drew on these same wider-scope imports, which
remain necessary.

## Net effect

`git diff --stat` for this file: 85 insertions / 107 deletions (my edits only — the working tree also
carries concurrent, unrelated changes from other live sessions per this ticket's normal
multi-agent workflow; not summarized here). Remaining `serde_json::` call sites in the file after
this pass: ~130 (down from ~150+ before, most of the reduction from the ArtifactInferenceWire/
MediaArtifactDescriptor call-site conversions above, since simple derive-stripping alone doesn't
touch `serde_json::` call counts). Remaining bare `#[derive(…Serialize…Deserialize…)]` lines: 18
(mostly the deferred owned_abi/world3d_host items above, plus legitimate test-only fixtures this
ticket's own convention says to leave alone).
