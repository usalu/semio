# Puzzle5D editor authoring identity

The retained editor test binary ran `created_part_board_and_world_edits_reopen_with_history` and reached `addPartKind` after switching to the empty document. The first creation returned `module.vcs` with `native encoding refused: original work requires its original caller loan`. The file and pack reopen assertions had yet to run.

The debugger stopped at the exact error literal inside `OriginalWork::prepare` in the old stripped binary. The creation worker had already produced its mutation. The test dispatcher wraps application authoring in the shared `with_authoring_identity!` fixture macro. That macro created a native authority with no Work lender. The shared `fixture_identity()` helper already installs an original Work authority for each native authoring hop. Other 5D codec tests successfully use this helper.

The fixture macro now delegates to `fixture_identity()`, keeping its existing closure and await usage. The production authoring identity API and original Work enforcement are unchanged. The editor history law and large example case are queued against the repaired fixture.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs`
- `history-unfunded-encoding.gdb`
