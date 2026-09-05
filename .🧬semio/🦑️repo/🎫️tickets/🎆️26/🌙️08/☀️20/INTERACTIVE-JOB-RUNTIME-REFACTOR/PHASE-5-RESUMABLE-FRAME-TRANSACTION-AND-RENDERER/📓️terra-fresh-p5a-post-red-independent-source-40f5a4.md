# Terra Fresh P5a Post-RED Independent Source/Static Audit — 2026-08-24

## Verdict

**RED.** The current source is not admissible for P5a. Three independently
reachable production counterexamples remain, and the claimed P5a static
mutation suite has material false-green holes. P5b/P5c isolated semantic
verifiers still pass, but their scoped Rust formatter census is not clean.

This was a read-only source/static audit. No production source, test, contract,
verifier, or existing report was modified.

## Scope and governing material

Read in full:

- `AGENTS.md`;
- `📓️p5-frame-transaction.md`, the P5a mounted-frame repair contract, and
  the P5b/P5c reconcile/layout repair contracts;
- all three coordinator P5a pre-acceptance RED reports;
- `📓️p5a-third-independent-source-static-audit-2026-08-24.md`;
- `📓️sol-p5a-mounted-frame-transaction-implementation-2026-08-24.md`;
- the relevant P5b/P5c coordinator and Terra acceptance reports;
- live P5a/P5b/P5c Rust and Shell source, their focused tests, and the exact
  P5a/P5b/P5c verifier regions in repository-root `📜️script.ts`.

The audit deliberately traced the current production callees rather than
treating token presence in the static verifier as proof of incremental work.

## P5a B1 — mounted UiNode output transaction

The retained direct node painter is broadly cursorized:

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs:559-982`,
  `paint_node_step`, contains arms for Text, Stack, Separator, Button, Input,
  Select, Toggle, KeyValue, Slider, NumberStepper, Ring, IconSelect, Field,
  Section, Group, Tree, Image, ComponentScene, and ExternalSlot.
- Select's retained cursor is at `paint.rs:641-718`; KeyValue's is at
  `paint.rs:745-778`; Tree's retained cursor is at `paint.rs:273-525`.

That direct route is not the complete mounted call graph. On every mounted
Synchronize visit, `frame_into_step` calls:

```text
engine.rs:975 / 1123 frame_into_step
  -> sync_interactive_state_node(&mut window.tree, node, &theme)
       paint.rs:1057 sync_interactive_state_node
       -> paint.rs:1093 sync_select_popup_rows
       -> paint.rs:1110 sync_tree_row_layout
          -> paint.rs:1143 sync_tree_item_layout (recursive)
