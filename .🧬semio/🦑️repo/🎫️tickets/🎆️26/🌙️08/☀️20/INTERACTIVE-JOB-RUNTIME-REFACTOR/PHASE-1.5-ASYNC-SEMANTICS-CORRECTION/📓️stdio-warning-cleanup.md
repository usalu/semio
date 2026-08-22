# Stdio Warning Cleanup

## Scope

- Removed 154 generated stale `UiNode` imports.
- Moved the two `UiNode` imports still needed by raw-binary and deflate editor unit tests behind `#[cfg(test)]`.
- In the root `StdioApps` dyn-enum, replaced fully-qualified `VcsArtifactApp`, `EditorApp`, and `ViewerApp` paths with the existing close-prelude imports; this removes the unnecessary-qualification diagnostics without changing types.
- Changed the doc comment immediately preceding the macro invocation to a regular source comment, removing the macro rustdoc warning.
- No codec, owned-compression, or interpreter behavior changed.

## Verification

| Check | Result |
| --- | --- |
| `cargo check -p semio-s-plugin-stdio --lib --no-deps` | Cargo rejects `--no-deps` for `check`; it is a Clippy-only option. |
| `RUSTFLAGS='-D warnings' cargo check -p semio-s-plugin-stdio --lib` | Blocked before stdio by 21 pre-existing dependency `async_fn_in_trait` warnings elevated to errors. |
| `cargo clippy -p semio-s-plugin-stdio --lib --no-deps -- -D warnings` | Reaches stdio, but fails on 1,230 unrelated existing Clippy errors. The requested stale-import, root dyn-enum qualification, and root macro-rustdoc categories no longer appear. |
| `cargo check -p semio-s-plugin-stdio --lib` | Passed in 1m 18s; the crate still emits 20 unrelated warnings (19 unnecessary qualifications and one macro rustdoc warning in the semio artifact component). |
| `rustfmt --check` (root and the two test-scoped files) | Passed. |
| Static scope checks | Zero non-test `UiNode` import leftovers, zero root qualified dyn-app type paths, and zero root doc comments directly before `dyn_enum_close!`. |

The strict lint result is not clean due to warnings outside this assigned cleanup. See the command log and exact path inventory for reproducibility.
