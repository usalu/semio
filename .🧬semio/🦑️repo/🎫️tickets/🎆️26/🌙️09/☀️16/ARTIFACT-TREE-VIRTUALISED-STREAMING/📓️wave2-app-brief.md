# 📓️ Wave 2 — app migration brief (read with 📓️design-virtualised-tree.md §5 and §8)

## Rules for every app packet
- Read `/Users/ueli/Documents/semio/AGENTS.md`, then 📓️design-virtualised-tree.md (§5 SDK API, §8 migration rule), then the audit
  section for your plugins in 📓️audit-app-panels-a.md / 📓️audit-app-panels-b.md and the test inventory 📓️audit-paging-tests.md.
- cd into the repo explicitly in every Bash call; quote emoji paths; quote `--include='*.rs'`; never `echo ====`.
- Never `git stash/commit/checkout/reset/worktree`; never sweep any 🗑️generated folder; never close/reopen tickets.
- Other agents edit other plugins and the framework concurrently. Write your code first against the SDK names in §5.
  Do NOT run any cargo command until BOTH `📓️p1-contract.md` and `📓️p3-sdk.md` exist in this ticket folder (poll every
  60 s in a foreground loop, up to 45 min). Read 📓️p3-sdk.md when it appears — if a signature differs from §5, follow
  📓️p3-sdk.md.
- Foreground builds only; never claim a test passed without running it; capture output under 🗑️generated/<packet>/.
- Shared cargo target dir with fine-grain locking: never set CARGO_TARGET_DIR; an idle-looking cargo may hold/await a lock;
  Xcode license link errors → `DEVELOPER_DIR=/Library/Developer/CommandLineTools`; plugin crates verify with
  `cargo test -p <crate> [--features component-app-assembly]` then `cargo check -p <crate> --target wasm32-wasip2
  [--features …]` (feature flags per 📓️audit-paging-tests.md §8; check each crate's Cargo.toml).

## What "migrated" means (per panel)
1. Every list section is built with `PanelTreeBuilder::window_section(&windows, id, label, default_open, &entries, row)`
   or `window_section_or_placeholder(...)`; every group row that nests children uses `tree_window_item(&windows, item,
   id, default_open, &children, row)`. `windows = TreeWindows::for_body(view_state, BODY_KEY)` is built once in the
   app's `render_body` and passed into the panel (panels take `&TreeWindows<'_>`; tests use `TreeWindows::unhosted()`).
2. Domain-bound pick rows: no per-row `interactionSelect` argument map. `.interaction_domain(CONTROLLER_ID, DOMAIN)` on
   the tree builder + `.granularity(g)` on each pick row; row key = raw target id. Rows with their own app action
   (add/install/checkout/toggle) keep their binding. Row actions (hide/lock/delete) stay as they are.
3. Delete all app-local paging: `paged_section*`, `continuation_row*`, `section_page`, `SECTION_ROWS`, `IDS_ROWS`,
   `LIST_ROWS_MAX`, `CATALOGUE_GROUP_ROWS`, `PANEL_RECONCILE_NODE_BUDGET`, `.take(N)` list truncations and every static
   `+N`/`…more` row, the `setPanelPage` command dir, enum/const, dispatch arm, `.view_action`, `.action_interactive_job`,
   `ArtifactToolPublicationContract`, `toolIds`/`TOOL_JOB_IDS` entries, `panel_pages` on config / window-config /
   window-transient structs (+ their generated schemas `🧬️schema/{🔣️.json,🔗️.graphql,🟦️.ts,🛰️.proto}` and
   retire/round-trip tests), retained-jobs / publication-authority fixtures naming `setPanelPage`, memo caches keyed on
   pages, and the crate-local `ui_node_list` copy (import the SDK's). Regenerate any checked-in generated manifest
   (`✏️s/🔌️plugins/<plugin>/🔣️.json`) with the plugin's own nx/script target rather than hand-editing.
4. Tests (replace the paging laws, keep everything else): (a) oversized document → every container stamps
   `window.total == entries.len()` and materialises ≤ its slice, body JSON contains no `.more` key and no `"+` label;
   (b) closed container (`TreeWindowRequest{open: Some(false)}` or `default_open false`) → `total` stamped, zero
   children; (c) `TreeWindowRequest{offset: k, rows: n}` → exactly entries `[k, k+n)` keyed by raw id; (d) domain rows
   carry `granularity` and no per-row activate binding while the tree root carries exactly one `interactionSelect`.
   Build request fixtures with `ViewModel { tree_windows: vec![TreeWindowRequest{…}], ..Default::default() }`.
5. Write `📓️<packet>-<plugins>.md` in this folder: panels migrated (paths), symbols deleted, tests (commands + counts),
   anything not finished with the reason. Reply in chat with the report path and a 5-line summary only.
