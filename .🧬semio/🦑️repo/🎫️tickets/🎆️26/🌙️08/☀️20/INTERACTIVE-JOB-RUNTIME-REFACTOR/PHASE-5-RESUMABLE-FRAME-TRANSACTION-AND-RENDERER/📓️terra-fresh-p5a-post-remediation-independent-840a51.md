# Terra Fresh P5a Post-remediation Independent Source/Static Re-audit — 2026-08-24

## Verdict

**RED.** P5a's new retained Select/Tree synchronizer, mounted dynamic chrome
subcursors, maintenance handback registry, find authority, atlas authority, and
the declared P5a mutation suite are materially present.  However, the exact
legacy chrome requirement is not met: a whole-string legacy group measurement
helper remains production-compiled without `#[cfg(test)]`.  In addition, the
requested P5b and P5c edition-2021 `rustfmt --check` preservation gates fail.

This was a read-only source/static audit.  No Rust, verifier, production source,
test, Cargo, Nx, Wasm, browser, build, or runtime command was changed or run.
The only artifact created is this report.

## Scope read

Read in full:

- repository `AGENTS.md`;
- Phase-5 master contract and P5a/P5b/P5c repair contracts;
- the fresh P5a RED audit, P5a implementation report, P5b final Terra
  acceptance, P5c coordinator source acceptance, and relevant live sources;
- exact P5a/P5b/P5c regions in root `📜️script.ts`;
- live Shell, UI-WGPU paint/engine/prepared, and renderer glue source and the
  focused hostile Rust laws.

## Blocking P5a counterexample: legacy whole-string oracle is not test-only

`Shell/component.rs` correctly marks the direct legacy text renderer as test
only:

```rust
#[cfg(test)]
fn chrome_text(...) {
    // ...
    draw_text(&mut ctx, text, ...);
}
```

But the same production compilation unit subsequently retains both of these
unconditional legacy helpers:

```text
component.rs:6551 fn measure_chrome_group_item(...)
component.rs:6553 atlas.measure_text(label, theme.font_size_small)
component.rs:6680 fn render_chrome_group(...)
component.rs:6689 measure_chrome_group_item(atlas, theme, item)
```

Neither function has an immediately preceding `#[cfg(test)]`.  The former is
an actual whole-string measurement primitive; the latter is the old complete
group renderer with an item iterator.  Their existing callers are legacy/test
routes, but they are still production-compiled definitions, contrary to the
explicit P5a condition that *all* whole-string legacy helpers/oracles be both
`cfg(test)`-only and production-unreachable.  A dead production definition is
not a test-only oracle.

The permanent P5a predicate has a matching coverage hole.  Its
`chromeGroupBoundary` begins at `enum RetainedChromeGroupStep` and ends before
`fn measure_chrome_group_item`; the check therefore verifies the new retained
group helper but excludes the live production-compiled whole-string measure
helper.  `interactivityProductionSource` does not remove either legacy function
because neither has a `#[cfg(test)]` item attribute.  Thus the 81 mutation
baseline cannot establish the required global legacy-helper condition.

Required repair: place the legacy `measure_chrome_group_item` and
`render_chrome_group` oracle family behind `#[cfg(test)]` (and likewise every
legacy-only helper they require), then make the P5a predicate assert their
absence from production source.  The mounted retained paths must continue to
use `retained_chrome_group_item_width` and
`render_retained_chrome_group_item_step`.

## Retained mounted P5a census

### Select/Tree synchronization

The live frame engines import and call
`sync_interactive_state_node_step` at `engine.rs:947` and `engine.rs:1122`.
`RetainedInteractiveSyncCursor` in `paint.rs:1101+` has fixed:

- 256 collection items;
- 512 output records;
- 64 retained depth frames; and
- 256-byte copied keys.

The Select phases retain item/child-scan/write progress.  The Tree route holds
fixed DFS frames and fixed records, advances one item/ascent/record action, and
performs one retained child scan or layout write per later grant.  Source item
storage is addressed only after the cap checks; no recursive mounted helper is
inside the new synchronization boundary.  The stale/fault engine branches call
`node_sync.close_step()` until the cursor is empty before retiring/faulting the
frame.  The cursor close path removes one record, then one frame, then its node
binding per grant.

The old recursive `sync_interactive_state` and
`sync_interactive_state_node` remain after the verifier's new boundary and are
not called by either mounted frame step.  They should remain clearly test-only
or otherwise outside the production graph in a later cleanup, but this audit
found no direct mounted call from `frame_step` or `frame_into_step`.

### Dynamic mounted chrome

Mounted Navbar, TutorialBar, Footer, Overlay, Error, context menu, tooltip,
dialog, and tour now park dynamic labels through `RetainedGlyphCursor` and, for
chrome items, `render_retained_chrome_group_item_step`.  The retained group
cursor emits a background, icon, each glyph, each border, and the hit target in
separate phases.  The direct `chrome_text` / `draw_text` pair is test-only.

