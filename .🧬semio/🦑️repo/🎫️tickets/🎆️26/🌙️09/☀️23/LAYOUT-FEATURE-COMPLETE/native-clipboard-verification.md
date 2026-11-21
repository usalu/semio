# Native Clipboard Verification

The mounted clipboard is included by `🧊️wgpu/🖱️ui-engine/🦀️.rs` in the production library `semio-renderer-wgpu-ui-engine`, whose owner manifest is `🧊️wgpu/🖱️ui-engine/📦️packages/🦀️rust/Cargo.toml`. Its native implementation is guarded by `all(not(target_arch="wasm32"), not(any(test, feature = "testkit")))`; neither a Wasm check nor compiling the crate as a unit test exercises that code. After applying the drafts, run on the native host with no `testkit` feature:

```sh
repo cargo --store=isolated --configuration=prototype -- check -p semio-renderer-wgpu-ui-engine-focus --lib --locked
```

The focus crate owns the scene and clipboard module. Its complete dependency closure is verified below.

The born-session regression draft adds the unit test `original_unpublished_worker_session_returns_its_cancel_alias_to_live_root` to `🧵️job/👷️worker/🚪️session-admission/🧪️tests/🦀️.rs`. The production package is `semio-framework-job-worker`, manifest `🧵️job/👷️worker/📦️packages/🦀️rust/Cargo.toml`; its admission module includes this file under `cfg(test)`. Run the named test after applying the draft:

```sh
repo cargo --store=isolated --configuration=prototype -- test -p semio-framework-job-worker --lib --locked original_unpublished_worker_session_returns_its_cancel_alias_to_live_root
```

`repo cargo` is the supported ingress and `--store=isolated` chooses its isolated target store; repo agent sessions own Cargo lane identities and still share machine admission/resource gates. A separate agent session can therefore run this native work independently of the browser and guest lanes, subject to capacity and any shared resource leases. Both commands select one real production package and its dependency closure; neither requests a full workspace build. Cold native dependencies can still be substantial. The regression is present only in the unapplied draft, and no compilation or tests were run during this audit.

## Dependency and Draft Evidence

Relative to the UI engine Cargo manifest directory, the worker package is `../../../../../../../../../../🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust`; the worker authority package is `../../../../../../../../../../🔨️modules/🧵️job/👷️worker/🎟️authority/📦️packages/🦀️rust`. Resolving `package.workspace` from the engine manifest gives the repository root `Cargo.toml`. Its dependency declarations permit `workspace = true` for async slots and value. The worker and worker-authority packages are resolved by the two relative paths above. Keep these dependencies under the native target condition because the mounted implementation is native-only.

The unapplied ticket drafts are `🗑️generated/native-clipboard-retirement.patch`, `🗑️generated/native-clipboard-close-schema.patch`, and `🗑️generated/unpublished-worker-cancel-alias-law.patch`. The source is mounted by `🧊️wgpu/👆️ui-engine-focus/🦀️.rs`; the production cfg and mounted dependencies are in `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🎬️scene-intents-and-clipboard/🦀️.rs:209–220`. The regression is included by `🧵️job/👷️worker/🚪️session-admission/🦀️.rs:77–79`.

## Confirmed Production Owner

The retained UI engine facade reexports `scene_intents_and_clipboard` from `semio-renderer-wgpu-ui-engine-focus`. The focus crate mounts the source file through an explicit Rust path attribute. The earlier dependency audit inspected the facade manifest, so the facade manifest and lock proposal is superseded. The actual owner already declares the internal worker, worker authority and value crates, plus async slots on both supported platforms. The root lock includes each exact edge. The revised manifest test passed against the live files with Bun TOML and smol-toml. No dependency or lock edits are needed. The focused native production check targets `semio-renderer-wgpu-ui-engine-focus`. The named born-session Rust regression remains staged.