```

Exact violations:

- At `paint.rs:1058-1064`, an open `UiNode::Select` clones the complete
  `select.items` collection before calling `sync_select_popup_rows`.
  `sync_select_popup_rows` iterates every supplied item at
  `paint.rs:1093-1103`.
- At `paint.rs:1077-1079`, Tree synchronization calls
  `sync_tree_row_layout`. That helper clones the complete tree node at
  `paint.rs:1111-1114`, visits every section/item at
  `paint.rs:1119-1124`, and recursively walks expanded nested items at
  `paint.rs:1143-1167` (the nested-item loop is
  `paint.rs:1154-1155`).

Thus one mounted worker grant can clone/materialize and traverse a whole Select
or recursive Tree before the retained painter is reached. This violates the
one hard-admitted output/scalar/collection/depth unit requirement and the
ban on uncredited dynamic materialization. It is reachable from the normal
mounted frame path; it is not the test-only legacy `paint_tree` path.

The P5a verifier does not cover this callee body. Its
`paintNodeBoundary` ends before `paint_tree` at
`📜️script.ts:8339-8341`, while its UI-frame check merely requires the token
`sync_interactive_state_node` at `📜️script.ts:8571-8587`.

## P5a B2 — live Shell text transaction

Dialog and tour retained subroutines have the intended scalar/glyph shape:

- `🧩️frontend/🧩️shell/🦀️component.rs:6290-6303`,
  `chrome_text_step` / `chrome_text_complete_step`, advance retained text;
- dialog stepping begins at `component.rs:10839`;
- tour stepping begins at `component.rs:10894`; it uses
  `chrome_text_complete_step` at `component.rs:10911-10960` and returns
  false while the text work is pending.

Nevertheless the live mounted Shell dispatcher retains reachable legacy
whole-string seams:

```text
component.rs:9817-9821 render_chrome_step -> render_tutorial_bar_step
component.rs:9829-9833 render_chrome_step -> render_overlay_step
component.rs:9843-9851 render_chrome_step -> error Chrome path
```

- `render_tutorial_bar_step` calls legacy `chrome_text` at
  `component.rs:10549` and measures the entire label immediately afterward
  at `component.rs:10550`.
- `render_overlay_step` calls legacy `chrome_text` at
  `component.rs:10688` and `component.rs:10690`.
- `chrome_text`, `component.rs:6281-6287`, constructs dynamic maps and
  invokes whole-string `draw_text`.
- Additional direct legacy sites are present at `component.rs:6386, 6561,
  9258, 10423, 10761, 10829`; direct `draw_text` is also called at
  `component.rs:9849`.

Consequently the required global condition—no reachable
`chrome_text`/`draw_text` whole-string seam—is false, even though the
dialog/tour-specific retained functions look incrementally structured. The
P5a verifier slices only from `fn chrome_text_step` to `fn chrome_icon`
at `📜️script.ts:8445-8559`, so it omits these live callers.

## P5a B3 — generation-qualified maintenance and native preferences

The ordinary accepted/refusal path has several correct elements:

- `FrameMaintenanceAuthority` at
  `🧰️framework/🔨️modules/🖥️renderer/📦️packages/🦀️rust/🔌️glue.rs:8489-8519`
  performs generation-qualified atomic claim/release.
- `RuntimeApply::start_frame_deferred`,
  `glue.rs:8582-8660`, preserves the exact owner and cancels the completion
  reservation when direct submission refuses.
- `RuntimeMailbox::try_spawn_frame_maintenance_reserved`,
  `glue.rs:9411-9466`, uses shared-pool `try_submit(Lane::Io, job)`;
  it returns the exact owner on immediate refusal at `glue.rs:9459-9465`.
- The worker applies cancel/stale/deadline checks before work at
  `glue.rs:9437-9440`, advances one maintenance step at `glue.rs:9444`,
  then checks deadline at `glue.rs:9448-9451`.
- `FrameDeferredCursor::close_step`, `glue.rs:8321-8403`, normally drains
  one retained action/pump/tutorial owner per close step.

The accepted-job interruption/Drop case is still unsound:

- `FrameMaintenanceOwner`, `glue.rs:8406-8436`, owns
  `AppInteractionState`, `FrameDeferredCursor`, and the deadline, but has
  no `Drop` / retained `begin_close` / terminal recovery protocol.
- `FrameMaintenanceOwnerCell`, `glue.rs:8439-8473`, likewise has no Drop
  recovery; nor do the refusal/authority wrappers.
- After successful `try_submit`, the only holder is the captured job's
  `Arc<FrameMaintenanceOwnerCell>`. If the lane queue or worker is shut down
  after acceptance but before invoking the job, dropping that closure drops
  the owner recursively and leaves its authority reservation live. No
  terminal handback or completion-reservation recovery executes.

This is a direct violation of the required Drop/interrupted-owner recovery,
not merely an untested branch.

Production native preferences retain the accepted bounded design:

- fixed per-key pages are read at `component.rs:13311-13322` and written at
  `component.rs:13325-13337`, both using
  `SHELL_CHROME_IO_FIELD_BYTES` (4 KiB);
- bounded get/set are `component.rs:13359-13370`;
- the old mutex-backed whole-config JSON store is test-only
  (`component.rs:13119+`, with full config helpers at
  `component.rs:13271` and `13280`).

That production preference preservation does not repair the accepted-job Drop
hole.

## Already accepted find and atlas source

The fixed, generation-qualified Shell find authority remains statically
preserved in `component.rs:132-263`: fixed boxed capacity, exact byte
accounting, generation qualification, bounded refusal, one-item close, and
thread-local non-nesting are all present.

Prepared atlas permits remain statically preserved:

- atomic item/page/payload/backing acquisition:
  `🧰️framework/🔨️modules/🖥️renderer/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs:195-268`;
- reservation before fixed page-slot allocation:
  `prepared.rs:321-345`;
- one-page push: `prepared.rs:347-363`;
- one-unit terminal close: `prepared.rs:390-417`;
- pre-reserved Drop abandonment and one-unit drain:
  `prepared.rs:421-453`;
- mounted deferred drain: `glue.rs:11661`.

No new source counterexample was found in those accepted authorities.

## P5b/P5c preservation

The isolated P5b verifier passed and the isolated P5c verifier passed. P5c's
verifier directly mutates the accepted-layout guard inside `paint_node_step`
at `📜️script.ts:8245`:

```rust
let Some(layout) = tree.accepted_layout(id) else {
    return RetainedNodePaintStep::Fault;
}
```

and rejects the default-layout replacement. This confirms the accepted-layout
semantic guard remains live under the P5c static suite. The P5b reconciliation,
patch, and reactor static suite also remained green.

That preservation is qualified: the scoped Rust formatter census currently
fails for P5b/P5c source. Reported formatting diffs include:

- `ui/runtime/reconcile.rs:3216,3249`;
- P5c `component.rs:488,1662,1669,1910,2804,3354`;
- `widgets.rs:1,9,219,282,697,721`;
- `mounted_layout.rs:712,723`;
- `tree.rs:5,354,505`;
- `events.rs:143,1351`.

Therefore the P5b/P5c functional static verifiers preserve their accepted
semantics, but their requested scoped rustfmt preservation gate is not green.

## Independent live-callee mutation fidelity

The P5a suite declares 81 mutations at `📜️script.ts:8774-8856`. Baseline
P5a selftest passes all declared mutations, but the following five actual
live-callee mutations were applied only to in-memory source strings and all
incorrectly produced no failures:

| Live mutation | Result |
| --- | --- |
| Add `select.items.iter().collect::<Vec<_>>()` to the live Select arm in `paint_node_step` | `B1-select-whole-materialization: FALSE-GREEN` |
| Add `tree.children(id).collect::<Vec<_>>()` to `sync_interactive_state_node` | `B1-sync-whole-child-materialization: FALSE-GREEN` |
| Add `text.chars().count()` immediately before `paint_retained_glyph_step` in `chrome_text_step` | `B2-glyph-whole-string-count: FALSE-GREEN` |
| Replace one-action `FrameDeferredCursor::close_step` pop with bulk `FrameActionOwners::default()` | `B3-bulk-deferred-close: FALSE-GREEN` |
| Replace `FrameMaintenanceAuthority::release` generation/atomic release logic with `true` | `B3-authority-release-erasure: FALSE-GREEN` |

These are independent direct callee boundaries, not edits to fixtures. They
show that “all 81 P5a mutations are faithful” is false. Several declared
mutations are also intrinsically weak: `dynamic-engine-packets` and
`dynamic-atlas-slots` add unused vectors, `whole-icon-clone` adds an
unconnected allocation, and `chrome-loop` is a token-level replacement
without a valid loop body. Passing those cases cannot establish a live
incrementality property.

## Commands and observed gates

Only allowed read-only static commands were run:

```text
bun -e '<import root 📜️script.ts; invoke isolated P5a/P5b verifier>'
P5a isolated verifier: PASS
P5b isolated verifier: PASS

