# 🌉️ Wave 4 Result: `semio-framework-os-mcp` (22 producers plus a thin facade)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool (left to the coordinator).
Brief and rules: `📓️WAVE4-BRIEF.md` (including the later "god objects" rule), `RESULT-OS-HOST-MCP-WORKSPACE.md`, `PRODUCER-CRATE-RECIPE.md`.
`M` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`. Every crate node is `M/<node>/📦️packages/🦀️rust` with a `🦀️.rs` lib root next to the node.
Tools and logs: `osmcp-tools/` (`plan3.py` is the decision file; `apply.py`, `emit.py`, `oracle2.py`, `verify_registry.ts`, `defscan2.py`, `orphan.py`, `traitcheck.py`, `depcheck.py`, `census_stage.ts`) and `🗑️generated/osmcp-w4-*.log`.

## 0. Verdict

The 51-file, 29,297-line crate is now 22 producer crates plus a 3-file facade. Every lib closure is inside the target (at most 3500 lines, 28 files, 230,000 bytes). All embedded assets stay inside their crate and inside budget. No real `cargo check` or `cargo test` compiled any of it: the launcher now gets as far as the source gate and refuses `semio-framework-os-kernel` (peer-owned, still oversized; section 6.1). Everything else is static evidence and a union-sandbox rustc oracle, with the limits stated in section 6.

## 1. Census

Counter: `readRustSourceClosureV1` through `🗑️generated/census.ts` (`ONLY=<all 23 names>`; raw output `🗑️generated/osmcp-w4-census-after2.log`).

Before: `semio-framework-os-mcp` 51 files / 29,297 active lines / 1,822,586 B (over on lines, files and bytes).

| Crate (`semio-framework-os-mcp-…`) | Node | Lib files | Lines | Bytes | Embedded |
| --- | --- | --- | --- | --- | --- |
| (facade `semio-framework-os-mcp`) | `📦️packages` | 3 | 1,344 | 85,442 | 0 |
| `foundation` | `🧱️foundation` | 12 | 1,570 | 75,366 | 0 |
| `wire` | `📡️wire` | 5 | 2,003 | 93,028 | 0 |
| `schema` | `📐️schema-contract` | 5 | 1,294 | 165,236 | 3 files, 99,096 B |
| `catalog` | `📚️catalog-compiler` | 8 | 2,130 | 123,871 | 1 file, 17,757 B |
| `capabilities` | `🧰️capabilities` | 4 | 633 | 37,758 | 0 |
| `port` | `🔌️channel-port` | 3 | 981 | 62,662 | 0 |
| `bridge-codec` | `🧶️bridge-codec` | 2 | 1,755 | 79,741 | 0 |
| `bridge` | `🪢️bridge-session` | 3 | 1,298 | 61,604 | 0 |
| `consent` | `🙋️consent` | 3 | 520 | 27,199 | 0 |
| `probe-document` | `🔬️probe-document` | 6 | 688 | 59,430 | 1 file, 966 B |
| `plugin-runtime` | `⚙️plugin-runtime` | 6 | 2,811 | 187,289 | 0 |
| `inference-hub` | `🌐️inference-hub` | 3 | 720 | 38,429 | 0 |
| `hub-pair` | `🔗️hub-pair` | 3 | 962 | 45,183 | 0 |
| `hub-remote` | `🌍️hub-remote` | 3 | 2,145 | 127,911 | 0 |
| `shell-channel` | `🎛️shell-channel` | 3 | 609 | 33,769 | 0 |
| `routing` | `🧲️channel-routing` | 3 | 1,132 | 68,717 | 0 |
| `dispatch` | `🪝️action-dispatch` | 2 | 781 | 49,109 | 0 |
| `workspace` | `🪟️workspace-host` | 5 | 1,704 | 119,598 | 0 |
| `tools-inference` | `🔭️inference-tools` | 3 | 1,576 | 100,401 | 0 |
| `tools-gateway` | `🧳️gateway-tools` | 5 | 1,358 | 88,870 | 0 |
| `server` | `🎯️server` | 3 | 574 | 30,724 | 0 |
| `http-transport` | `📮️http-transport` | 2 | 1,862 | 91,498 | 0 |

All 23 rows report `overLimit: false`. The binding dimension is lines for most, bytes for `schema`, `plugin-runtime`, `probe-document` and `workspace`. `closureCompilable` is false for all of them because the oversized peer crate `semio-framework-os-kernel` (and four more oversized deps) sits in their closure.

Test closures (cfg `test`, feature `testkit`, from `osmcp-tools/census_stage.ts` on the staged tree): every crate is below 3500 lines except the facade (3,657 lines / 240,629 B, which also exceeds the 230,000 B target but not the 4000 / 262,144 hard limits) and `hub-remote` (3,570 lines). `plugin-runtime` (3,062 / 210,811 B) and `http-transport` (3,322) are close. Both are inside the hard limits.

## 2. Crate graph (downward DAG, `[dependencies]` only)

Tier by dependency depth, lowest first. `fnd` = foundation.

| Crate | Depends on (os-mcp crates) |
| --- | --- |
| `foundation`, `hub-pair`, `probe-document` | none |
| `wire`, `inference-hub`, `bridge-codec` | `fnd` |
| `schema` | `fnd`, `wire` |
| `catalog` | `fnd`, `wire`, `schema` |
| `capabilities` | `fnd`, `schema`, `catalog` |
| `port` | `fnd`, `catalog` |
| `bridge` | `fnd`, `wire`, `bridge-codec` |
| `consent` | `fnd`, `wire`, `bridge-codec`, `bridge` |
| `plugin-runtime` | `fnd`, `schema`, `catalog`, `port`, `probe-document` |
| `hub-remote` | `fnd`, `wire`, `schema`, `inference-hub`, `hub-pair` |
| `shell-channel` | `fnd`, `catalog`, `port`, `bridge-codec`, `bridge` |
| `routing` | `fnd`, `catalog`, `capabilities`, `port`, `plugin-runtime`, `hub-remote`, `shell-channel` |
| `dispatch` | `fnd`, `schema`, `catalog`, `port`, `consent`, `inference-hub`, `routing` |
| `workspace` | `fnd`, `wire`, `schema`, `catalog`, `port`, `probe-document`, `plugin-runtime`, `inference-hub`, `hub-remote`, `shell-channel`, `routing`, `dispatch` |
| `tools-inference` | `fnd`, `wire`, `catalog`, `capabilities`, `port`, `probe-document`, `plugin-runtime`, `inference-hub`, `routing`, `dispatch`, `workspace` |
| `tools-gateway` | `fnd`, `wire`, `schema`, `catalog`, `capabilities`, `port`, `bridge-codec`, `bridge`, `probe-document`, `shell-channel`, `dispatch`, `workspace`, `tools-inference` |
| `server` | `fnd`, `wire`, `bridge-codec`, `bridge`, `consent`, `workspace` |
| `http-transport` | `fnd`, `wire`, `bridge-codec`, `bridge`, `consent`, `server` |
| facade `semio-framework-os-mcp` | all of the above (explicit `pub use` only) |

Closed-enum dispatch (`dyn_enum_close!`) crosses crates through `use <trait_crate>::__semio_dispatch_<Trait>;`, with `#![allow(async_fn_in_trait)]` in the trait-declaring crates. Test-only shared items sit behind the `testkit` feature (forwarded through the crates, dev-dependencies for the consumers). `test_allocation` (a shared external helper with a `global_allocator`) is mounted exactly once, in `foundation` under `testkit`.

