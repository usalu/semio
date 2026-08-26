# VCS Decorative Async Removal

## Scope

- Plugin: the complete `semio-s-plugin-vcs` artifact/editor/viewer/schema/I/O taxonomy.
- Baseline: 264 `async fn` declarations and zero `.await` expressions (99 in the editor subtree captured first, then 165 in the remaining plugin tree).
- Change: removed all 264 decorative async markers across 63 Rust files; retained async test attributes are test-runtime declarations, not production call-chain suspension.

## Evidence

- Raw first-pass editor inventory: `vcs-deasync-before.txt` in this ticket directory. The remaining-plugin count was captured immediately after that pass, before its mechanical rewrite.
- Post-change source scan: zero `async fn` and zero `.await` expressions across the complete VCS plugin.
- `cargo fmt --check` equivalent diff check completed successfully for the touched sources.
- Runtime compilation is deliberately pending the shared single-compiler lease; no runtime-green claim is made here.

## Interaction Consequence

The VCS app command, render, configuration, presence, and taxonomy helper paths are now synchronous where their bodies were already synchronous. This removes false executor/future boundaries without changing serialized commands or emitted event-sourced mutations.