bun -e '<import root 📜️script.ts; invoke isolated P5c verifier>'
P5c isolated verifier: PASS

bun -e '<read exact live source strings; inject five mutations in memory; invoke P5a verifier>'
five results above: FALSE-GREEN

rustfmt --edition 2024 --check --config skip_children=true <scoped P5a sources>
PASS

rustfmt --edition 2021 --check --config skip_children=true <scoped P5b/P5c sources>
FAIL (locations listed above)

git diff --check -- <scoped P5a/P5b/P5c sources and root 📜️script.ts>
PASS
```

No Cargo, Nx, Wasm, browser, broad build, or runtime gate was run.

## Exact blockers to acceptance

1. **B1:** `engine.rs:975,1123` reaches
   `paint.rs:1057-1167`, where mounted Synchronize performs whole Select
   clone/iteration and full recursive Tree clone/traversal per grant.
2. **B2:** `component.rs:9817-9851` reaches legacy whole-string
   `chrome_text`/`draw_text` at `component.rs:6281-6287, 9849, 10549,
   10688, 10690`.
3. **B3:** `glue.rs:8406-8473` lacks any accepted-job interruption/Drop
   recovery, so queue/worker shutdown after accepted `try_submit` loses the
   owner and strands authority/completion state.
4. **Verifier fidelity:** all five live-source counterexample mutations above
   are false-green despite the claimed 81 P5a mutations.
5. **Preservation gate:** scoped P5b/P5c rustfmt check currently fails at the
   stated locations.

