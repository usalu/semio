# 🔱 Residual Audit: Trinity s-plugin and Jack Artifact

Read-only audit. No source file was edited. Sources are the ticket folders under `C:/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/`, the plugin under `C:/semio/✏️s/🔌️plugins/🔱️trinity/`, and the goal folders under `C:/semio/.🧬semio/🦑️repo/🎯️goals/RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-OS/RUNNING-S/RUNNING-PLUGINS/RUNNING-TRINITY/`.

## 1. Project to targets

`repo show project` did not complete. The `@semio-tech/trinity-jack-rs` call ran past 120 s on a `build:source-admission` script and was stopped. The targets below come from the ticket records and `Cargo.toml` playground declarations, not from the repo CLI. Re-run the CLI once the build graph is idle.

| Project | Targets and role (source) |
|---|---|
| `@semio-tech/trinity-jack-rs` | Jack Rust crates under `🗿️artifacts/🔌️jack` (Trinity TRINITY-JACK-CQRS-GATE, GET-TRINITY-WORKING) |
| `trinity-jack` | Playground `dev:trinity:jack` on react port 6054, `app` pin in `📦️packages/🦀️rust/Cargo.toml` (TRINITY-PLUGIN-END-TO-END) |
| `trinity-jack-lsp` | Crate `semio-s-plugin-trinity-jack-lsp`, separate installable shim at `🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp/📦️packages/🦀️rust` (residual handoff) |
| `trinity-jack-lsp-worker` | `@semio-tech/trinity-jack-lsp-worker`, TS worker at `🧠️lsp/🟦️component.ts` (residual handoff) |
| `trinity-jack-shell` | Crate `semio-s-plugin-trinity-jack-shell`, bin at `🐚️shell/📦️packages/🦀️rust/📦️bin.rs` (residual handoff) |
| `trinity-rewriting` | Playground `dev:trinity:rewrite` on react port 6056, `app` pin in `Cargo.toml` |
| `trinity-rewriting-rs` | Rewrite engine and host, `trinity/rewrite/engine` (CONVERT-TRINITY-RAM-AND-REWRITE, COMPLETE-GRAPH-REWRITING) |
| `trinity-js` | Not located as a project name in the reviewed records. Closest are `@semio-tech/trinity-react` (TRINITY-PLUGIN-END-TO-END) and `trinity-rewrite-react`. Unverified. |
| `trinity-plugin` | `semio-s-plugin-trinity`, `trinity/plugin/rs/lib.rs` (GET-TRINITY-WORKING, FIX-TRINITY-PLUGIN-PORT-KEY, NORMALIZE-TRINITY-PLUGIN-CRATE) |

## 2. Documented but unresolved gaps, TODOs, coverage limits, follow-ups

Source paths are relative to the ticket folder unless absolute. Items marked "verify" were claimed fixed in a later record but not re-checked here.

### Runtime and performance

1. **Whole-document load takes about 8 minutes** (`setActiveExample`). Each `JackSnapshot` copy carries its own 42-kind Nakagin manifest, so every retired snapshot costs about 10 s. The proposed fix (share the manifest, e.g. behind an `Arc`) is a schema-level change that was deliberately deferred. Source: `🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/📓️status.md`, section 2026-09-17.
2. **Rewriting has no `setActiveExample`.** The shell's boot example dispatch is refused as `undeclared-action`, one cosmetic warning per boot. Source: same file.
3. **`addRuleClause kind=parameter`** was once recorded as adding no row. Later record says it now works, so this is closed. Source: same file, 2026-09-18.
4. **Probe trap, not a product bug:** `undo` pops the shell's shared input ledger, so any panel toggle between a mutation and undo is what gets undone. Source: same file.

### Build and test debt

