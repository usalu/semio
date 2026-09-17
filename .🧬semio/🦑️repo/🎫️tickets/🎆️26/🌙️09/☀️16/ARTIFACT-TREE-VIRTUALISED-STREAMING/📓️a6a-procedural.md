# 📓️ A6a — 🌀️procedural (generation2d + generation3d)

Packet A6a of ARTIFACT-TREE-VIRTUALISED-STREAMING. Crates
`semio-s-artifact-procedural-generation2d` and `semio-s-artifact-procedural-generation3d`
(`✏️s/🔌️plugins/🌀️procedural/`). This packet resumed a dead A6 session that had migrated most of the
two apps' panels and left no report; the stale logs under `🗑️generated/a6/` are superseded by
`🗑️generated/a6a/`.

## 1. State found on disk (previous A6 session, verified by reading the code)

Already migrated and left compiling-but-unverified:

| Surface | File | Shape |
| --- | --- | --- |
| gen2d artifact tree | `🌀️generation2d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | `window_section_or_placeholder` over `host_snapshot.widgets`, rows keyed by raw widget id + `granularity`, one tree-level `interaction_domain("graph")` |
| gen2d catalogue | `🌀️generation2d/…/📌️panels/🛍️catalogue/🦀️.rs` | four `window_section`s (sources / components / sinks / modes) |
| gen3d artifact tree | `🧊️generation3d/…/📌️panels/🗿️artifact/🦀️.rs` → `flow::graph_outline` | two `window_section_or_placeholder`s (nodes / wires) + one `tree_window_item` per node for its ports |
| gen3d catalogue | `🧊️generation3d/…/📌️panels/🛍️catalogue/🦀️.rs` | `window_section` over the catalogue sections, one `tree_window_item` per group; `CATALOGUE_GROUP_ROWS` / `panel_continuation_row` / `PanelRowBudget` / `catalogue_page()` all gone |
| gen3d mode-panel fixture | `🧊️generation3d/…/✳️any/🧫️fixtures/📌️mode-panel-publication.json` | Artifact row re-pointed from `procedural-play-document` to `procedural-play-graph` + `.nodes`/`.wires` (this is why the stale `gen3d-mode-panels.txt` shows one failure — the fixture edit was on disk but had not been re-run) |

## 2. What this packet changed

**The generations roster — the one tree both apps still built unwindowed.** It is document-derived
and unbounded, and every row costs three `UiValue` argument maps (select + rename + remove), so it is
the most arena-expensive tree in either app.

- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs` — `generation_tree` takes a
  `&TreeWindows<'_>`; the `for` loop that pushed into a `UiFixedList` became a row closure fed to
  `PanelTreeBuilder::window_section_or_placeholder` over `&generation.generations`. The one-row
  `.actions` section stays a plain `.section` (a fixed add-row, no extent to publish). This file is
  `#[path]`-mounted into BOTH artifact crates, so one edit migrates both apps.
- `🧊️generation3d/…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` — `render` takes the windows;
  new `GENERATION_3D_PLAY_GENERATE_PREFIX` / `GENERATION_3D_PLAY_GENERATIONS_SECTION` consts.
- `🌀️generation2d/…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` — same, with
  `GENERATION2D_PLAY_GENERATE_PREFIX` / `GENERATION2D_PLAY_GENERATIONS_SECTION`.
- `🧊️generation3d/…/✏️editor/🦀️.rs` and `🌀️generation2d/…/✏️editor/🦀️.rs` — the
  `…_BODY_GENERATIONS` arm of each `render_body` now builds
  `TreeWindows::for_body(view_state, <that body key>)`.

**Tests added**

- `🧊️generation3d/…/🗂️generations/🧪️tests/🔬️unit/🦀️.rs` — laws (a)/(d), (b), (c) on the roster
  container, seeded through the real `addGeneration` command (never a hand-built `GenerationPlayState`).
- `🌀️generation2d/…/🗂️generations/🧪️tests/🔬️unit/🦀️.rs` — the same three laws plus a
  `seed_generations` helper.
