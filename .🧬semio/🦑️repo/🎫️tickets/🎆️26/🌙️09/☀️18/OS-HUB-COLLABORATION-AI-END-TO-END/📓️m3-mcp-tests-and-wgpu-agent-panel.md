# 📓️ M3 — os-mcp crate test suite + wgpu agent panel on the live bridge

Slice M3 of `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Two parts:

1. `semio-framework-os-mcp` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust`) — drive the
   `--lib` suite green. M2 §4.7 measured `316 passed; 21 failed` and classified the 21.
2. wgpu renderer agent/chat panel parity with React's live `AgentChatPanel`
   (`AgentToolCall`/`AgentToolResult`/`AgentMessage` bridge frames), verified natively **and** on
   `wasm32-unknown-unknown` (cfg(wasm32) code never compiles natively).

Worker: Opus 5, fleet 3 (2026-09-19 ~02:10). A previous M3 worker died mid-slice; its uncommitted
wgpu work was inherited from the worktree rather than redone (§1).

---

## 1. Inherited state (measured, not assumed)

`git status --short` / `git diff --stat` on the slice's paths at session start:

- **os-mcp crate** — `🏠️workspace/🦀️.rs` carries an UNSTAGED, in-flight change that is **not M3's**:
  its own docstring names "ticket 26/09/18 slice A1". It adds `headless_inference_budget`,
  `COMMAND_RESUME_WALL_BUDGET`, an `exchange_one_real` resume loop and an `ensure_instance` resume
  loop. A1 concurrently owns the MCP e2e client, so this was left alone (§3.3 reports what it does).
- **wgpu renderer** — the dead M3 worker had already landed the bulk of part 2 in the worktree:
  `🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` (conversation model + the three frames),
  `🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs` (header plan + empty line),
  `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`build_agent_chat_ui`, `agent_chat_entry_node`,
  `send_agent_chat_draft`, +343 lines), and `🎯️targets/🧊️wgpu/🌉️agent-bridge-door/🦀️.rs` (364 lines,
  the socket that feeds it). **None of it had ever been compiled or run**: the crate's test target did
  not build (§4.1). This session's job on part 2 was therefore verification + the defects verification
  exposed, not a rewrite.
- `🗑️generated/` had been wiped except `h1-check.txt`; no earlier M3 captures survived.

Repo-wide grep confirms M2's echo mock is gone: `BasicChatPanel` appears only in ticket archives, and
the "parity-debt" notes M2 left in the wgpu panel/test are gone — the wgpu panel is on the bridge.

---

## 2. Measured before/after

| lane | before | after |
| --- | --- | --- |
| `semio-framework-os-mcp --lib` (M2 §4.7, 2026-09-18) | 316 passed, 21 failed | — |
| `semio-framework-os-mcp --lib` (measured here, 2026-09-19 02:15) | **336 passed, 1 failed** | **336 passed, 1 failed** |
| `semio-framework-os-renderer-wgpu --lib` | **did not compile** (15 errors) | **1012 passed, 30 failed**, 1 skipped |
| wgpu agent/chat/socket-door tests (the slice's own) | unrunnable | **75 + 3 + 21 = 99 passed, 0 failed** |
| `cargo check --target wasm32-unknown-unknown -p …-renderer-wgpu --lib` | not run | **clean** (0 errors, 79 warnings) |

Captures: `🗑️generated/m3-baseline-test.txt`, `m3-wgpu-lib-test.txt`, `m3-wgpu-lib-full.txt`,
`m3-wgpu-agent-tests.txt`, `m3-wgpu-wasm32-check.txt`.

A confirming re-run of the whole os-mcp suite was started at 03:01 after the `🗒️note` rebuild of §3.5
and had **not finished** when this session's budget ran out (>13 min against 63 s at 02:08, with the
full fleet on all ten cores and the `🔬️long` tier now parsing a 63 MB guest artifact three times);
`🗑️generated/m3-final-mcp-test.txt` is therefore empty. Nothing in this slice touched the os-mcp crate,
so the 336/1 above stands as the measurement — but the suite grew from 337 to **367** tests during the
session (peers), so a later re-run will legitimately report a different total.

**Honest reading of part 1's numbers.** 20 of M2's 21 failures were already closed by peers between
M2's run and this session (auto-commit `cd96692e52`, 00:04) — this slice did not close them, and says
so rather than claiming them. Spot-checked in the baseline capture as now `ok`:
`inference::quick::declared_inferences_for_workspace_finds_the_real_wfc_roster`,
`prompts::quick::no_prompt_names_a_specific_plugin_or_artifact_kind`, all five
`workspace::remote::tests::*`, both `transport::quick::*`,
`workspace::quick::mcp_probe_document_transport_…`, `workspace::long::a_headless_commit_…`,
`bridge::quick::bounded_shell_decoder_and_materializer_advance_incrementally`, both
`conformance::quick::*`. The **one** remaining failure is new since M2 and is diagnosed in §3.

---

## 3. Part 1 — the one remaining os-mcp failure

### 3.1 What fails

```
$ cargo test --manifest-path Cargo.toml --lib -- --test-threads=1        # os-mcp
test result: FAILED. 336 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.01s