5. **Jack native mounted suite is not green.** Partial run: 50 pass, 24 fail, 3 unfinished (`documents_reload_identically` over 60 s). The default stack overflows at 2 MiB and 32 MiB, so a 128 MiB minimum stack is required. The query-ownership group fails 3 of 6, refusing the mounted actor and instance at test lines 24, 133 and 185 (fixture routing drift). The other full-suite failures were not classified. Source: `🌙️11/☀️20/RUN-JACK-END-TO-END-ON-ARTIFACTS/closure-audit.md`. Note the RUN-JACK ticket is closed while this record remains open.
6. **Coverage limits for Jack and Puzzle runs.** The native runs were on macOS only. Windows and Linux were not run. Source: `🌙️11/☀️20/RUN-JACK-END-TO-END-ON-ARTIFACTS/verification.md` (Coverage limits) and `🌙️11/☀️21/COMPLETE-GRAPH-REWRITING-FOR-5D-PUZZLE-GRAPHS/implementation.md`.
7. **Workflow with both compiled guests was never run.** The Puzzle 5D and Trinity rewrite workflow, with both packaged guest components, was not executed. The desktop UI was not exercised on any OS. The shared build fleet stalled during admission. Source: `🌙️11/☀️21/.../implementation.md`, and `🌙️11/☀️21/.../workflow-cache.md` ("did not run a workflow with both compiled guest components").
8. **Mounted Trinity archive and media tests were fixed through harness changes.** The fixture settlement changed the testkit, not the product. Source: `🌙️11/☀️21/.../📓️archive-fixture-settlement.md`. Worth a regression check.
9. **Planning bound in Semio mutation.** The final conversion to native edit operations stays in one worker turn after paged query evaluation. The remaining bound and a paged design are documented but not implemented. Source: `🌙️11/☀️20/RUN-JACK-END-TO-END-ON-ARTIFACTS/content-adapter.md` and `verification.md`.
10. **Native doctests** failed because `rustdoc` was missing from PATH. Source: `verification.md`.
11. **Stale doctest or extern error** on a `libvcs.rlib` path, judged unrelated to the change. Source: `🌙️07/☀️27/FIX-TRINITY-PLUGIN-PORT-KEY-DELIMITER-MISMATCH/🎫️ticket.json` summary.
12. **Dag play tests failed** (4 of 14 in `dag/play`). Flow play had an import failure from a pre-existing environment issue. Recorded 2026-07-02 and likely stale. Source: `🌙️07/☀️02/GENERALIZE-JACK-GRAPH-DSL/verify-log.md`.
13. **Headless Playwright has no WebGPU**, so the GPU path logs `NoCompatibleDevice`. Visual GPU rendering was not verified. Source: `🌙️07/☀️23/GET-TRINITY-WORKING-END-TO-END/notes.md`.

### Shell, LSP and worker (residuals)

14. **`JackLspSession` wasm surface regression (verify).** The TS worker imports `JackLspSession` from `@semio-tech/trinity-jack-lsp`. The handoff recorded this as missing from the shim. The type is now present in `🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp/🧩️component/🦀️.rs`, so the wasm export and worker wiring need a check. Source: `🌙️08/☀️06/TRINITY-PLUGIN-RESIDUAL-MOP-UP-JACK-TOOLS/📋️registrar-handoff.md` and `📌️important.md`.
15. **Shell live `cargo check` and `test` blocked.** Cause: an external `infinite-canvas` `build.rs` path (assets moved to `🖼️assets/🔣️icons`), plus a `core ↔ ui ↔ s-3d` workspace cycle. The handoff marks this as external. Source: `📌️important.md` and `📋️registrar-handoff.md` (`shell.check` and `shell.test`, blocked-external).
16. **Stale paths in handoff records.** The handoff lists `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust` and `🧠️lsp/...`. The current tree has these under `🗿️artifacts/🔌️jack/🔨️modules/...`. Root `Cargo.toml` members and `package.json` workspaces need re-checking against that layout.

### Closure and attribution

17. **Jack close file list not enumerated.** The closure audit requires each implementer to confirm its files and shared-file hunks before the close operation's file list. Untracked SetRoot files are included in that requirement. Source: `🌙️11/☀️20/.../closure-audit.md`.
18. **Shared `framework` API adaptations** (retirement and snapshot receiving contracts) are attributed to Jack but still need owner confirmation. Source: `🌙️11/☀️20/.../verification.md`.
19. **Mounted editor generic-factory check** was changed to match the outer short name. Confirm no behavior drift. Source: `🌙️11/☀️20/.../editor.md`.

### Features and design gaps

