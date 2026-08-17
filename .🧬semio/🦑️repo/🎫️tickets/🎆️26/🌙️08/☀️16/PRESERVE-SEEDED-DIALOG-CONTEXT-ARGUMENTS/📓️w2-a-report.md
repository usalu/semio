# W2-A report — Home app = table of spaces

Lane 2-A. Replaces the pre-ticket virtual-file-system Home scene with a real overview table of every
space, fed by the event-sourced hub directory read model (contract §C1) UNIONED with the local-only
catalog, per the worker brief.

## Changed files

New (8 command leaves under `✏️editor/🎮️commands/`, each mirroring the existing command-leaf shape —
`dsl::DslRecord` payload struct + free `handle`/`handle_copy_invite_link` fn + its own `🧪️Tests`):
- `🌱create-space/🦀️component.rs` — `CreateSpace { name, kind, visibility }`. Empty `name` opens the
  declared `createSpace` dialog; non-empty `name` emits `HostEffect::ReplayShellCommand{ action_id:
  "os.directory.create-space", args }`.
- `🗑️delete-space/🦀️component.rs` — `DeleteSpace { space_id, confirmed }`. Two-phase from ONE command
  id: `confirmed: false` (the default) emits the `deleteSpace` confirm dialog and NEVER touches the
  network; `confirmed: true` (the dialog's own resubmit, `confirmed` riding along in `OpenDialog`'s
  pre-seeded args) emits the real relay.
- `🏷️rename-space/🦀️component.rs` — `RenameSpace { space_id, name }`. Empty `name` opens `renameSpace`
  pre-seeded with the CURRENT name read from `cfg.snapshot.directory()`.
- `🔗️share-space/🦀️component.rs` — `ShareSpace { space_id, email, role }` → `os.directory.upsert-member`.
- `📋copy-invite-link/🦀️component.rs` — `CopyInviteLink { space_id, role, ttl_secs }` →
  `os.directory.share-link` (sugar for `create-invite`). Its own leaf (not nested inside `share-space`)
  because `app_commands!`'s `$module::handle` binding is one Rust module per command — two structs
  needing their own `handle` fn cannot share one file's module path.
- `📇️fold-directory-events/🦀️component.rs` — `FoldDirectoryEvents { events_json }`. View action only:
  parses the JSON array of `DirectoryEvent` and emits one `HomeConfigMutation::FoldDirectoryEvent` per
  event — never an artifact mutation.
- `👥️presence-heartbeat/🦀️component.rs` — `PresenceHeartbeat {}`. Deliberate, documented no-op stub:
  Home has no `(space_id, document_id, surface)` to scope a heartbeat against (contract §C0); kept
  dispatchable so the action id is never silently dropped, ready for the day Home's table gets a
  presence roster (`👥️PresenceBar`, lane 2-F/3-A territory).
- `🪪️set-client/🦀️component.rs` — `SetClient { client_id, client_name }`. Not in the brief's named list
  of 7, but required: without SOME command dispatching it, `HomeConfigMutation::SetClient` would be
  unreachable dead code once the shell's identity bootstrap needs to set it.

Edited:
- `✏️editor/🎚️config/🦀️component.rs` — `HomeConfig` gains `directory_json: String`, `client_id: String`,
  `client_name: String` + a `directory() -> DirectoryReadModel` accessor; `HomeConfigMutation` gains
  `FoldDirectoryEvent { event_json }` / `SetClient { client_id, client_name }`. See "Design decision:
  DirectoryReadModel storage" below for why the field is `directory_json: String`, not a typed
  `directory: DirectoryReadModel` field. New tests: default-directory-is-empty, fold-applies-the-real-
  fold, fold-ignores-malformed-json, set-client-updates-fields, op-text round trip for both variants.
- `✏️editor/🗣️terminology/🦀️component.rs` — trimmed to editor-exclusive strings only (`window_main` +
  `action_open/rename/share/delete`); the table-column/origin/empty-message strings moved to the new
  shared `crate::HomeTableLabels` (see below) once I found the viewer needed them too and duplicating
  them in two `app_labels!` structs would violate CLAUDE.md's "no repeated code" rule.
- `✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️component.rs` — VFS scene replaced with a real
  `TableWindowKit` table (columns: name · kind · visibility · members · updated · origin · actions),
  built from `crate::home_space_rows(&cfg.directory())`. `render_rows` split out as a pure core so the
  empty-state branch is unit-testable without depending on the process-global catalog singleton (see
  "Design decision: test isolation" below). 6 new tests (empty/local-row/hub-row/seeded/German-locale/
  locale-resolution-from-config).
- `✏️editor/🦀️component.rs` — `HomeCommand` gains the 8 new variants; `command_from_action` bridges all
  8 action ids from shell JSON args; `create_home_app()` registers each as `.shell_action`/`.view_action`
  (never `.mutation`: none of these touch `SHomeMutation`, they only emit `HostEffect`/config
  mutations) plus 4 `.dialog(...)` declarations (createSpace/deleteSpace/renameSpace/shareSpace) with
  typed `ActionArgDef` fields, and adds the 5 user-facing ids to `window_kind_action_refs`. Rewrote the
  two pre-existing locale tests (`home_labels_resolve_native_english_by_default`/`_german_locale`) —
  they asserted on the old VFS scene's ALWAYS-present `emptyMessage` field, which no longer exists on a
  `TableView`; rewritten to fold a known directory event and assert on the real thing "labels resolve
  to the right locale" means for a table: the locale-correct column headers.
