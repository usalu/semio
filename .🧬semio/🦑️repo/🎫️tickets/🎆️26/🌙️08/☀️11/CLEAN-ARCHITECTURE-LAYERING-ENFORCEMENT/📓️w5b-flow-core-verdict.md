# Wave 5b — flow-core relocation: verdict (orchestrator)

## Extension-world prototype: verified, real bugs fixed
Confirmed directly: `cargo check -p semio-framework-plugin -p
semio-framework-plugin-host` both clean, `extension_runtime_constructs_engine_and_linker`
test still passes. The 3 fixes (fuel budget on ExtensionRuntime calls,
epoch deadline never set, guest-side extension init not installing the real
bundle) are real, targeted, non-regressing. The WIT extension-world
mechanism is now proven end-to-end with a real compiled component, not
just type-checked.

`WasmPluginRuntime`'s identical missing-epoch-deadline gap (flagged by the
prototype agent as out-of-scope, spawned as a background task) is a
plugin-world concern, not extension-world — outside this ticket's
Contribution/layering scope. Left for that background task to handle.

## Flow-core relocation: ABANDONED — original plan over-corrected

Investigation found the wgpu renderer (a genuinely framework-owned crate,
`role = "framework"`) has a real, non-generic dependency on `os-flow`'s
`FlowHost`/`FlowFixture`/DAG types. Moving `os-flow` into the flow plugin
would create a NEW `framework → plugin` edge — exactly the class of
violation this ticket exists to eliminate, not fix one.

Re-examining the actual rule set (framework must not know ✏️s or plugins;
✏️s must not know plugins; plugins must not know extensions):
**`os-flow` living in the framework tree, with multiple PLUGINS depending
on it, is not a violation at all.** Plugins consuming a framework-owned
capability is the correct, intended direction — that's what a framework
module is *for*. The original design decision (plan §6) called for
relocating flow-core into the plugin based on an assumption that turned out
wrong once real consumer dependencies were mapped; `.dependency-cruiser.cjs`
already independently reflects this (its `crossPluginRules` explicitly
exempts `flow` as a "media-graph canvas embed" — pre-sanctioned, not a
lint violation).

**Verdict: no relocation. `os-flow` stays exactly where it is.** This
mirrors and confirms Wave 4b's `s.*`→`os.*` treatment of `space`/`workflow`/
`store` (rename in place, don't relocate) — `flow` gets the same treatment,
which Wave 4b's naming sweep already covered (no `s.`-prefixed literals
found in the workflow file beyond what was already fixed). No further
action needed on flow-core placement.

## Next: C2 unlink can now proceed
With the extension-world mechanism proven, `🌀️procedural`'s 7 direct Cargo
dependencies on flow extensions (the one confirmed real runtime
plugin→extension violation, audit finding C2) can be replaced with real
`extension_invoke` calls through the now-verified `ExtensionRuntime`.