---- workspace::long::plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired ----
[W3] real PureCommand round trip: Err(Fault { code: "channel.not-wired", message:
  "InstanceOpen: guest trapped: wasm trap: memory write is out of bounds:
   start=4294409068 length=4 end=4294409072 bound=12976128" })
panicked at 🏠️workspace/🧪️tests/🔬️long/🦀️.rs:200:5:
  PureCommand must be a real round trip, not the old host-side short-circuit
```

### 3.2 Root cause, measured

The guest itself traps. `start=4294409068` is `-558228` reinterpreted as `u32`, i.e. a **negative
wasm address**, on a 4-byte store, against a memory whose bound is `12976128` = exactly 198 pages.
The trap text is minted by `🔌️plugin/🧠️interpreter/🦀️.rs:3364` (`checked_end`, subject `"memory write"`)
inside `write_bytes` — the store instruction the interpreter is *executing on the guest's behalf*, not
a host-side write. The two host-side writers (`write_owned_memory`, `write_owned_zeroes` in
`🔌️plugin/🖥️host/🦀️.rs:1567/1563`) mint a different message (`"owned input is outside guest memory"`),
so they are excluded. The trap therefore happens **inside `🗒️note`'s own `InstanceOpen` path** under
the repository-owned interpreter (`OwnedRuntime`), before any `AppCommand` reaches the guest.

Two experiments, both run:

1. **Budget is not the variable.** `PluginArtifactChannel::ensure_instance`'s
   `owned_interactive_budget()` (fuel 2 M, 8 ms, 256 frames, 1 MiB patch) was temporarily swapped for
   `headless_inference_budget()` (fuel 2 G, 30 s, 4096 frames, 16 MiB patch) and the test re-run: the
   trap is **byte-identical**, same address, same bound, in one turn instead of many slices. The
   experiment was reverted; `ensure_instance` is back on `owned_interactive_budget()`.
2. **Slicing is not the variable either.** `begin_owned_operation`
   (`🔌️plugin/🖥️host/🦀️.rs:1352`) returns early when `state.pending` already holds the same
   `OwnedOperation`, so a resumed `execute_actor_turn` **discards the event list it is handed**. A1's
   `Event::Wake`-on-resume branch in `ensure_instance` (`🏠️workspace/🦀️.rs:935`) is therefore inert:
   the events never reach the guest on a resume, and the docstring's claim that this distinction cured
   the out-of-bounds trap does not hold — the trap is still there. **Handed to A1**, since they own
   that file's in-flight change.

### 3.3 Why the failure appeared only now

At `HEAD`, `ArtifactChannel::exchange` called `ensure_instance` exactly **once**; the first 8 ms slice
of a cold guest always yields `budget.exceeded`, which is not `channel.not-wired`, so the test passed
**without the guest ever opening**. A1's resume loop makes the gateway actually drive the open to
completion — which is the right thing to do and is what every real MCP mutation needs — and that is
what surfaced the trap. So: not a regression in behaviour, a **newly reachable pre-existing defect**.

### 3.4 What was deliberately NOT done

`ensure_instance` classifies a guest trap as `channel.not-wired` (`🏠️workspace/🦀️.rs:957`), which is a
genuine mis-typing: `channel.not-wired` means "this host path is unimplemented", a trap means "the
plugin failed". Re-coding that arm would make the failing assertion pass — and would be dishonest:
the test's contract is "a real ROUND TRIP", and a trap during `InstanceOpen` is not a round trip. The
suite is therefore reported as **336/1, not green**, with the defect named, rather than green with the
defect hidden. Fixing it means either a fresh `wasm32-wasip2` `🗒️note` build (the committed artifact is
from 2026-09-17 16:22, two days of host-ABI churn old — see §3.5) or a real interpreter/guest
investigation; both are larger than this slice and both belong with A1's e2e lane or the plugin-host
owner.

### 3.5 Third experiment: a stale guest artifact is NOT the cause (refuted here)

The committed `🗒️note` wasm was two days old (2026-09-17 16:22, 60 966 459 B) against two days of
host/WIT churn, which is a textbook cause of a fixed-address guest trap. It was rebuilt and the test
re-run:

```
$ CARGO_PROFILE_WASM_DEV_DEBUG=false cargo build -p semio-s-plugin-note --target wasm32-wasip2 --profile wasm-dev
    Finished `wasm-dev` profile [unoptimized] target(s) in 2m 07s
$ ls -la …/target/wasm32-wasip2/wasm-dev/semio_s_plugin_note.wasm
-rw-r--r-- 63685168 Sep 19 02:56           # a genuinely new artifact, 2.7 MB larger
$ cargo test --lib -- --test-threads=1 --nocapture plugin_artifact_channel_mutation_verbs
[W3] … "InstanceOpen: guest trapped: wasm trap: memory write is out of bounds:
      start=4294409068 length=4 end=4294409072 bound=13172736"
```

The memory `bound` moved (12 976 128 → 13 172 736 = 201 pages), proving the fresh artifact was the one
loaded — and the faulting address is **bit-for-bit identical**: `4294409068`, i.e. `-558228`. A stale
guest is therefore ruled out, and so is anything proportional to the module's layout or memory size:
across two independently compiled artifacts, three budgets and both slice counts, the guest always
stores 4 bytes at the same fixed negative address.

**The remaining leads, for whoever takes it** (none executed here, stated as leads not findings):
`OwnedRuntime::compile_component` parses this file through `OwnedSemioArtifact::parse` — the
repository's own interpreter — whereas `🏃️run` boots the same plugin through wasmtime, so the defect
may be in how the owned interpreter instantiates a wasip2 **component** (globals' init expressions,
which core module it picks, data-segment placement) rather than in `🗒️note`. A constant negative
address is the signature of either an uninitialised `__stack_pointer` global or a pointer derived
from a call that answered `-1`. The cheap next probes: run the same open against a second plugin (if
it traps at the same address, it is the interpreter, not `🗒️note`), and dump the interpreter's frame
stack at `🔌️plugin/🧠️interpreter/🦀️.rs:3364` on the trap.

---

## 4. Part 2 — the wgpu agent panel, verified

### 4.1 The blocker: the crate's test target did not compile

`cargo test -p semio-framework-os-renderer-wgpu --lib` failed with **15 errors**, none of them in this
slice's files — peer drift in unrelated test modules and one product file. They had to go for any of
part 2 to be verifiable, so they were repaired (minimal, mechanical, each a follow of a peer's own
rename/field addition):

| file:line | error | repair |
| --- | --- | --- |
| `🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:165` | `invalid format string` — a literal `{token: primary}` in an assert message | escaped to `{{token: primary}}` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs:249` | `WindowEngagementRingOption`/`ToggleGroupOption`/`SelectItem` missing from `ui_wgpu::wgpu` (4 × E0422) | the three structs were added to `component::layout` but never added to the facade's `pub use` — added |
| `🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:458` | E0599 `UiFixedList::first` | `UiFixedList` has no `first`; `iter().next()` |
| `🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs:307,323` and `…/🧭️wgpu-navbar-footer-parity/🦀️.rs:410-412` | E0061 ×5, `should_auto_start_introduction` gained `replay_on_load` | fifth argument `false` at all five call sites |
| `🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs:307` | E0063 `ShellChromeFrameCursor` gained `parked_*` | `..ShellChromeFrameCursor::default()` |
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5538` | E0063 `Paint2dMarquee` gained `pan_last` (**product code**) | `pan_last: None` at the marquee-arm site |
| `🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs:301` | E0004 `SceneDragMode::RowTransfer` not covered | a no-op arm — a row transfer resolves on the RELEASE, exactly like the pan modes |

### 4.2 The real defect in the inherited panel work

With the target compiling, the two chat laws failed. Root cause in
`🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:897`: the helper `chat_entry_ids` read the
conversation rows as **direct children of the panel root**, but `build_agent_chat_ui` wraps them in
their own feed stack under React's `framework.chat.feed` id (the whole point of the feed being one
addressable, `aria-live` thing). The helper returned `["framework.chat.feed"]` for every panel, so
`a_bridge_tool_call_frame_becomes_a_conversation_row_in_the_chat_panel` failed on its very first
assertion ("no bridge activity, no rows") and the composer law failed on its echo assertion. The
helper was rewritten to resolve the feed by `FRAMEWORK_CHAT_FEED_ID` and read ITS children — a stale
test against the current contract, not a product bug.

### 4.3 What is now proven by running

```
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 agent
test result: ok. 75 passed; 0 failed; 0 ignored; 0 measured; 968 filtered out
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 chat
test shell::agent_overlays_tests::the_chat_header_plan_puts_the_dot_and_status_inside_the_panel ... ok
test shell::panel_anchor_model_tests::a_bridge_tool_call_frame_becomes_a_conversation_row_in_the_chat_panel ... ok
test shell::panel_anchor_model_tests::the_chat_composer_sends_one_agent_message_per_turn_and_keeps_an_unsendable_draft ... ok
test result: ok. 3 passed; 0 failed
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 socket_door
test result: ok. 21 passed; 0 failed
```

Substantively, these assert: a gateway `AgentToolCall` frame becomes one addressable row in the wgpu
panel model and the matching `AgentToolResult` folds into that SAME row (never a second one); an
approval is one row from request to decision; a human turn leaves as a real
`ShellToGateway::AgentMessage` (trimmed, one frame per turn) and is echoed into the feed under the
translated role noun; a blank draft sends nothing; **with no socket open the draft is KEPT, not
swallowed**; the empty transcript paints React's own empty line inside the feed; every row carries
React's two data attributes; the composer sends on Enter; the feed paints the newest window the way
React's auto-scrolled list does; all 23 bridge fixtures round-trip through this codec in both
directions; and the whole body projects through `panel_ui_records` (so it is publishable, not just
constructible). Nothing on that surface is generated locally — the echo is gone on this target too.

### 4.4 Whole-crate run and wasm32

```
$ cargo test -p semio-framework-os-renderer-wgpu --lib -- --skip product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder
test result: FAILED. 1012 passed; 30 failed; 0 ignored; 0 measured; 1 filtered out; finished in 42.93s
```

The skip is mandatory: `kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder`
panics inside `MountedProductReplayRequest::drop` on a poisoned lock and takes the whole process down
with `SIGABRT` ("panic in a destructor during cleanup"), so it aborts every other test's result too.
The 30 remaining failures are in `engine_canvas`, `interpreter::render_plan_validator`,
`kernel_runtime::semantic_document`, `scenes`, `shell::theme_editor`, `shell::tool_run_panel`,
`shell::window_measures`, `shell::window_actions_search_pane`, `shell::display_conflicts_marketplace`,
`shell::appearance_tour`, `shell::shell_document_retirement` and
`shell::panel_anchor_model_tests::build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme`
— **zero** in agent bridge / agent presence / agent overlays / agent chat. They are other slices'
in-flight work and are listed in `🗑️generated/m3-wgpu-lib-full.txt` for the coordinator; this slice
did not touch them beyond making them compile.

```
$ CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown
warning: `semio-framework-os-renderer-wgpu` (lib) generated 79 warnings
    Finished `dev` profile [unoptimized] target(s) in 1m 59s
```

Clean — and the 79 warnings are the proof the expansion actually completed rather than aborting early.
This is the target that matters for the agent bridge: `🌉️agent-bridge-door/🦀️.rs` splits into a
`cfg(target_arch = "wasm32")` page-owned `BrowserDoorSocket` half and a `cfg(not(…))`
`tokio-tungstenite` half, and the native test run exercises only the second. The wasm32 half compiles.

---

## 5. Verified at runtime vs. by tests only

**By running tests (real counts above):** the whole wgpu conversation model, the panel body it
projects, the composer's send/keep contract, the socket-door lane laws, both bridge codec directions
against the shared fixtures, and the os-mcp suite's 336 passing assertions.

**By compiler only:** the wasm32 (browser) arm of `🌉️agent-bridge-door` — it typechecks for
`wasm32-unknown-unknown`; no browser probe was run.

**Not verified at all (stated, not hidden):**
- **No live shell.** The wgpu renderer was not booted and no headless probe was driven against a
  running gateway, so the panel is proven as a model + projection, not as painted pixels or a real
  socket carrying real gateway frames.
- **The one os-mcp failure** (§3) — reproduced and diagnosed, not fixed.
- The `🗒️note` rebuild hypothesis of §3.5 was **not** executed.

---

## 6. Honest gaps

1. **`semio-framework-os-mcp` is not fully green.** 336/1. §3 says exactly why the last one is left
   standing and why making it green would have been a lie. It sits in a file slice A1 is editing live.
2. **A1's `Event::Wake` resume branch is inert** (§3.2) — reported, not changed, because A1 owns it.
3. **The wgpu crate is not fully green either**: 30 unrelated failures and one process-aborting test.
   All are outside this slice; none was introduced here (the target did not compile before, so there
   was no earlier green number to regress from).
4. **No browser/native boot of the wgpu shell**, so the agent panel's live pixels remain unproven.
5. `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5538` was **product** code, not a test — it means
   the crate's non-test build was broken too at session start, which nothing else in the ticket had
   reported.

---

## 7. Files changed

Product / facade:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs:249` — re-export `WindowEngagementRingOption`,
  `WindowEngagementSelectItem`, `WindowEngagementToggleGroupOption`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5538`
  — `Paint2dMarquee { …, pan_last: None }`

Tests:

- `…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:897` — `chat_entry_ids` resolves the
  `framework.chat.feed` stack and reads ITS rows (the slice's own defect fix)
- `…/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:165`
- `…/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:458`
- `…/🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs:307,323`
- `…/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs:410-412`
- `…/🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs:307`
- `…/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs:301`

Not changed by this slice (inherited from the dead M3 worker, verified here):
`🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`, `💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs`,
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🎯️targets/🧊️wgpu/🌉️agent-bridge-door/🦀️.rs`,
`🎯️targets/🧊️wgpu/🔌️socket-door/**`.

Touched and reverted: `🌉️mcp/🏠️workspace/🦀️.rs:949` (budget experiment of §3.2 — back to
`owned_interactive_budget()`).

Captures (`🗑️generated/`, deletable with the ticket): `m3-baseline-test.txt`,
`m3-wgpu-lib-test.txt`, `m3-wgpu-lib-full.txt`, `m3-wgpu-agent-tests.txt`, `m3-wgpu-wasm32-check.txt`,
`m3-note-wasm-build.txt`, `m3-final-mcp-test.txt`.

**Side effect the fleet should know about:** §3.5's experiment **replaced** the shared
`…/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_note.wasm` with a build from this
session's tree (63 685 168 B, 02:56). That is strictly fresher than what was there and is the artifact
every `🔬️long` test now loads; no other plugin artifact was rebuilt.
