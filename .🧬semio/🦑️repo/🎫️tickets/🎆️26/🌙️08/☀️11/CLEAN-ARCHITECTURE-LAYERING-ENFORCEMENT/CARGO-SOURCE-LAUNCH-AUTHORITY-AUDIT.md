# Cargo Source Launch Authority Audit

Read-only lexical review. No launch, wrapper, import, factory or native process executed or modified. Wrapper snapshot SHA-256: `347d77a6aa21131ca7711e98fbb8369c5905a110a24c65d18b43df5e18748112`.

## Existing Launch Rows

All six rows are in `.vscode/launch.json`, type `node-terminal`, request `launch`, cwd `${workspaceFolder}`, without row-local env. They invoke Bun on the other ticket `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/📜️script.ts` with the following arguments. No matching six rows were found in `.vscode/🧩️launch.seed.jsonc`.

| Name | Name / Command Lines | Wrapper Arguments |
|---|---|---|
| Repo Cache Exact Cargo Storage (Prototype) | 5664 / 5666 | `exact-storage prototype` |
| Repo Cache Repository Exact Cargo (Prototype) | 5671 / 5673 | `repository-exact prototype` |
| 🦑️ Repo • ⚡️ Cache • 🦀️ Rust Unit Artifacts • Prototype | 5678 / 5680 | `rust-units prototype` |
| Repo Cache Exact Cargo Storage (Release) | 5699 / 5701 | `exact-storage release` |
| Repo Cache Repository Exact Cargo (Release) | 5706 / 5708 | `repository-exact release` |
| 🦑️ Repo • ⚡️ Cache • 🦀️ Rust Unit Artifacts • Release | 5713 / 5715 | `rust-units release` |

No direct literal `exact-cargo-storage` launch was found; `exact-storage` is its existing indirect launch.

## Original Initial Authority

Wrapper line 20 defines policy `{version:1,owner:"repo:private-cache-headroom",maximumElapsedMilliseconds:600000}`. Line 110 creates the initial envelope using that policy and these capabilities:

- `workspaceRoot`: actual resolved workspace.
- `buildMode`: selected prototype/release.
- `arguments` and `selection`: `['nx','run','repo-private-cache:'+command,'--configuration='+mode]`.
- `native.artifactDirectory`: `cargoTargetDirectory(workspace)`.
- `native.transport`: maximumBytes 134217728, maximumLines 65536.
- `native.child.maximumElapsedMilliseconds`: 600000.
- `native.network.offline`: false.

None of these three commands is in `storageCommands`; therefore the conditional `tool` and `launches` additions do not apply. These are initial wrapper-issued orchestration grants, not a selected native-owner backend descriptor or a reusable grant for arbitrary replacement commands. A replacement Source launch must use the existing original build authoring/issuer path for its actual new selection, preserving the authored resource bounds; copying old raw capabilities would misrepresent selection.

## Derived Phases and Environments

The initial wrapper holds actual Repository tooling Store and admitted immutable Nx SDK use (lines 111–121). Its outer `cache:headroom:nx` compiler control is bounded at 600000. It runs the actual SDK Nx CLI `run repo-private-cache:<command> --configuration=<mode>` under captured output 134217728 bytes / 65536 lines. Its environment retains original invocation plus SDK identity/root, mode, `NX_DAEMON=false`, SDK `NODE_PATH`, private workspace path and ticket-owned per-command/mode Nx data/cache paths.

The other ticket `headroom/project.json` routes these through `nx:run-commands`, cache false, outputs empty, forwardAllArgs false: rust-units lines 1155–1170; exact-storage lines 1462–1477; repository-exact lines 1527–1542. Each calls the same wrapper with `execute <command> <mode>`.

Inner wrapper line 26 derives `nx:isolated:<command>` control at **600000 for repository-exact**, **300000 for rust-units**, and **300000 for exact-storage**. This distinction follows the explicit `nativeEnvelope` membership at line 18; it must not be flattened into the outer 600000 grant. Inner line 108 propagates original invocation with mode, original preparation storage descriptor, SDK identity/root, workspace variables, and `SEMIO_TEST_ARTIFACT_DIR` pointing to the other ticket's `🗑️generated` directory for all three commands. The launch rows themselves do not author these environment values.

Routes in wrapper line 14 / owner selection line 26:

| Wrapper Command | Final Registered Owner Call | Current Source Target |
|---|---|---|
| repository-exact | Repo library `⚡️caching/📜️script.ts test repository-exact`, cwd workspace | `repo:test-repository-exact-cargo`, caching project line 697 |
| rust-units | Repo library `⚡️caching/📜️script.ts test rust-units`, cwd workspace | `repo:test-rust-units`, caching project line 678 |
| exact-storage | General Process `📜️script.ts test exact-cargo-storage`, cwd Process owner | `@semio-tech/framework-process:test-exact-cargo-storage`, Process project line 454 |

Existing final registered owner bounds also remain material: caching `repository-exact` derives 120000; `rust-units` derives 600000 and is still capped by the inner 300000 and original remaining time. Process `exact-cargo-storage` uses the existing fundamental suite bound 15000 for its type/test children. No budget increase is proposed. Root/HighScope own replacing ticket indirection in launch authoring while keeping original-versus-derived authority explicit.