20. **Jack RETURN returns scalar rows only** (`RETURN a, r, b` coerces nodes to names). Planned in the FINISH-JACK-PLAY gap table. The closed summary does not mention it. Verify. Source: `🌙️07/☀️01/FINISH-JACK-PLAY/finish_jack_play_985e7edd.plan.md` (Current gaps).
21. **Canvas LOD and wheel handler.** The FINISH-JACK-PLAY gap table lists a LOD stub and no wheel handler. The closed summary claims 6-band LOD. Verify wheel zoom. Source: same plan, and `🌙️07/☀️01/FINISH-JACK-PLAY/🎫️ticket.json`.
22. **Query to graph sync.** The FINISH-JACK-PLAY gap table lists CREATE, SET and DELETE not reaching the canvas. The closed summary says `runJackJsonWithFixture` syncs mutations to `docStore`. Verify. Source: same plan.
23. **Rewrite LHS and RHS CRUD editing.** Node and edge creation by dragging from a palette is out of scope and recorded as a follow-up. Source: `🌙️07/☀️01/TRINITY-REWRITE-LHS-GRAPH-SYNC/trinity_rewrite_lhs_graph_sync_e3116e2a.plan.md` line 80.
24. **Generic-VCS TS mirror still used.** `framework/core/vcs-sync.ts` is still used by about 20 `*/play` files, and its retirement is out of scope. Source: `🌙️07/☀️01/TRINITY-JACK-CQRS-GATE/trinity_jack_cqrs_gate_dfca5379.plan.md` line 45.
25. **Manifest is not in the DSL round-trip.** This is by design. Manifest and TrinityManifest are resolved by ID, and only `PortDirection` gets a local twin. Source: `🌙️07/☀️26/CONVERT-TRINITY-RAM-AND-REWRITE-TO-DSL-DERIVE-ENGINE/🎫️ticket.json` summary.
26. **Trinity document birth.** Parent-only blank and demo loads were fixed, and a mounted parent-only test passed. The record also says this was not confirmed by execution at audit time, so it needs regression coverage. Source: `🌙️11/☀️21/.../trinity-document-birth.md`.
27. **`s` hub activation for rewriting is not documented.** Jack activation is documented (`s.trinity.jack`). The reviewed records do not show rewriting activation in `s`. This matters for the RUNNING-REWRITING goal.

### Documentation

28. **`✏️s/🔌️plugins/🔱️trinity/AGENTS.md` has empty concept sections.** The headings Node Kind, Node, Edge Kind, Edge, Port Kind, Port, Property and Data have no body. `README.md` is frontmatter plus a title only. Documentation is a gap. Both files are read-only in this audit.

## 3. What RUNNING-TRINITY and its sub-goals require

- **RUNNING-TRINITY** (`🎯️goal.json`, due 2026-12-31, open, GitHub issue 2657): "Trinity loads in s and exposes its artifacts." Parent is RUNNING-PLUGINS.
- **RUNNING-JACK** (GitHub issue 2697, open): "The jack engine and surface run inside Trinity." Requires the Jack engine and editor, viewer, LSP, shell and dev graph to run inside the Trinity plugin in `s`. Evidence so far: mounted Hub activation through `s.trinity.jack`, the native aggregate, and dev-server boots on 6054.
- **RUNNING-REWRITING** (GitHub issue 2698, open): "The rewriting engine and surface run inside Trinity." Requires the rewrite engine and surface inside Trinity in `s`. Evidence so far: playground boot on 6056 and the Puzzle 5D workflow ports. `s` activation is not yet documented (gap 27).
- **Current ticket GET-TRINITY-AND-JACK-WORKING-END-TO-END** (GitHub issue 2793): builds pass, tests pass, loads in `s`, exposes artifacts. Fall back to native tools if the shared fleet fails. The ticket has only its open record, with no work logged yet. The fleet stalled during the 5D and RUN-JACK runs, so the native fallback is already in use in prior records.
- **Minimum to close RUNNING-TRINITY:** (a) green native Jack mounted suite under a 128 MiB stack, (b) `repo show project` results for all nine projects, (c) shell and LSP cargo check and test unblocked, (d) rewriting activation in `s`, (e) a run of the workflow with both guests on a single OS.
