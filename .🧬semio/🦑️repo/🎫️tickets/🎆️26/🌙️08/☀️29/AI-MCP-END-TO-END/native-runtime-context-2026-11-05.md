# Preserved Runtime Context

Long source narratives retained here while concise native docstrings stay beside their definitions. No assertions, code or policies were removed.

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/⏯️operation-begin-resume-and-cancel/🦀️.rs

🩺️ A guest fault, spoken in the guest's own words. `unreachable executed` from a
`panic = "abort"` component IS a Rust panic, and the panic message it printed a moment earlier is
the only thing that names the failing frame — so the retained tail of guest stderr travels with
the trap instead of being thrown away with the instance.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📣️host-replies-wasi-and-channel-faults/🦀️.rs

🩺️ Retains what the owned guest writes to `wasi:cli/stdout`/`stderr`, bounded by
[`GUEST_DIAGNOSTICS_CAPACITY_BYTES`]. The owned shim used to answer `output-stream.write` with a
zeroed result and DISCARD the bytes, so a guest panic — which `panic = "abort"` prints to stderr
immediately before its `unreachable` — reached the host as the bare text `wasm trap: unreachable
executed` and nothing else. That is what made ticket 26/09/18 slice TC3c's `codec.genesis` trap
undiagnosable from the outside: the guest had already said exactly what was wrong. The wasmtime
path has kept this since [`guest_wasi_ctx`]; this is the owned interpreter's half of it.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📥️snapshot-operation-and-input-records/🦀️.rs

🔀️ `serde`-only, not `ToValue`: `events`/`command_page`/`budget` embed `semio_framework::kernel::
{Event, CommandPageCursor, FixedCommandPage, Budget}`, none of which have `ToValue`/`FromValue`
yet (same orphan-rule/`rename_all_fields`-unsupported blocker as `owned_abi::PollInput`'s own
hand-written bridge in `🔌️plugin/🦀️.rs` — see that impl's docstring). Converting this
needs the `🎠️kernel` module's own conversion pass first.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📥️snapshot-operation-and-input-records/🦀️.rs

🪪️ The owned engine's semantic identity: a digest of the interpreter's source and of this owned
host adapter's source — every instruction a guest executes and every host import answer it can
observe. A fact one build of the engine derived from a component (a verified pack-schema hash)
holds for every other build of the same engine — a copied, re-signed or cross-compiled binary —
and a changed engine never reuses one.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs

🏗️ Admits a fresh instance's channel ([`admit_owned_channel`]: the codec decodes channel-versioned `.spr`, envelope
and native bytes), then runs `codec.pack-schema-hash` for [`OWNED_CODEC_ORIGIN_SCHEMA`] on it — every owned codec
export first assembles the plugin bundle — and keeps the instance it completed on. Its answer, a refusal, is read only
to know the export completed as the ABI says.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs

🌱️ Runs one of the component's pure `codec` functions on a throwaway instance. This is
the host half of ticket 26/09/18 slice TC3b: a headless server resolves a document kind's
schema fingerprint, its canonical empty document, its pair-validation mirror and its edit
apply from the component itself, so a package whose Rust codec the server does not link is
still fully creatable and editable. The instance is a copy of the compiled guest's codec
origin ([`OwnedCodecOrigin`]), created and dropped per call — nothing here observes or mutates
live actor state, and no call sees what another did. `budget.fuel` bounds the call including
the origin assembly it may run first; `progress` reports both as one count.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs

🧬️ `codec.pack-schema-hash` with the same original deadline and fuel observations as [`Self::codec_genesis_observed`].
Catalog startup verifies every unlinked package through this call under
`OperationContext::stall_bounded`; without guest fuel reaching that context a 30 s quiet span
on a loaded machine looks like a wedged load (`trusted-catalog-load-stalled-before-it-finished`).


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs

📈️ `codec.genesis` with bounded fuel-progress observations, for a caller whose own bound is a
STALL bound. A hub creates a document on a detached task under
`OperationContext::stall_bounded`, and the guest call is the single longest thing that task
does — without the guest's fuel progress reaching it, the hub cannot tell an interpreter that
is stepping from one that is wedged, and has to pick between refusing honest work and never
refusing anything. The observations are the same ones `describe_observed` reports: one per
25 M fuel or per 5 s, whichever comes first.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧪️tests/🔬️owned-runtime/🦀️.rs

🤝️ LAW (audit F2/F7): both owned host paths admit a guest's app channel before serving it anything. An actor
instantiation and a codec call (the origin a hub Check In fold, the SQLite export/import and the wgpu document codec run
on) refuse an older or newer guest with `plugin.channel-mismatch` naming both versions — before any other export ran,
since every other export of the guest traps — while a guest of the host's own channel is admitted and reaches its
first frame.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧮️owned-constants-and-codec-origin/🦀️.rs

🧊️ A compiled guest's codec origin: its owned instance right after a first codec export assembled the plugin bundle
(`__semio_ensure_plugin_runtime`: every artifact declared, every mutation roster, the built bundle) and completed.
Measured on catalog B3 (H12, 2026-09-27): a fresh instance spends 521.5 M of a puzzle codec call's 524.7 M fuel on that
assembly and the operation itself ≈ 3.2 M, and every codec call used to pay it on a throwaway instance. Each codec call
now starts from a copy of this state, so every call still sees exactly the same guest — the assembled bundle and
nothing another call did — and answers byte for byte what a fresh instance answers.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧮️owned-constants-and-codec-origin/🦀️.rs

🧊️ The codec origin of one compiled guest, assembled once, by the first codec call that needs it. A call that arrives
while another assembles waits for that assembly instead of running a second one, relays its fuel as its own progress —
a stall bound watching the waiter sees the assembly move, and sees it stop — and observes its own cancellation. An
assembly that faults, is cancelled or panics leaves the origin absent, and the next call assembles it itself.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🛰️owned-operation-and-guest-instance/🦀️.rs

☠️ Set by the first guest trap. A trapped guest keeps whatever linear memory, shadow-stack
pointer and allocator state the trap left behind, so every later call starts from a lower
stack pointer and traps again 112 bytes further down — ticket 26/09/18 slice A1 read that
drift as the defect itself. There is no recovery short of re-instantiation, so this refuses.


### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🛰️owned-operation-and-guest-instance/🦀️.rs

🩺️ Everything this guest has written to `wasi:cli/stdout`/`stderr`, bounded by
[`GUEST_DIAGNOSTICS_CAPACITY_BYTES`] — see [`retain_owned_guest_diagnostics`]. Unlike the
wasmtime path this is ALWAYS armed: the owned interpreter serves the stream shim itself at
no cost, and a guest panic message that is thrown away is a whole investigation.