- `✏️editor/🎮️commands/🏙️create-studio/🦀️component.rs` — identity fix (see below): both
  `create_folder_studio` and the ephemeral-studio branch now thread `cfg.snapshot.client_id`/
  `client_name` into the new `SpaceUser` instead of the hardcoded `"local"` sentinel.
- `👁️viewer/🦀️component.rs` — `Config`/`ConfigMutation` changed from `NoConfig`/`NoConfigMutation` to
  the SHARED `HomeConfig`/`HomeConfigMutation` (the viewer renders the same directory-fed table and must
  read the same `directory_json`); `HomeViewCommand` gains `FoldDirectoryEvents { events_json }`
  (hand-rolled `OpBinary`, length-prefixed by a 1-byte discriminant) so a mounted VIEWER session also
  receives folded directory events, per contract §C6's "whichever session is mounted." `render` now
  reads `cfg.snapshot.directory()` and `cfg.snapshot.locale`. New tests:
  `fold_directory_events_command_never_touches_the_document_store` (uses the SDK's own
  `assert_viewer_never_mutates::<HomeViewer>()`), `editor_and_viewer_share_one_dialect`.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️component.rs` — VFS scene replaced with the SAME
  `TableWindowKit` table, read-only: 6 of the editor's 7 columns render (name/kind/visibility/members/
  updated/origin) — the trailing "Actions" column is DROPPED, not left empty, since the viewer has
  nothing to summarize there. `render_rows` split out for the same test-isolation reason as the editor.
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs` (plugin root, in-lease region) —
  1. The two `SpaceUser { id: "local", … }` sites: `create_and_register_ephemeral_studio` gained
     `owner_id`/`owner_name` params (falls back to `"local"` only when both are empty — the local-only
     no-hub path stays exactly as before). `catalog_port_concrete`'s ONE remaining `"local"` seed is a
     deliberate non-change: it is a process-global `LazyLock` boot fixture with no session/`HomeConfig`
     in scope at all, so there is no real identity to attribute it to — documented inline rather than
     fabricating one.
  2. New `HomeTableLabels` (`app_labels!`) + `HomeSpaceRow`/`home_space_rows` — the shared row-union
     (hub directory ⋃ local catalog, hub wins on id collision) and bilingual table strings, reachable by
     both the editor and the viewer (a viewer file can never import through `::editor::`), following the
     EXACT precedent this file's own `list_all_space_catalog_entries` already set for the same reason.

Foreign touch (outside the narrow plugin-root lease, additive-only, documented per lane 1-E's own
precedent for this exact situation):
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — 8 additive `#[path]` mounts for the new command
  modules inside the EXISTING `pub mod home { pub mod commands { … } }` block. No line touched or
  reordered.

## Design decision: `DirectoryReadModel` storage

The brief says `HomeConfig` gains `directory: DirectoryReadModel`. `HomeConfig` derives
`dsl::DslArtifact` (must round-trip through DSL text AND binary pack); `DirectoryReadModel`/
`DirectorySpace` (`🧰️framework/**`, forbidden to me) derive neither `Serialize`/`Deserialize` nor any
`dsl::*` trait. I checked the `dsl` derive's field-attribute parser for an opaque/json escape hatch —
none exists (`key/positional/list/tuple/statements/block/base64/flatten/table/unit/angle/refs/defines/
lang/lang_from/coord/dir` only; `base64` is `Vec<u8>`-only). The established precedent for exactly this
shape in this codebase is `🔱️trinity/🔌️jack`'s `JackConfig.jack_result_json: String` (a plain,
un-annotated `String` DSL field decoded by hand outside the derive). I followed it: `directory_json:
String` + a `directory() -> DirectoryReadModel` accessor. Rather than deriving `Serialize` on the
forbidden framework types (a foreign edit I do not need), I hand-wrote a small wire-shape pair
(`DirectoryReadModelWire`/`DirectorySpaceWire`) entirely inside my own config file, built from the
leaf types (`SpaceView`/`MemberView`/`UserView`) that already ARE `Serialize`/`Deserialize` — zero
framework touches needed for this at all.

## Design decision: test isolation from the process-global catalog

`crate::list_all_space_catalog_entries()` (pre-existing, plugin root) is a process-global accumulate-
only singleton — every test in this crate's ONE test binary shares it, and it only ever grows. My first
pass wrote "renders the empty message when nothing exists" tests against the real `render`/`home_space_
rows` path; they were flaky-then-reliably-failing once other tests (mine and pre-existing) had created
studios earlier in the same test run. Fixed by splitting a pure `render_rows(rows: &[HomeSpaceRow], …)`
core out of both the editor's and the viewer's `render`, and testing the empty branch against an EXPLICIT
empty slice literal — true isolation, no dependency on catalog state or test execution order. The two
pre-existing locale tests in `✏️editor/🦀️component.rs` had the same latent fragility (mitigated
previously only by accident: the old VFS scene always serialized an `emptyMessage` field regardless of
actual row count) — rewritten to fold a known directory event instead of relying on emptiness.

