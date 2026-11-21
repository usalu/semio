# Raw Picker Reactor Publication

The PDF picker retains its authored `RAW_PICKER_ENABLED = false` gate in `📂️picker-work.rs`. The gated branch creates a typed issue marker and emits a `RequestFileOpen` effect. The typed-operation unit transfers the emitted effect into the bounded `typed_effect_outbox`. The runtime then calls `take_typed_operation_effect` and serializes the effect directly into `PluginExchangeOutput.effects`. The ordinary `OriginalCommandExchangeState.pending_effects` owner serves app background effects.

`raw-media/📤️picker.rs` accepts the effect by mutable reference. It checks the producing instance, exact live allocation and mounted document revision, then uses the real native slot to admit the typed issue before replacing the internal marker. It reserves the effect projection's work and retention before attempting admission. Missing picker publication authority yields a typed unsupported-owner refusal.

The reactor turn routes raw controls and pages before generic decoding and drives the mounted format against the native command Source. The stale outbox-based picker call was removed. Actual PDF activation remains unverified. Activation requires paid custody of the original effect through publication, yield, and retirement. After the current pop, the typed value has no retained reactor owner before serialization. The final turn-output collector also has no effect return path when output preparation fails. The active PDF picker uses the ordinary effect path.

Source inspection confirmed the producer, native admission, raw ingress and output paths. The main ticket owner is running the pinned native and WASI build verification.
