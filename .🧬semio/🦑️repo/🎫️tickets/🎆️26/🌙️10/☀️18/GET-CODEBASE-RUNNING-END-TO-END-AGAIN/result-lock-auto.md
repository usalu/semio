FIXED 21:32 - the lock hook no longer loads a provider: neutral `registerCargoLockEnsurerV1`, the repo lock module registers itself when it loads (static import, inside the captured closure). Verified: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-value --message-format=short` reached cargo, Finished, rc=0 (chk1.raw).

# result-lock-auto (executor lock-auto, wave 1022a) - IMPLEMENTED, live-verified on hub; dag + chk1 handed over

Tools: `C/lock-auto-tools/`, generated: `C/🗑️generated/lock-auto/`.

## Design (implemented, live edits)
- Core (neutral, schema-first): `🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/}`: receipt = sha256 of every manifest of the workspace closure (members, path dependencies, owning workspace roots, .cargo/config) + Cargo.lock; hit = zero cargo processes. Miss -> per-workspace try-lease `cargo-lock:<workspace dir>` (nonblocking poll + EN/DE progress, peers dedupe through the receipt), probe `cargo update --workspace --locked --offline`, refresh `cargo update --workspace --offline` (shared registry), online `update` only when a crate is missing and the build has a network grant, else EN/DE refusal naming the crate.
- Provider method `ensureLockCurrent` on the `cargo-workspace` port; repo adapter `🗂️workspaces/🦀️cargo/🔒️lock/🟦️.ts`.
- Hooks: (a) `📥️dependencies/🟦️.ts` `withRepositoryCargoInputsV1` (every launcher/pass/compiler-input fetch) before the locked fetch + one forced repair on a stale fetch failure; (b) `captureCargoCompilationGraphV1` (every planning: Bazel actions, component/wasm/native drivers) before `cargo metadata --locked` + one forced repair on a stale metadata/unit-graph failure.
- Test: `bun test ./<…>/🔒️lock/🧪️tests/🟦️.ts` 8/8 (real cargo is the third-party oracle).

## NOTE from slice-demo2 (21:25): your hook breaks every `bun repo cargo …` (the main repo process)
`ensureCargoLockCurrentV1` (`🏃️process/🦀️cargo-workspace/🟦️.ts:37`) calls `loadScriptProviderV1` from `captureCargoCompilationGraphV1` (`🕸️units/📥️capture/🟦️.ts:166`) inside the captured main process (`cargo/🚪️launcher/🟦️.ts:405`). There the computed provider import is refused: "Uncaptured implementation import refused: …/🦀️cargo-workspace/🔌️provider/🟦️.ts (imported by …/🔌️providers/🟦️.ts)" (`providers/🟦️.ts` has `roots: []` in ComputedImportSites). Reproduce: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-plugin-runtime-exchange` (my chk12-chk15, 20:32-21:20). Your live runs went through Bazel/child processes (no capture) so they pass. Fix options: literal import of the lock adapter in a file the main-process capture reaches, or a ComputedImportSites row with the provider as root.

## Changes since the first draft
- No provider (captured closures refuse computed imports): neutral `registerCargoLockEnsurerV1` / `ensureCargoLockCurrentV1` in `🦀️cargo-workspace/🟦️.ts`; the repo lock module registers itself at load. Provider method removed again.
- Extra guard just before the final locked spawn: `component-pass/🟦️.ts` (before `spawn` of the pass), `native-build/🟦️.ts` (driver spawn), launcher `🚪️launcher/🟦️.ts` (before the compile execute). Receipt hit = no cargo process.
- Capture wiring test added to `🕸️units/📥️capture/🧪️tests/🟦️.ts` (2/2 pass); lock tests 8/8 pass (real cargo as oracle: stale before / locked-ok after, concurrent = one refresh, receipt hit = 0 cargo calls, unresolvable crate refused EN+DE naming it).

## Live evidence (`C/🗑️generated/lock-auto/`)
- base1: hub lock was stale from peers; `bun repo cargo -- fetch --locked --offline` printed "The Cargo.lock of …/🌎️hub is behind its manifests; refreshing it from the shared registry." then "… is current again.", rc=0. base2 immediately after: receipt hit, no message, rc=0.
- Receipt cost: hub closure 3006 manifests, verify 250-460 ms under load 16.
- live1 (negative control): a dev-dependency edit on the dag-host crate does not stale a workspace that only reaches it by path, lock unchanged, rc=0.
- live2 (dependency edit on `semio-framework-os-infinite-board-dag-host`): hub-edited run refreshed and rc=0 (live2-hub-edited.raw). dag-edited, hub-restored, dag-restored are still running.

## Handover (detached, not killed)
- pid 8710 `sh C/lock-auto-tools/live2.sh live2` removes the temporary dependency line from the dag-host manifest (`edit2.py remove`) after dag-edited; check `git diff` of `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/📦️packages/🦀️rust/Cargo.toml` shows no `semio-framework-tool-run` line when `C/🗑️generated/lock-auto/live2.done` exists. If it dies, run `python3 C/lock-auto-tools/edit2.py remove`.
- chk1 finished rc=0.

## Open
- Pre-existing red, not mine: dependencies consumption test "repository compiler callers…" (`withCompilerWorkspaceSharedWriteV1` not bound in its modeled bindings).
- `repo sync cargo` keeps its own receipt; not unified.
- Bazel end-to-end action not run by me; hooks sit on every locked fetch, planning and spawn path.
