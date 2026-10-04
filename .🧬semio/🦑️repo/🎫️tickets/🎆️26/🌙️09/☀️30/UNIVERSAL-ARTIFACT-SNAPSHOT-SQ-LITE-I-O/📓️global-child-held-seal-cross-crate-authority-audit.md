# Held Child Read Sealing: Actual Crate Boundary

Read-only current source audit; no compilation, activation, or runtime receipt.

The held sealing pair is not compiler-ready with the existing capture placement. Kernel and Plugin are separate Rust crates, rather than one host crate.

| Authority | Exact evidence |
|---|---|
| Kernel | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:3,18,20` names `semio-framework-os-kernel`, library `semio_framework_os_kernel`; its Rust root:239–242 includes the actual Store component under `os_store`. |
| Plugin | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml:3,16–17,70` names `semio-framework-plugin`, selects its own Rust root and depends on Kernel. Its Rust root:9–12 imports Kernel as dsl/protocol/store/vcs and :33–34 includes its own Plugin component. |
| Held capture | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/👁️capture/🦀️.rs:5` implements `store::ArtifactChildSnapshotOwner<S>` for the Plugin-owned `ChildContentEntry`; :22 calls `store::ArtifactChildRead::from_owner`. |
| Held seal | `📥️inputs/global-child-real-registry-read-construction-seal-held-pair.json` changes both that trait and constructor to `pub(crate)` in Kernel-owned Store read. Neither is accessible to the separate Plugin crate. |

This is a precise held integration blocker, not an executed compiler failure. The read/capture facets are still unmounted in the actual module graph. Syntax-only parses cannot check the privacy boundary. Keep the external construction seal, but place the registry-backed construction capability in its real owning crate or design a sealed cross-crate capability whose authentic producer cannot be forged by artifact callers. Do not restore a public arbitrary-Arc constructor merely to resolve this prerequisite. Full logical/target identity, typed snapshot authority, command-frame capture, all caller closure, and retirement remain separate unfinished obligations.
