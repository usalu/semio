# Terra P5a Narrow Legacy-chrome Re-audit — 2026-08-24

## Verdict

**GREEN at the requested P5a source/static handoff.** The two legacy Shell
whole-string helpers and their entire remaining caller chain are test-only;
the mounted production paths do not reach `FontAtlas::measure_text` through
that family. The live P5a verifier rejects removal of either test boundary and
restoration of the mounted legacy call. The requested P5b/P5c formatter
preservation scopes and all three isolated semantic gates also pass.

There is **no remaining source/static blocker in this narrow audit.**

**RED/deferred only:** Cargo, Nx, Wasm, browser, build, executable Rust
hostile-law, runtime, allocation, worker-count, and timing gates were not run,
by the stated packet restriction. This report makes no claim about them.

## Materials And Live Scope Read

- Repository `AGENTS.md`, Phase-5 master plan, P5a repair contract, P5b/P5c
  acceptance materials, the prior independent P5a RED report, and the current
  P5a implementation report.
- Root `📜️script.ts`, especially `P5aMountedFrameTransaction`, P5b
  `interactivityLiveReconcileSelfTests`, and P5c
  `interactivityMountedLayoutTextSelfTests`.
- Live Shell, UI-WGPU paint/engine/prepared sources, renderer glue, and the
  retained maintenance, find, atlas, and interactive-sync authorities.

No existing report, production source, test, or verifier was edited.

## Legacy Chrome Proof

The live Shell has these exact adjacent attributes:

```rust
#[cfg(test)]
fn measure_chrome_group_item(...)

#[cfg(test)]
fn render_chrome_group(...)
```

`measure_chrome_group_item` contains the legacy
`atlas.measure_text(label, theme.font_size_small)` call. `render_chrome_group`
calls that helper and is likewise test-only. `chrome_group_border`, the sole
legacy rendering support helper, is also test-only.

The complete direct-caller inventory is test-only:

- `render_presence_bar` (`cfg(all(test, not(target_arch = "wasm32")))`);
- `render_footer_utility_nodes`;
- legacy `render_tutorial_bar`, `render_navbar`, and `render_studio_canvas_bars`;
- legacy `render_window_measures_rail`, `render_window_engagement_rail`, and
  `render_window_actions_rail`;
- `render_staged_form` and `render_staged_arg`;
- the internal call from `render_chrome_group` to
  `measure_chrome_group_item`.

The mounted `render_*_step` family—main window, navbar, tutorial, footer,
overlay, dialog, and tour—uses `retained_chrome_group_item_width`,
`render_retained_chrome_group_item_step`, and `chrome_text_complete_step`.
It has no call to either legacy helper. The P5a production-source projection
also removes the two cfg(test) definitions; the clean baseline accepts that
projection.

## Faithful In-memory Counterexamples

I loaded the exact 16 live P5a verifier inputs, changed only the Shell string
in memory, and called `interactivityMountedFrameTransactionFailures`.

| Mutation | Result |
| --- | --- |
| Remove `#[cfg(test)]` from `measure_chrome_group_item` | **REJECTED**: missing explicit test-only oracle, production helper reachability, and `measure_text` inside the retained chrome boundary. |
| Remove `#[cfg(test)]` from `render_chrome_group` | **REJECTED**: missing explicit test-only oracle and production helper reachability. |
| Restore `render_chrome_group(...)` in the live retained chrome fault arm | **REJECTED**: `P5a live Shell child body reaches whole subtree render_chrome_group(`. |

The isolated P5a self-test independently passed its full registered mutation
suite, including these three legacy-chrome mutations.

## Preservation Gates

```text
bun -e 'import { interactivityMountedFrameTransactionSelfTests as p5a,
  interactivityLiveReconcileSelfTests as p5b,
  interactivityMountedLayoutTextSelfTests as p5c } from "./📜️script.ts";
  p5a(process.cwd()); console.log("p5a=PASS");
  p5b(process.cwd()); console.log("p5b=PASS");
  p5c(process.cwd()); console.log("p5c=PASS");'
# p5a=PASS
# p5b=PASS
# p5c=PASS
```

```text
rustfmt --edition 2021 --check --config skip_children=true <P5b nine-file union>
# exit 0
rustfmt --edition 2021 --check --config skip_children=true <P5c eight-file scope>
# exit 0
```

### P5b Nine-file Union

1. `ui/runtime/reconcile.rs`
2. plugin reactor `patches/component.rs`
3. plugin reactor `component.rs`
4. UI contract `document.rs`
5. renderer WGPU `glue.rs`
6. Shell `component.rs`
7. UI-WGPU `component.rs`
8. UI-WGPU `widgets.rs`
9. plugin-host shard `component.rs`

### P5c Eight-file Scope

1. UI-WGPU `mounted_layout.rs`
2. UI-WGPU `engine.rs`
3. UI-WGPU `tree.rs`
4. UI-WGPU `paint.rs`
5. UI-WGPU `events.rs`
6. UI-WGPU `scene_slots.rs`
7. renderer Interpreter `component.rs`
8. renderer WGPU `glue.rs`

## Retained-authority Regression Census

- Interactive Select/Tree synchronization remains mounted through
  `sync_interactive_state_node_step` with `RetainedInteractiveSyncCursor`;
  the P5a baseline and its live whole-collection mutations pass.
- `FrameMaintenanceExecutionRegistry` remains the generation-qualified,
  nonblocking exact-owner handback authority. P5a baseline checks retain its
  queued/running/abandoned recovery and `Drop` handback predicates.
- `ShellFindItems` remains fixed-slot, generation-qualified, thread-local,
  one-item admission/close authority; the P5a baseline passes its MAX+1 and
  stale-owner laws.
- `PreparedAtlasPages` remains behind the atomic process-permit and one-page
  close/abandonment boundary; the baseline passes the interrupted-close and
  abandonment-drain predicates.

## Diff And Scope Hygiene

```text
git diff --check -- <root verifier and P5a/P5b/P5c audited inventory>
# exit 0
git diff --cached --check -- <same inventory>
# exit 0
```

The scoped working name-status contains modifications only; the scoped cached
name-status is empty. No audited source file is deleted. This audit's only
write is this report.

## Deferred Gates

Cargo, Nx, Wasm, browser, build, runtime, allocation, worker-count, replay,
and measured timing gates remain deferred and must be run only after the
coordinator declares the overlapping source tree quiescent.
