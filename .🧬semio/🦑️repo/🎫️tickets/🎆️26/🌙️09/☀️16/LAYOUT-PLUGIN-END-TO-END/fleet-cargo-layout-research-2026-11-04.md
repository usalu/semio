# Cargo Build Directory Evidence

Official Cargo compiler layout documents place an intermediate beneath target/profile/build/package/hash/out. Both package and crate names therefore consume the shared 256 UTF-16 path budget. The Cargo configuration reference permits build-directory templates, but does not supply a per-unit template to omit the repeated package segment. The repository keeps the current fine-grained layout and its finite path guard.

Sources: [Cargo compiler layout](https://doc.rust-lang.org/stable/nightly-rustc/cargo/compiler/layout/index.html), [Cargo configuration reference](https://github.com/rust-lang/cargo/blob/master/doc/book/src/reference/config.md), [Cargo build cache](https://doc.rust-lang.org/nightly/cargo/reference/build-cache.html), [Rust build layout v2 testing](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/).

The authenticated product attempt reached real compilation and materialized an over-budget intermediate. The canonical repair shortens owned storage and checks the actual selected unit graph before compilation. It does not change Cargo layout mode, Source size limits, the 256-character guard, or physical custody. Exact measured object suffix widths and allocator decisions are retained in the execution and allocator audit reports.