This favorable mounted-path result does **not** negate the blocking legacy
measure helper above.

### Maintenance exact-owner handback

Native renderer glue now has a generation-qualified
`FrameMaintenanceExecutionRegistry`, queued/running/abandoned/recovering
states, a queued execution envelope `Drop`, and a running guard `Drop` that
restores the exact owner cell before publishing an incremental recovery wake.
Submission publishes before `try_submit(Lane::Io, ...)`; immediate refusal
uses `reclaim_rejected`, retains the exact cell, and releases the reservation.
`RuntimeApply::start_frame_deferred` polls abandoned execution ownership before
normal work and resumes it through the same retained close path.  The focused
queue-drop and running-interruption laws are present.

Static inspection found no new blocking mutex, dynamic registry, whole-owner
close, or interactive-lane maintenance submission inside this authority.

### Previously accepted authorities

The Shell find owner remains fixed and generation-qualified:
`ShellFindItems` holds 256 boxed optional slots, has fixed payload accounting,
exact `try_push_at` refusal, one-item `pop_front`, one-item `close_step`, and
the nonblocking thread-local active binding.  The exact MAX+1 and stale-owner
laws remain present.

Prepared atlas pages retain atomic item/page/payload/backing permits before
fixed slot allocation, one-page population and close, an abandonment `Drop`
authority, and mounted incremental abandoned-close consumption.  The accepted
atlas interruption law remains present.  No regression was found in either
authority during this static pass.

## Mutation evidence

The direct isolated P5a self-test passed all **81** declared mutations.  I
independently applied the five prior-auditor changes to in-memory live source
strings and passed those strings to
`interactivityMountedFrameTransactionFailures`; each was rejected:

| Direct live mutation | Resulting rejection |
| --- | --- |
| Insert `select.items.iter().collect::<Vec<_>>()` in the actual retained Select arm | `P5a paint node recursively reaches child work .collect::<Vec` |
| Insert `tree.children(id).collect::<Vec<_>>()` in `sync_interactive_state_node_step` | `P5a mounted interactive synchronization retains whole collection/depth work Vec<` |
| Insert `text.chars().count()` immediately before retained Shell glyph paint | `P5a Shell text callee retains whole-string work .chars().count()` |
| Replace one action pop in `FrameDeferredCursor::close_step` with `FrameActionOwners::default()` | `P5a deferred close bulk-drops the populated action owner` |
| Replace generation-qualified `FrameMaintenanceAuthority::release` with `true` | `P5a maintenance authority is not generation/cancel/deadline-qualified with nonblocking exact-owner refusal handback` |

This evidence is positive for the five repaired false-green cases, but it does
not cover the separate `measure_chrome_group_item` boundary described above.

## P5b/P5c preservation and formatting

Both isolated semantic suites pass:

```text
P5a=PASS (81 declared mutations)
P5b=PASS
P5c=PASS
```

The requested formatter preservation is nevertheless RED:

| Gate | Result | Observed source locations |
| --- | --- | --- |
| P5a scoped `rustfmt --edition 2024 --check` | PASS | paint, engine, Shell, renderer glue |
| P5b eight-file `rustfmt --edition 2021 --check` | FAIL | plugin shard `component.rs:166,178,189,1410,2212,2227`; renderer glue also has formatter diffs |
| P5c declared eight-file `rustfmt --edition 2021 --check` | FAIL | UI-WGPU `engine.rs:9,17,25,1355,1366`; `paint.rs` from line 15 onward (and the shared glue scope) |
| Scoped `git diff --check` | PASS | P5a/P5b/P5c verifier and audited source paths |

The P5c failure is not inferred from the P5b command: an independent P5c
command exited `1` and its separate `Diff in` census is recorded above.

## Commands run

```text
bun -e '... interactivityMountedFrameTransactionSelfTests ...
             interactivityLiveReconcileSelfTests ...
             interactivityMountedLayoutTextSelfTests ...'
# P5a=PASS (81 declared mutations); P5b=PASS; P5c=PASS

bun -e '... read 16 live P5a files; apply the five mutations in memory;
             call interactivityMountedFrameTransactionFailures ...'
# five / five rejected, as tabulated above

rustfmt --edition 2024 --check --config skip_children=true <four P5a Rust files>
# PASS

rustfmt --edition 2021 --check --config skip_children=true <eight P5b Rust files>
# FAIL

rustfmt --edition 2021 --check --config skip_children=true <eight P5c Rust files>
# FAIL, exit=1

git diff --check -- <scoped P5a/P5b/P5c source and root verifier paths>
# PASS

rg -n <legacy chrome / find / atlas / retained-sync / maintenance census>
```

No Cargo, Nx, Wasm, browser, build, or runtime gate was run.

## Acceptance disposition

Do not accept P5a or the P5b/P5c requested formatting-preservation handoff
until the two blocking conditions are repaired and the exact source/static
gates are rerun on the quiescent tree.