- `🧊️generation3d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` and
  `…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — `every_windowed_container_carries_a_distinct_node_key`,
  for F2's new "duplicate `node_key` within one body is a loud SDK error" rule. These are the two
  bodies where a key could realistically collide: the catalogue derives group keys from registered
  `CatalogueSection` ids, and the outline mixes authored `procedural-play-graph.*` keys with RAW
  widget ids for the per-node port windows.

## 3. Audit results (grep, whole `🌀️procedural` plugin)

- `setPanelPage` / `set-panel-page` / `panelPage` / `panel_pages`: **zero hits** anywhere — command
  dirs, enums, dispatch, publication contracts, `toolIds`, config + window-config + window-transient
  structs, the generated `🧬️schema/{🔣️.json,🔗️.graphql,🟦️.ts,🛰️.proto}`, the plugin manifest
  `✏️s/🔌️plugins/🌀️procedural/🔣️.json` and every fixture. Matches `📓️audit-app-panels-a.md` §6/§7:
  procedural never had a navigable page cursor.
- `paged_panel_section` / `panel_continuation_row` / `panel_page_rows` / `PanelRowBudget` /
  `CATALOGUE_GROUP_ROWS` / `SECTION_ROWS` / `IDS_ROWS` / `LIST_ROWS_MAX` /
  `PANEL_RECONCILE_NODE_BUDGET` / `section_page`: **zero hits**.
- No crate-local `fn ui_node_list` copy survives — both apps import the SDK's.
- Remaining `.take(N)` hits are `Option::take` and one `matrix.iter().take(6)` in the DSL
  *schema* projection (not a UI truncation).
- Remaining `.more` hits are the settle-latch `step.more` field in `🟦️.ts` probes and the window
  laws' own `!json.contains(".more\"")` assertions.

### Deliberately NOT windowed (matches the fleet's convention)

The two inspection panels (`🌀️generation2d/…/📌️panels/🔍️inspection/🦀️.rs` ≤3 rows,
`🧊️generation3d/…/📌️panels/🔍️inspection/🦀️.rs` ≤4 rows per widget kind) keep plain `.section`
calls over SDK `ui_node_list`. They are selection-derived, bounded by the widget variant, and carry
the slider's editable control as a tree-row child — a shape `window_section`'s `entries`/`row` split
cannot express without inventing a descriptor enum. The peer packets left the equivalent bounded
inspector sections alone in 🔋️energy, 📐️cad and 🧩️puzzle-3d/5d, so this is the fleet convention,
not an omission. Same for `generation_tree`'s one-row `.actions` section.

## 4. Verification

Every command below run in the foreground from `/Users/ueli/Documents/semio` with
`DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0
CARGO_PROFILE_WASM_DEV_DEBUG=false`, shared build dir, no `CARGO_TARGET_DIR`. Logs under
`🗑️generated/a6a/`.

| Command | Result | Log |
| --- | --- | --- |
| `cargo check -p …generation3d --features component-app-assembly --lib --tests` | ✅ rc=0, `Finished dev profile in 1m 23s`, 0 errors | `gen3d-check.txt` |
| `cargo check -p …generation2d --features component-app-assembly --lib --tests` | ✅ rc=0, `Finished dev profile in 1m 14s`, 0 errors | `gen2d-check.txt` |
| `cargo test -p …generation3d --features component-app-assembly --lib panels` | ✅ **20 passed; 0 failed** (473 filtered out) | `gen3d-panels.txt` |
| `cargo test -p …generation3d --features component-app-assembly --lib generations` | ✅ **10 passed; 0 failed** (483 filtered out) | `gen3d-generations.txt` |
| `cargo test -p …generation3d --features component-app-assembly --lib mode_panels` | ✅ **3 passed; 0 failed** (490 filtered out) | `gen3d-mode-panels.txt` |
| `cargo test -p …generation2d --features component-app-assembly --lib panels` | ✅ **11 passed; 0 failed** (250 filtered out) | `gen2d-panels.txt` |
| `cargo test -p …generation2d --features component-app-assembly --lib generations` | ✅ **4 passed; 0 failed** (257 filtered out) | `gen2d-generations.txt` |
| `cargo test -p …generation3d --features component-app-assembly --no-fail-fast` (full suite) | ⏳ see §6 | `gen3d-test.txt` |
| `cargo test -p …generation2d --features component-app-assembly --no-fail-fast` (full suite) | ⏳ see §6 | `gen2d-test.txt` |
| `cargo check -p …generation3d --target wasm32-wasip2 --features component-app-assembly` | ⏳ see §6 | `gen3d-wasm.txt` |
| `cargo check -p …generation2d --target wasm32-wasip2 --features component-app-assembly` | ⏳ see §6 | `gen2d-wasm.txt` |

The `panels` filters cover laws (a)–(d) for every panel tree of both apps; `generations` covers
(a)/(b)/(c) for the roster window in each app; `mode_panels` re-runs the three mode-parity laws that
the stale `🗑️generated/a6/gen3d-mode-panels.txt` showed failing — the fixture on disk already had the
fix and the run had simply never been repeated.

## 5. Peer breakage met on the way (not this ticket, evidence recorded)

Two foreground runs died on code this packet does not touch; both were confirmed as a peer mid-refactor
and both cleared on retry:

1. `semio-framework-os-infinite` — 12 errors, `cannot find type TransformGumballFlags`,
   `transform_pivot_of`, `transform_ring_hit`, … Working tree showed
   `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/{🦀️.rs,🔌️ports/➡️directed/…}` modified
   minutes earlier. Cleared without any change from this packet.
2. `semio-framework-pixels` — 8 errors, `cannot find module or crate semio_framework_deflate`. The
   peer had added `semio-framework-deflate` to
   `🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust/Cargo.toml` 8 minutes earlier. `cargo check -p
   semio-framework-pixels` was green again 13 minutes later, with no change from this packet.

Neither touches trees, panels, paging, windows, `setPanelPage`, `UI_BUILT_CHILDREN_MAX` or
`UI_VALUE_PAGE_ROWS`.

## 6. Not finished

The full crate suites and the two `wasm32-wasip2` checks were still outstanding when this section was
written; they are being run now and this report is updated in place with their counts and triage.
Nothing here is claimed as passing that was not run — the ⏳ rows above are exactly the commands whose
results are still missing.
