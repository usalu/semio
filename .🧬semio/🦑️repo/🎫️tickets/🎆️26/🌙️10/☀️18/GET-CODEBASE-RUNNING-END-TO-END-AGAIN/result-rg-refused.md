# Result rg-refused (executor `rg-refused`, Sonnet 5.5): refused crates repaired, crates at or over the gate split

Foreground, no sub-agents, no modifying git command. Tools in `C/rg-refused-tools/` (`one.ts` canonical single-crate gate, `deps.ts` file dependency graph, `owners.ts` which crates compile a file, `uses.ts` residual ident check, `repoint.py` importer re-pointing, `usage.py` per-name consumers, `parse.ts` TOML/JSON parse). `C` = this ticket folder. Gate = `readRustSourceClosureV1` with `crateTreeHostCfg()` (prod) and `+test`: 4,000 active lines / 32 files / 262,144 bytes.

## 1. Why the six crates were refused (root causes, all repaired)

| crate | reader error | root cause | fix |
| --- | --- | --- | --- |
| `semio-framework-repo-tickets` | Unbalanced Rust source delimiter | the bulk commit replaced the children block of `🦀️.rs` (7 `#[path]` mounts) by the bodies of `search` and `lifecycle::RealPurge` (106 stray lines, an unmatched `}`) | restored the children block verbatim from `b84bf526c7b` (file is byte-identical to that revision again) |
| `semio-framework-repo-mcp` | Unbalanced Rust source delimiter | bulk commit inserted the 5-line tail of `contract_server` (`}), )?; Ok(server) }`) under `//#region Children` of the root | removed the 5 lines (tail exists in `🗄️repository`) |
| `semio-framework-os-vcs` | Missing module history_read | `#[path="🔗️read/🦀️.rs"] pub mod history_read;` inside `🗃️history-ledger/🦀️.rs`: a `#[path]` in a file module resolves against the directory of THAT file | `#[path="../🔗️read/🦀️.rs"]` |
| `semio-framework-plugin-decl-app` | Missing module original_command | same defect twice in `👷️job/🧵️reserved-tool-job/🦀️.rs` (`🎟️request/🔒️command`, `🎟️request/♻️retirement`) | `../🎟️request/...` for both |
| `semio-framework-server` | Missing module tests | bulk commit appended a stray `mod tests;` (no `cfg`, no path) to `📡️gateway/🦀️.rs`; the unit tests are mounted by the companion crate | removed the line |
| `semio-framework-3d-mesh-document` | Missing module journal | `#[path = "📝️journal/🦀️.rs"]` resolves under `📄️document/`; another agent corrected it to `../📝️journal/...` while I worked (also `🔍️structure`) | the crate is no longer refused; it was then 1.253 prod / 1.436 test and is split below |

Canonical gate after the repairs (prod / test): repo-tickets 0.712 / 0.740, repo-mcp 0.557 / 0.601, os-vcs 0.688 / 0.938, plugin-decl-app 0.795 / 0.875, server 0.865 / 0.865.

## 2. Splits

| crate | before (prod / test) | after | how |
| --- | --- | --- | --- |
| `semio-framework-3d-mesh-document` | 1.253 / 1.436 (26 files, 269,748 B, 5,012 lines) | document 0.512 / 0.875; journal 0.472 / 0.544; surface 0.194; quality 0.113 | three leaf crates carved out of the mounts of the document root: `semio-framework-3d-mesh-journal` (`📝️journal`: overlay, editor, six gestures), `-surface` (`🎨️surface`), `-quality` (`🔎️quality`). None of them is used by the core (checked by name). They import from `semio_framework_3d_mesh_document` explicitly; `protocol::value::DslValue` in the journal never resolved in this crate and is now `semio_framework_value::DslValue`; `super::super::` paths in the journal tests re-pointed. The facade `semio_framework_3d::mesh` re-exports the three crates explicitly (no glob) so the 2 generation3d consumers and the mesh test crate keep one public path. |
| `semio-framework-os-services` | 0.949 (3,794 lines) | 0.760; new `semio-framework-os-services-native-io` 0.219 | `🚪️native-io` (+ `📮️path-return`) is self-contained (job + value only) and was mounted from the package glue. Own crate; importers re-pointed with `repoint.py` (8 files in host-base, kernel-protocol, host-plumbing, runtime-mailbox-tests, renderer tests; `semio-framework-os-services` dependency replaced by the new crate in those four manifests, added next to it in the wgpu crate that still uses another service); companion `testkit::native_io` hook replaced by a direct dependency. |
| `semio-framework-plugin-reactor-turn` | 0.908 | 0.879; new `semio-framework-plugin-reactor-turn-limits` 0.031 | `🧯️retirement-drivers-and-turn-limits` (pure bounded retirement drivers + turn-limit constants) is a leaf; `pub(super)` widened to `pub`, broken intra-doc links turned into plain code spans, root imports the nine names explicitly (no `pub use *`), 4 external test roots re-pointed, `tests-03-2` got the dependency. |
| `semio-framework-plugin-tests-01` | 0.918 (240,545 B) | 0.645 (169,082 B) | the established run-time fixture practice: the 59,891 B `tool-latest-wins.json` (4 sites) and the 11,811 B `📮️publication` source text (1 site) are read with `semio_framework_test_fixtures::fixture_text!` (manifest-relative, same prefix depth as the existing `fixture_path!("../../../..")`) instead of `include_str!`. No new crate. |

## 3. Registration