## 3. God objects (rule added mid-task)

The rule triggers only when one struct's inherent impl cannot fit a crate. It does not trigger here, and I checked it per type instead of assuming:

- `HeadlessWorkspace` (about 1,490 impl lines): struct in `🏠️workspace/🪪️headless-workspace-state`, impls in `🛂️gateway-backend`, `🪪️headless-workspace-state` and `💡️inference-facade`. All three folders are in `workspace` (lib 1,704 lines, 119,598 B). One crate owns the struct and all its impls.
- `HubRemoteBinding`: all three `impl` blocks (`🔗️remote/🦀️.rs` twice, `🧩️pair/🔌️pair-binding/🦀️.rs`) live in `hub-remote`. The pairing half was moved up out of the pair file for that reason. `CanonicalPairActor` and `HubBindingError` stay in `hub-pair`, a lower crate that does not need the binding.
- `orphan.py`: 270 impls scanned, 0 whose inherent impl sits outside the type's crate. `traitcheck.py`: 0 trait-method scoping suspects.
- No extension traits were introduced, no `&mut WholeState` parameters were added to break a cycle (the partition had no atom-level cycle after piece splitting).

## 4. Files changed

- 22 new crate nodes under `M/` (each: `🦀️.rs` lib root, `📦️packages/🦀️rust/Cargo.toml`, `📋️project.json`), plus new piece folders where an atom group had to move (e.g. `🧭️protocol/🎯️mcp-server`, `🧵️bridge/🪢️session`, `🔀️dispatch/🔌️channel-port`, `🖥️ui/📋️job-registry`, `🏠️workspace/🔗️remote/🚨️binding-errors`, `🏠️workspace/🔗️remote/🧩️pair/🔌️pair-binding`).
- Every facet `🦀️.rs` and every `🧪️tests/**/🦀️.rs` under `M/` rewritten in place: imports regenerated, units redistributed, private members that cross a crate boundary widened to `pub(crate)`/`pub` (see 5.4). All `//#region` markers preserved: 308 in the original, 319 now (the extra ones are empty pairs for mount-only regions), no name missing.
- Facade glue `M/📦️packages/🦀️rust/🦀️.rs` and `Cargo.toml`: explicit `pub use` for the agent-credential, catalog, protocol, inference, workspace and flat names that consumers use; `[[bin]]` and the two `[[test]]` integration targets kept; `📜️script.ts` passes `-p` for every gateway package on cargo check/test/probe routes.
- Registries: root `Cargo.toml` (22 members, byte-sorted), `📋️build-owners.json` (22 rows), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (132 routes each, after the pets-traversal release test route), `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (new memberNames by depth).
- Lock files, produced by `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` (cargo itself, nothing hand-edited): root `Cargo.lock` (already held the 23 packages; the run added one peer package), `🌎️hub/Cargo.lock` and `✏️s/🧑‍💻dev/💡️services/Cargo.lock` (now 23 os-mcp packages each; the same runs also locked peers' not-yet-locked packages).
- TS and readers: `M/🟦️.ts` (new `MCP_GATEWAY_CRATE_ROOTS`, `MCP_GATEWAY_PACKAGES`, `readMcpGatewaySource(repoRoot, readFile?)`, a directory walk skipping `🧪️tests`, `🤖️generated`, `📦️packages`, `dist`, `node_modules`, because `readModuleTreeSource` refuses `#[path]` mounts that leave the declaring directory); root `📜️script.ts` (`mcpWorkspace` and the interactivity audit's bridge text); `…/📖️source-access/🟦️.ts` (stale workspace split root removed); hub `📦️packages/🦀️rust/📜️script.ts` (dispatch read includes `🔌️channel-port`); hub `🧪️tests/🧱️foundation-source/🟦️.ts` and `🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts` (use the reader); snapshot test `🛫️snapshot/🧪️tests/🟦️.ts` (slice path now `🔬️probe-document`).

