# Native Artifact Contract

The first restoration probe recovered the rlib byte-for-byte but the Rust consumer rejected it: the workspace enables `no-embed-metadata`, so the separate rmeta is a required deliverable. The corrected contract stages the matching rmeta with its rlib and includes link dependencies for library consumers. It excludes incremental directories, fingerprint stores, dep-info, and build-script executables.

Cargo itself emits both paths in its compiler-artifact JSON. The root rlib uses a stable name while its rmeta stays under `deps` with a hash; staging gives the pair matching names.

This requirement agrees with the [Rust compiler metadata contract](https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/embed-metadata.html) and the [Cargo uplift issue](https://github.com/rust-lang/cargo/issues/17359).

Verified on macOS arm64 with the pinned nightly toolchain: cold target completion, warm Nx cache hit, deletion/restoration of the complete pair, byte identity, compilation of a separate Rust consumer using only the restored rlib/rmeta, and successful consumer execution. The consumer produced the known SHA-256 of abc.

The consumer passes both rlib and rmeta with `--extern`, matching [Rust’s documented manual linking requirement](https://blog.rust-lang.org/inside-rust/2026/08/18/reducing-target-dir-size-on-nightly/). No Cargo target directory was used to compile or execute that consumer.

Artifact hashes:

- `.nx-artifact.json`: `0db003ab5dd6eee88dcfef4a85067a4fa7d6c911a4d93d2bbb666af13e624d99`
- `libsemio_framework_hash.rmeta`: `3ed6bec660a16cb669b4d55742f05ccd7baaf01f826faee4714465d86c7495a8`
- `libsemio_framework_hash.rlib`: `5f681f9b4673b254737aa9a2b629d50fea0775f8d393453b8ffe24f868ade218`