- `🧰️framework/Cargo.toml` members: `3d` journal, surface, quality. Root `Cargo.toml` members: `os-services-native-io`, `reactor-turn-limits` (+ `[workspace.dependencies] semio-framework-os-services-native-io`).
- `📋️build-owners.json` rows: `semio-framework-3d-mesh-{journal,quality,surface}` (Cargo manifests, like `-document`), `@semio-tech/os-services-native-io-rs` and `@semio-tech/plugin-reactor-turn-limits-rs` (minimal `📋️project.json`, like the uicarve crates).
- `🧪️packaging/🧪️.json` of the mesh family: three new packages and the facade's three new production edges.
- `semio-framework-3d` and `tests-03-2`, `os-services-tests` manifests got the new path dependencies.

## 4. Not split / open (with the reason)

- `semio-framework-plugin-instance-app` 0.949 (bytes, hard cap not at risk): every file is an inherent `impl VcsArtifactApp` or a trait impl of a foreign trait (`PluginApp`); inherent impls cannot leave the crate of the type, and the previous waves already moved every field family into the ten `instance-*` crates. Cutting more needs a bundle migration wave (move `staging/*` methods onto the `*Ctx` bundles), not a crate cut.
- `semio-framework-artifact-workflow-run` 0.906 and `semio-framework-os-config` 0.906 (both on the file count, 29 / 32): snapshot, diff, mutations and the `io` codecs form one cycle (the snapshot names `RunMutation`, the io impls are foreign traits for the snapshot type), so any crate cut violates the orphan rule or creates a cycle; removing files would break the one-file-per-component taxonomy.

## 5. Verification

- Canonical gate, final run (prod / test), full table in `C/🗑️generated/rg-refused-logs/gate-final.txt`: tickets 0.712 / 0.740, repo-mcp 0.557 / 0.601, os-vcs 0.688 / 0.938, decl-app 0.795 / 0.875, mesh-document 0.512 / 0.875, mesh-journal 0.472 / 0.544, mesh-surface 0.194, mesh-quality 0.113, os-services 0.760 / 0.773, os-services-native-io 0.219, reactor-turn 0.879, reactor-turn-limits 0.031, tests-01 0.645, instance-app 0.949, workflow-run 0.906, os-config 0.906.
- `bun test ./🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🟦️.ts`: 2 pass, 0 fail (4,818 expects): schema (now `minItems/maxItems 11`), owners, direct codec imports, per-package source gate, manifest parity, and the lock law over all 30 mesh-family `Cargo.lock` files. Lock state: framework / root / `✏️s` / procedural-bridge / puzzle-5d were refreshed by `cargo update --workspace --offline` (queue); the other 27 mesh locks were patched textually with `C/rg-refused-tools/locksync.py` (3 packages + 3 facade edges each, sorted, idempotent).
- `rustfmt --edition 2021 --check` parse of the 30 edited `.rs` roots: 0 errors; TOML/JSON parse of every edited manifest, root and framework `Cargo.toml`, `📋️build-owners.json`: 0 errors.
- `cargo update --workspace --offline` root (adds `os-services-native-io`, `reactor-turn-limits`) and framework workspace: ok.
- NOT RUN (blocked): every `bun repo cargo ... check`. (1) The source-boundary admission refuses every crate graph that contains `semio-framework-job` (1.088 / 1.250) or `semio-framework-value` (1.795 / 3.470): `Rust crate Source refused before compiler: semio-framework-job`; this clears when `rg-core` lands. (2) A mesh-free check of `-p semio-framework-repo-mcp -p semio-framework-repo-tickets --lib --tests` (no job/value in its graph) failed three times in the storage layer before any rustc ran: first `System image plist required`, then `Bounded unique parent kernel roster required` (`💾️storage/🧹️cleanup/🎛️command/🟦️.ts:132`: the persistent-parent `hdiutil info -plist` inventory, 21 images mounted / 35 KB at the time). That needs the cache-image owner; I did not touch images.

## 6. Concurrent change in scope that I did not fix

`semio-framework-server`: at 15:25 another agent rewired the gateway: `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` in `📡️gateway`, removed the `testkit` hooks and the companion `🛰️gateway` mount, added `pub mod test_instance` under `cfg(test)`. That repairs the refusal (the stray `mod tests;` I had deleted) but puts the gateway laws into the product crate: prod stays 0.864, **test is 1.394** (5,576 lines; bytes 1.069). Cheapest cut that keeps their direction: carve the presence family (`👥️presence`, `🛰️presence-stream`, `📣️fanout`, 1,278 prod lines + its 826 test lines) into its own producer crate re-exported by the facade like `contract`/`storage`/`policy` (test then ~0.87). Left alone because the file set was still being edited.

## 7. Crates for the coordinator to compile once job/value are split and the image roster is healthy

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`:
`semio-framework-repo-mcp`, `semio-framework-repo-tickets`, `semio-framework-os-vcs`, `semio-framework-plugin-decl-app`, `semio-framework-server`, `semio-framework-3d-mesh-document`, `semio-framework-3d-mesh-journal` (its unit tests now need the dev-dependency on `semio-framework-3d`; they call `MeshModelingOps::{box_prim, plane_prim, extrude_faces, ...}` which never existed on the document crate, so the old in-document mount could not compile either), `semio-framework-3d-mesh-surface`, `semio-framework-3d-mesh-quality`, `semio-framework-3d` (facade), `semio-framework-3d-mesh-tests`, `semio-framework-os-services`, `semio-framework-os-services-native-io`, `semio-framework-os-services-tests`, `semio-framework-os-renderer-wgpu-{host-base,kernel-protocol,host-plumbing,runtime-mailbox-tests}`, `semio-framework-os-renderer-wgpu`, `semio-framework-plugin-reactor-turn`, `semio-framework-plugin-reactor-turn-limits`, `semio-framework-plugin-tests-03-2`, `semio-framework-plugin-tests-01`.