## 5. Consumers re-pointed

Rust consumers (hub, gismap, dev-services) import only items the facade re-exports explicitly (`GatewayError`, `GatewayErrorCode`, `HeadlessWorkspace`, `run_mcp_entrypoint`, `CatalogSource`, `compile`, `InMemoryToolRegistry`, `ToolRegistry`, the credential-exchange names, the hub inference client names, `load_package_descriptor`, `exchange_delegated_session_v1`), so no consumer manifest changed. No compatibility alias exists.

## 6. Verification (real output)

### 6.1 Real compile at start and end

- Start (`🗑️generated/osmcp-w4-cargo-baseline.log`): `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-mcp --message-format=short` ended with `Finite original maintenance authority required`.
- End (`🗑️generated/osmcp-w4-cargo-end.log`), after the lock refresh: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-mcp-foundation --message-format=short` exits 1 with `error: Rust crate Source refused before compiler: semio-framework-os-kernel (Rust crate Source boundary exceeded before compiler byte allocation)`. The launcher is working again and passes `cargo fetch --locked`, but the source gate refuses the peer-owned `semio-framework-os-kernel` that every os-mcp crate depends on (the os-kernel-db carve is still running). No compiler ran. Cargo did resolve the whole workspace with the new manifests (the `update --workspace --offline` runs exited 0 for root, hub and dev-services), which validates the 23 manifests, the member paths and the feature names as cargo reads them.
- `bun test` on `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts` ends in a Bun segmentation fault (`panic(main thread): Segmentation fault at address 0x8033`, Bun 1.3.14) on repeated runs, while a trivial test in the scratchpad passes. I did not find the cause, so that test has no result from me. I ran the new reader directly: `readMcpGatewaySource("/Users/ueli/Documents/semio")` returns 1,718,929 characters containing `fn exchange_delegated_session_v1`, `struct HeadlessWorkspace` and `//#region`.

### 6.2 Union-sandbox rustc oracle (`osmcp-tools/oracle2.py`, nightly-2026-07-20, `--emit=metadata`)

Original tree and split tree are compiled in two sandboxes (each split crate root mounted as a module of one synthetic crate, external globs and `extern crate` neutralised, `dyn_enum_close!` stubbed to a plain enum); the error multisets are compared. Run after the final member widening:

