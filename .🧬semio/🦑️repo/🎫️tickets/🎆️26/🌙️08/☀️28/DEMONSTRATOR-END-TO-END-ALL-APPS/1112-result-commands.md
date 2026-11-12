# 1112 lane commands
- Fixed: runtime-exchange exchange-finish crate path (semio_plugin_runtime_ops); instance-commands staging/actions: ArtifactOwnedContextHandle::admit(original, grant, cx) new signature, dispatch_command_frame bind_context as_deref + ActionMeta expected_document_revision + Arc view_state.
- Refusal protocol: new `ArtifactOwnedContextHandle::admitted(self) -> Result<Self, ValueError>` (decl-app lifted/context) surrenders a refused handle's untouched original and returns the refusal; used in staging/actions and typed-command (one protocol). Handle never drops holding a payload; turn charges nothing extra.
- Native W-commands-2 rc0 (5 crates), W-commands-4 rc0 (+decl-app). Wasm W-commands-3 rc0, W-commands-5 rc0 (+decl-app).
- Not verified: tests (no tst.sh run), ladders/reactor crates.
STATUS: FIXED-verified W-commands-5
