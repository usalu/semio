# W2-B report — the Space app is the table of a space's artifacts

Lane 2-B. Extends lane 1-E's `s.space` artifact (kind `s.space`, dialect `s.space.space@1/*`, document
`index`) into the real working space app: table columns, commands, members panel, viewer parity, en+de.

## Changed files

All inside my lease `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/**`, plus one glue-registration line set
(explicitly permitted by the brief).

**New**
- `✏️editor/🎚️config/🦀️component.rs` — `SpaceIndexConfig` (real `ArtifactApp::Config`, replaces `NoConfig`):
  `visibility: String`, `members: Vec<SpaceIndexMember>`, `presence: Vec<SpaceIndexArtifactPresence>`.
  Handcrafted `ArtifactDsl`/`ArtifactPack` (mirrors `SSpaceSnapshot`'s own pattern) +
  `store::impl_whole_record_config!`. `SpaceIndexConfigMutation` — one whole-record `Snapshot` variant
  (mirrors `DrawConfigMutation::Snapshot`), hand-written `Mutation`/`OpText`/`OpBinary`. Deleted the
  now-obsolete `✏️editor/🎚️config/📌️empty.md` placeholder.
- 10 new command leaves under `✏️editor/🎮️commands/` (each: payload struct + `handle()` + tests):
  - `❔request-delete-artifact` — opens the `deleteArtifact` confirm dialog (`HostEffect::OpenDialog`),
    never mutates. The pre-existing `🗑️delete-artifact` stays the real, undecorated delete (its own
    dispatch/test surface unchanged) — the dialog's `submit_action` re-dispatches it.
  - `📂open-artifact` — relays `os.open-artifact` with `documentId`/`spaceId`, no `role` (lets the shell
    resolve `OpeningPreferences`'s default — see "Design decisions" below).
  - `🗃️open-artifact-with` — relays `os.open-artifact-with` with the user's explicit `role`/`pluginId`/
    `appId` (the "Open with…" chooser).
  - `📇fold-directory-events` — decodes a JSON `DirectoryEvent[]` batch, folds via the OS's own pure
    `semio_framework_os_kernel::os_directory::fold_all` (contract §C1, reused not re-derived), writes
    `visibility`/`members` for THIS space into `Config`. No-op for events naming a different space.
  - `💓presence-heartbeat` — replaces (never appends to) one artifact's live actor-id CSV in `Config`.
  - `💌invite-member` / `🚪remove-member` / `👁️set-visibility` / `🔗copy-invite-link` — members-panel
    relays to `os.directory.upsert-member` / `remove-member` / `set-visibility` / `share-link`.
  - `❕request-invite-member` — opens the `inviteMember` staged-form dialog (email + role); its submit
    re-dispatches `invite-member`.
- `✏️editor/📌️panels/👥️members/🦀️component.rs` — the members panel: folded-member list (each row a
  `member:<userId>` tree item with a remove action), invite/copy-link/visibility-toggle action buttons
  (`#s-space-invite`, `#s-space-share`, `#s-space-visibility` — contract §C0 id grammar), empty state.

**Edited**
- `✏️editor/🦀️component.rs` — `Config`/`ConfigMutation` now `SpaceIndexConfig`/`SpaceIndexConfigMutation`;
  `app_commands!` grew from 4 to 14 rows; `render()` now dispatches the members panel body key and
  passes `cfg.snapshot` into the table window; new `KNOWN_ARTIFACT_KINDS` static table (see "Known
  artifact kinds" below) + `known_artifact_kind()`; three `.dialog(...)` registrations
  (`createArtifact`, `deleteArtifact`, `inviteMember`); every new command wired via `.shell_action(...)`
  (relay-only, no document mutation) or `.view_action(...)` (config-only fold).
- `✏️editor/🎮️commands/🌱create-artifact/🦀️component.rs` — payload shrank to
  `{name, kind_id, now_ms, actor}` (no more caller-supplied `id`/`schema`/`dialect_*`): `handle()` now
  mints the id (`mint_artifact_id`, new schema helper) and resolves `schema`/`dialect` from
  `known_artifact_kind(kind_id)`, then emits BOTH the `SSpaceMutation::CreateArtifact` AND a
  `HostEffect::ReplayShellCommand{action_id: "os.open-artifact", args: {artifactRef, role: "editor",
  documentId, spaceId}}` so the new artifact opens immediately (worker-brief task 2, verbatim).
- `✏️editor/🎮️commands/🗑️delete-artifact/`, `🏷️rename-artifact/`, `🕒touch-artifact/` — mechanical
  `NoConfig`/`NoConfigMutation` → `SpaceIndexConfig`/`SpaceIndexConfigMutation` signature updates; their
  own tests updated to create a seed artifact through the new (id-less) `CreateArtifact` payload and
  read the minted id back off the snapshot instead of hardcoding `"artifact-1"`.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️component.rs` — table now renders the full worker-brief
  column set (id · name · kind · subset · updated · updated-by · presence) via a shared helper (below);
  `render()` gained a `config: &SpaceIndexConfig` param so the presence column is live.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️component.rs` — same column set via the same shared
  helper; presence cell is always empty (the viewer has no `Config` fold of its own — `NoConfig`,
  unchanged, `assert_viewer_never_mutates` still holds).
- `🧬️schema/📸️snapshot/🦀️component.rs` — added `mint_artifact_id(existing, now_ms) -> String` and the
  shared `SPACE_INDEX_TABLE_COLUMNS`/`space_index_table_row(row, presence)` table-projection helpers
  (schema-layer, neutral — importable by both editor and viewer without a `policyViewerPurityBreaches`
  hit).
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — additive `#[path]` mounts: `space_index::config`,
  10 new `space_index::commands::*` modules, `space_index::panels::members`. Mirrors 1-E's existing
  wiring shape verbatim; no line removed or reordered.

## Task-by-task status

1. **Table columns** — done: id · name · kind · subset · updated · updated-by · presence, both surfaces.
   **Not done, framework-owned, documented in-file**: `TableView`/`TableWindowKit`
   (`🧰️framework/🔨️modules/🔌️plugin/🦀️component.rs`, outside this lease) carries plain
   `{columns: Vec<String>, rows: Vec<Vec<String>>}` — no sortable flag, no row-id field distinct from a
   cell. "Sortable if the table primitive supports it" is a documented no-op. The `artifact:<id>`
   `data-row-id` prefix (contract §C0) is a renderer-layer (`📺️renderer`/`🖱️ui`) concern; the ID column's
   cell value is the raw id, ready for that layer to prefix.
2. **Commands** — `create-artifact`/`open-artifact`/`open-artifact-with`/`delete-artifact` (confirm via
   `request-delete-artifact` + dialog)/`rename-artifact`/`fold-directory-events`/`presence-heartbeat` —
   all done, all with tests.
3. **Members panel** — done: folded members, invite-by-email+role (via dialog), remove-member,
   visibility toggle, copy-invite-link — every mutating affordance an `os.directory.*` relay, zero
   network calls from the guest.
4. **Viewer** — done: same table (read-only), no mutating affordances. `assert_viewer_never_mutates`/
   `assert_editor_and_viewer_share_dialect` (1-E's tests, plugin root `🦀️component.rs`) still pass.
5. **en + de** — done for every STATIC manifest string (panel tab label, all three dialog
   titles/bodies/submit labels, every `ActionArgDef`/`ActionArgOption`, every `.shell_action`/
   `.view_action`/`.mutation` label). **Known limitation, documented in-file** (members panel's own doc
   comment): `UiTreeItemNode`/`PanelTreeBuilder` take a plain `Label` (`Label::data`/a locale-resolved
   `LabelText` only — no `From<LocalizedLabel>`, `🖱️ui`'s wgpu `Label` type, outside this lease), so the
   panel's TREE CONTENT strings (member rows, action-button labels, empty state) are English-only. Real
   bilingual tree content needs an `app_labels!` terminology struct + a `locale` field threaded through
   `SpaceIndexConfig` (mirrors `🖍️draw`'s `DrawPlayLabels`/`DrawConfig.locale`) — a real facet, not a
   one-line fix; deferred rather than half-built at this effort level.

## Design decisions worth flagging

1. **`create-artifact`'s payload dropped `id`** (1-E's shape had the caller supply it). The brief says
   the command itself "mints a document id" — `handle()` is pure/no-IO, so `mint_artifact_id` derives
   uniqueness from `now_ms` + a collision-probed counter over the snapshot's existing rows, not a
   random/host-global source (tested: two creates at the same `now_ms` still get distinct ids).
2. **Known artifact kinds is a static Rust table, not a live registry read.** The brief's own escape
   hatch: "if the guest cannot see that list, take it from a config-lane value the host sets and say so
   in your report." The space plugin is its own isolated guest crate with no in-process visibility into
   every OTHER plugin's `ArtifactKindSpec`, and the shell's directory/opening lane doesn't expose that
   catalog either yet (confirmed: `📓️w2-c-report.md`'s own "Design decisions" #2 admits there is no
   general dialect→schema formula available to the shell today). Curated 4 real, verified kinds
   (`draw`→`s.draw.draw`/`draw.document`, `note`→`s.note.note`/`note.document`,
   `dag`→`s.dag.dag`/`dag.dag`, `writer`→`s.writer.writer`/`writer.document` — each grepped from that
   plugin's own `DIALECT`/`DOCUMENT_SCHEMA` constants, not guessed) rather than a live read. Flagging for
   whoever next builds the config-lane `availableKinds` fold this should graduate into.
3. **`open-artifact` omits `role` entirely** rather than defaulting to a literal — `OpeningPreferences`
   (`🔌️plugin/🖥️host/🦀️component.rs`'s `OpeningResolver`) is host-side state the space plugin cannot
   read; per the existing `AppCommand::OpenArtifact{..., role}` shape, an absent/unset role is exactly
   what triggers preference-based resolution. `open-artifact-with` always sends an explicit role (the
   user's chooser pick).
4. **`request-delete-artifact`/`request-invite-member` are new "opener" commands**, not named in the
   brief's own command list, but required by the ONLY dialog-opening mechanism this codebase has
   (`HostEffect::OpenDialog`, "opened only via" — `🛂️manifest/🦀️component.rs`'s own doc comment). This
   is the same two-command shape (opener + real mutator, linked by a `DialogDefinition.submit_action`)
   `🧩️puzzle`'s `addObject`/`addObjectKind` pair already uses.
5. **Members panel's "Invite Member" dialog collects `email`+`role` only** (not the space id — that's
   read from `doc.snapshot.space_id` inside `invite-member`'s own `handle()`), so the staged form the
   user actually sees is minimal.

## Commands run + result counts (real tails, `🧪️2-b-*.txt`)

`cargo check -p semio-s-plugin-space` (`🧪️2-b-cargo-check-3.txt`): **0 errors** (17 iterations getting
there are in `-1.txt`/`-2.txt` — `DslValue` vs `serde_json::Value` on `HostEffect` fields,
`semio_framework_os_kernel::os_directory` not `semio_framework_os::os_directory`, `Label` vs
`LocalizedLabel` on `UiTreeItemNode`/`PanelTreeBuilder`).

`cargo test -p semio-s-plugin-space --lib` (`🧪️2-b-cargo-test-final.txt`): **187 passed; 7 failed**
(baseline handed to this lane was **124 passed / 15 failed**, all 15 `engine::space::*`). All 63 tests
under `editor::space_index::*` (38), `artifacts::space::*` (20), `viewer::space_index::*` (5) — my
domain plus 1-E's — **pass, 0 failures**. The 7 that fail are every one of them **foreign, live,
mid-edit right now**, confirmed two ways:
- `git status --porcelain` shows all 7 files ` M` (uncommitted, not mine — I never touched
  `🗿️artifacts/🏠️home/**` or `⚙️engine/🪐️space/**`, both explicitly forbidden to me):
  `editor::home::component::tests::home_labels_resolve_native_{english_by_default,german_locale}`,
  `editor::home::modes::explore::windows::main::...::empty_directory_and_catalog_render_the_empty_message`,
  `viewer::home::modes::view::windows::main::...::render_with_no_spaces_shows_the_empty_message`,
  `engine::space::commands::export_media::tests::...`,
  `engine::space::commands::set_active_panel_tab::tests::...`,
  `engine::space::component::tests::two_instances_converge_on_disjoint_edits_via_backbone`.
- A **later** re-run (`🧪️2-b-cargo-test-retry.txt`) hit an outright compile error in
  `🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:350` ("functions used as
  tests can not have any arguments" on a helper `fn config_with_one_folded_space(locale: &str)`) — that
  file's mtime was 20 seconds old at the time, i.e. lane 2-A editing it live, exactly the "running in
  parallel right now" the worker-brief itself warned about. The `🧪️2-b-cargo-test-final.txt` snapshot
  above is the trustworthy one (captured moments earlier, complete, no foreign compile error in flight).
  **Zero new failures added by this lane** — every failure present in my clean 187/7 run is a file I
  never edited.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/**` plus the one permitted
`📦️glue.rs` registration block.

## What is NOT done

- **Sortable table columns** — the shared `TableWindowKit`/`TableView` primitive has no sort concept at
  all (framework-owned, not this lease). Documented as a no-op in the window render file's own comment.
- **`data-row-id="artifact:<id>"` DOM attribute** — the ID column's cell carries the raw id; the
  `artifact:` prefix and actual DOM wiring is a renderer-layer (`📺️renderer`/`🖱️ui`) concern outside this
  lease.
- **Bilingual (de) tree-content strings** in the members panel (member rows, action-button labels, empty
  state) — `Label`/`UiTreeItemNode` structurally only take English-plain or already-locale-resolved
  text; every STATIC manifest string (dialogs, panel tab, action definitions) IS en+de. Needs an
  `app_labels!` terminology struct + a `locale` field on `SpaceIndexConfig` to close, deferred.
- **`KNOWN_ARTIFACT_KINDS` is a static 4-kind table**, not a live `ArtifactKindSpec` registry read —
  documented above as the brief's own sanctioned fallback.
- **End-to-end click-through in a running shell** — not attempted (no hub/dev server booted this
  session; this was a headless Rust-crate lane). Everything above is verified at the
  `cargo test`/unit level only, same scope every other lane in this ticket has operated at.
- The 7 foreign `engine::space::*`/`home::*` test failures documented above — not mine, not fixed,
  flagged for lanes 2-A/2-G.