| Config | Original errors | Split errors | Only original | Only split |
| --- | --- | --- | --- | --- |
| native | 274 | 273 | 2 (one is the `MockArtifactChannel` import) | 14, all warnings |
| `--cfg test` | 273 | 299 | 2 warnings | 43 (of which 26 are errors, all `env!("CARGO_*")` not defined in the sandbox: 24 `CARGO_MANIFEST_DIR`, 2 `CARGO_BIN_EXE_semio-os-mcp`) |

The 273/274 baseline errors are the sandbox's unresolved externals, identical before and after. wasm, wasmtest and p2 configs showed parity apart from message wording and integration-target artefacts (earlier run; not repeated after the widening, which only changes visibility). Warnings differ: unused imports, mostly because the hidden macro uses are not seen by the sandbox.
Negative control (`🗑️generated/osmcp-w4-oracle-negative.log`): a removed import plus a private `GatewayError` is caught as 53 E0603 and 3 E0425.

### 6.3 Definition and member scans

- `defscan2.py` (private members referenced from other files that mention the type): 26 hits before the receiver-agnostic widening, 1 after. The remaining hit is `SilentClient.gate`, two distinct test-local structs of the same name in `🛡️policy/🧪️tests/🔬️quick` and `🚚️transport/🧪️tests/🔬️quick` (false positive).
- Seeded sensitivity check on a scratch copy: making `GatewayToShell::copy_encoded_page` private makes the scan report `('GatewayToShell', 'copy_encoded_page', 'fn', '🧵️bridge/🦀️.rs', '🧵️bridge/🪢️session/🦀️.rs')`; the real tree does not.
- `verify_registry.ts`: `crates: 23 owners: 1889 launch configs: 10122 seed configs: 7154 … ALL REGISTRY CHECKS PASSED` (workspace members byte-sorted, owners unique, six routes per crate in both launch files, DAG acyclic, testkit forwarding consistent).
- `verify_static.py`: `missing: 0 longest path bytes: 147`, no unbalanced files, no region name missing (319 present), `rustfmt failures: 1` which is the pre-existing `🪶️sqlite/🫴️semantic/🦀️.rs` (the original has it too).
- `orphan.py`: `impls 270 suspects 0`. `traitcheck.py`: `suspects 0`.
- Declared os-mcp dependencies versus the full `#[path]` closure of each crate (inline scan): every declared `semio-*` dependency is referenced. (`depcheck.py` reports a few unused ones because it does not follow nested mounts; that is a tool limit.)
- All `#[path]` and `include_str!` literals resolve; the only duplicate `#[path]` owner is the shared `test_allocation` helper, now mounted once.

## 7. Residual risks and not done

1. No compiler has seen these crates. The oracle covers name resolution, imports, visibility at module level and macro-expanded items, but not type-level privacy (E0616/E0624 for members accessed through an inferred receiver), borrow checking, trait-method scoping beyond `traitcheck.py`, or `dyn_enum_close!` cross-crate expansion. The first launcher run that can compile (after the os-kernel carve lands) is the real test; expect small visibility fixes if any.
2. The receiver-agnostic widening is over-approximate by design: a private inherent member whose name appears as `.m(`, `.m` or a struct-literal field in any other unit that can see the defining crate was widened (`pub(crate)` inside one crate, `pub` across crates). It costs some encapsulation on the order of 600 members over about 160 types; tightening it needs the compiler.
3. Embedded assets (schema 99 KB, catalog 17.7 KB, probe-document under 1 KB) stay in their crates and inside budget. I did not add a runtime asset channel: `FacetLeaves` takes `&'static str` and is owned by the fleet. If the schema crate's bytes (165,236 B) become binding, a runtime channel is the next step.
4. Test closures of the facade (3,657 lines, 240,629 B) and `hub-remote` (3,570 lines) are above the soft target and inside the hard limit. A later split of the facade's test files along the component lines would fix it.
5. Pre-existing breakage unrelated to this work, confirmed before and after: the shared `⏱️trace/…/📥️requests/🦀️.rs` carries `pub(in crate::m_geometry)` from a peer, so os-mcp tests that mount it fail to compile identically; hub foundation-source (4 failures), the snapshot test (1 failure reading the snapshot crate), the flow wasm "original MCP source" test, `os-source-topology` and `windows-command-paths` (fixture path ENOENT).
6. `bun repo verify taxonomy report` timed out in graph construction, so the taxonomy entries were added but the report did not run.
7. Intra-doc links in moved docstrings may dangle.
8. Peer concurrency: the repo changed under me during the task (a peer's `package_assets` module moved out of the mcp tree; the generator always re-runs from the pre-split snapshot). The lock-file refresh also locked unrelated peer packages in the hub and dev-services locks; that diff is cargo's, not mine.