## Known gap (verified, not assumed): row ids and per-row action buttons

`semio_framework_plugin::app::{TableView, TableWindowKit}` — checked both the Rust type and the TS twin
(`🔌️plugin/📦️packages/🟦️typescript/🪟️window-kits/📊️table/🟦️component.ts`) — is a flat `{ columns:
Vec<String>, rows: Vec<Vec<String>> }` grid with NO per-row id or per-row action-button field anywhere.
Contract §C0's `data-row-id="space:<id>"` grammar and real clickable row actions therefore cannot be
produced from inside this lease using this window kit; the renderer that would need to stamp row ids is
`🧰️framework/**`-owned and forbidden to me. `render_rows` folds the available actions into the trailing
"Actions" column as TEXT (`open`, or `open · rename · share · delete` for hub-origin rows) so the
information is present and locale-correct even though no button renders it yet. Every row-scoped command
(`rename-space`/`delete-space`/`share-space`) already takes an explicit `space_id` argument — same shape
`navigateVirtualFileSystemNode`/`deleteVirtualFileSystemNode` always used — so they are fully
dispatchable and tested today; only the click-to-button wiring is blocked on a richer window kit. Lane
1-E's own `s.space` index editor hit the identical gap and deferred it the same way.

## Commands run + result counts (real tails)

`cargo check -p semio-s-plugin-space` (`🧪️2-a-cargo-check.txt`): **0 errors** (18-55 warnings across
several runs, all pre-existing/unrelated to this lane — `cargo fix` suggestions, elided lifetimes,
unused imports in files I never touched).

`cargo test -p semio-s-plugin-space --lib` (`🧪️2-a-cargo-test.txt`, the last of several runs — see
"Concurrent workspace churn" below for why several runs were needed): **196 passed, 3 failed** (was
124 passed / 15 failed at lane 2-0's baseline; the crate grew by ~150 tests, mine among them, and net
failures dropped from 15 to 3 through other lanes' concurrent work). The 3 remaining failures are ALL
`engine::space::*`:
```
engine::space::commands::export_media::tests::export_media_emits_download_effect_and_import_requests_file_open
engine::space::commands::set_active_panel_tab::tests::set_app_registrations_command_registers_app_and_surfaces_empty_document_apps_in_catalogue
engine::space::component::tests::two_instances_converge_on_disjoint_edits_via_backbone
```
`⚙️engine/**` is explicitly forbidden to me and explicitly named in the brief as "NOT yours." Attribution:
`git status --porcelain -- "✏️s/🔌️plugins/🪐️space/⚙️engine/**"` shows 5 files live-`M` (uncommitted,
mid-edit) right now, including `⚙️engine/🪐️space/🦀️component.rs` itself — lane 2-G's concurrent work, not
mine, not attributable to a specific commit since it is uncommitted. **My own new tests all pass; zero
new failures introduced.**

## Concurrent workspace churn (documented per the memory-note pattern from lanes 1-E/2-0)

`🗿️artifacts/🪐️space/**` (forbidden, lane 2-B) and `⚙️engine/**` (forbidden, lane 2-G) were both being
actively, uncommittedly edited throughout this session. Repeated `cargo check`/`cargo test` runs on the
SAME unchanged code from me surfaced a shifting set of errors in those forbidden trees (a
`store::dsl_value_to_json` that didn't exist yet, then didn't error, then a different
`InvocationResult.config_mutations` field mismatch, …) — never once in my own files. I polled rather than
chased, per the existing project memory on this exact pattern, and report the LAST observed, stable
(2 consecutive identical runs) state above.

## sharedFileRequests

None blocking. The `📦️glue.rs` touch above is additive-only and mirrors lane 1-E's own precedent for
mounting new command modules from outside the narrowest reading of a lease; flagging it here rather than
silently assuming.

## What is NOT done

- **Real per-row clickable action buttons / `data-row-id="space:<id>"` stamping** — blocked on
  `TableWindowKit`'s current flat shape, `🧰️framework/**`-owned; see "Known gap" above. All row-scoped
  commands are dispatchable and tested; only the UI click-to-command wiring is missing.
- **`copy-invite-link`'s "copy to clipboard" UI feedback** — the command emits the real
  `ReplayShellCommand` for `os.directory.share-link`; the invite token does not exist until the hub
  mints it and returns it over `/directory/ws`, so presenting/copying it is necessarily shell-owned, out
  of a Rust command handler's reach.
- **`presence-heartbeat` real behavior** — deliberate, documented no-op stub (see above); Home has no
  `(space_id, document_id, surface)` scope to attach a real heartbeat to yet.
- **End-to-end click-through verification** — no dev server / browser session was available in this
  lane's environment; every claim above is backed by a real `cargo test` run, never assumed.
- Postgres/neo4j hub directory backends, wasm32 target gate — unrelated to this lane, not touched.

Ticket not closed (coordinator owns that).
