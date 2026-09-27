# WP-SH1 — Space-Home Reachability + Kernel Lib Reds (prepared for landing window 3)

Session 13 slice SH1 (Opus executor, coordinator `main`). Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP. Rules: preamble
13 rules 1–33 (rule 33: no source edits after 15:45 until window 3). Inputs `wp-sh1/` (`sh1-apply.py`, `payload/`), captures
`wp-sh1/generated/`. Scratch overlay `.🧬semio/🌐hub/s13-sh1-overlay/` (gitignored copy of the tracked tree), private
`CARGO_BUILD_BUILD_DIR`/`CARGO_TARGET_DIR` under `.🧬semio/🌐hub/s13-sh1-*`. Nothing in the repo tree is edited before window 3.

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | space-home `bindSpaceFile`, `importSpace`, `deleteVirtualFileSystemNode` unreachable (`BatchOnlyPendingRewrite`, T13 census 4/1165) | **designed + written in the stage** (25-file payload, dry run clean on the tree 15:58); overlay compile + laws pending |
| 2 | space-home reds: `structural_correspondence` outcomeClasses; config `retained_config_cancel_and_cleanup_respect_the_production_grant` | root causes found (both stale tests, code is right): payload being written |
| 3 | kernel lib suite 7 reds (directory client ×5, open-plan fixture, norm grammar en1993) | **re-run in overlay: 1233/1234** — 6 fixed by others since 05:4x; the 1 left = en1993 mutation grammar (Wave C wrote a non-dialect grammar) → restore patch prepared, N1 informed |

### Log

- 14:58 read AGENTS.md, preamble 13 (rules 1–33), fleet log (T13 census 11:1x, LC 11:2x, LD 10:3x), `📓️wp-t13.md`
  (census: `generated/census-commands-1.txt`), `📓️wp-ld.md` (kernel reds: `wp-ld/generated/item2-laws-native-2.txt`),
  `📓️wp-lc.md` (space-home reds: `.🧬semio/🌐hub/s13-lc-laws/space-home{,-editor}.txt`).
- 15:02–15:20 overlay: tracked + untracked-unignored files (106 598) copied to `.🧬semio/🌐hub/s13-sh1-overlay/` (tar, 6.0 GB,
  1170 s under load 40; openrsync lacks `--ignore-missing-args`). Runner `wp-sh1/sh1-overlay-cargo.sh` (private build/target dirs,
  nice 10, incremental off).
- 15:22 kernel lib suite launched in the overlay (`cargo test -p semio-framework-os-kernel --features sync,ureq --lib`, pid 47240,
  detached; cold private build-dir).
- 15:2x item 2 root causes (read, not yet run):
  - `structural_correspondence…::direct_owner_descriptor_surfaces_and_catalog_correspond` asserts `outcomeClasses ==
    [applied, warning]`; the library schema (`📚️library/🧬️schema/🔣️.json` `outcomeClasses.items.enum`) admits only
    `applied|no-op|empty|disjoint|rejected` ("warning" is not a class; repo census: 2864 applied / 2306 rejected / 1696 no-op /
    0 warning). The descriptor `[applied, no-op]` is right (the diff's equal-counter branch answers `mutation.no-op`); the
    test is stale → assert `[applied, no-op]`.
  - `retained_config_cancel_and_cleanup_respect_the_production_grant` still pins the pre-S8/S16 close contract (1-byte grant →
    `Blocked`; one 1 MiB release). The shared `HomeConfigPreparation::close_step` (S16, 09-26) releases one owner per granted
    page because every framework pump grants `TYPED_OPERATION_RESULT_PAGE_BYTES` (4 KiB); `Blocked` until 1 MiB never closed
    (S8, measured). Test is stale → re-state it against the production page grant.
- 15:3x item 1 reading: census law = source-level `.action_interactive_job(…, BatchOnlyPendingRewrite)` count
  (`📜️script.ts` `runSourceCensusGate`); fixture `✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json` names the
  blockers ("filesystem binding and studio port registration…", "payload decode and authored document import…",
  "filesystem deletion… in one direct handler"); P8 routed them as "IO in `handle`, errors swallowed, empty success".
  Working examples: architect `ArchitectExchangeCommandJobFactory` (import = host `RequestFileOpen` → payload verb → pure
  reducer; exports = host `DownloadMediaExport`), studio `importSpacePackPayload` (Migrated, HostOnly), the retained
  `ArtifactCommandWork` trait (`🔌️plugin/🧵️retained-command/🦀️.rs`: multi-step `Progress{stage}` → `Complete(emit)`,
  checkpoint/restore, cancel via the job). Host facts: ShellHost intercepts `importSpace` on the landing controller and
  re-dispatches it with `{json}` (the guest reads `dsl`/`payload` only → the text never arrives); the host's file/folder
  binding is `FRAMEWORK_SYNC_CONTROLLER_ID` `attach` (`openDocument(ref, [{kind:"folder", dataClass:"persistedLocalOnly"}])`);
  the guest's catalog ports are process-global in-memory maps on wasm32-wasip2 (`LocalStorageBackbonePort` = memory fallback).
- 15:3x kernel run 1 failed to compile: the overlay lacked the 20 gitignored `🤖️generated/` dirs (e.g. `🌐️locale/🤖️generated/🦀️.rs`)
  → copied (94 MiB). Run 2 (`generated/kernel-lib-2.txt`): **1233 passed / 1 failed** in 25 s. The 5 directory-client reds +
  the open-plan fixture red of LD's 05:4x run are GREEN now (G11 socket-grant empty body 14:10, H11 channel-18 fixtures). Left:
  `os_dsl::grammar::tests::every_shipped_grammar_semio_parses_and_compiles` → `🔩️en1993/…/🧬️mutations/📝️text/📖️.grammar.semio`
  line 3 col 19 "cannot contain a Colon token": Wave C (commit d1dd02f785d, 09-26 15:13) replaced the family grammar with
  `rule x ::= …` lines (not the `.grammar.semio` dialect). The mutation op text is JSON (`📝️text/🦀️.rs` "OpText via JSON
  tokens"), the 🔤️.ebnf / 🅰️.g4 twins still state the family document (`schema norm.en1993.mutations` + payload) exactly
  like the 14 sibling families → root fix = restore the family grammar (d1dd02f785d^ content).
- 15:4x–15:58 item 1 design (decided, written in `.🧬semio/🌐hub/s13-sh1-stage/`, payload captured by `wp-sh1/sh1-capture.py`):
  - All three become `Migrated` routes of `HomeRetainedCommandJobFactory` (tool ids in fixture order, publication contracts
    bind/import = `Artifact`, delete = `Config`, exact extents: bind/delete = public-invocation scalars, import = retained wire
    budget; proof catalog; fixture rows + Home schema consts `Migrated`/lanes/blocker "").
  - **IO-owning job** `HomeCatalogWork` (editor, implements the retained `ArtifactCommandWork`): stage `validate` (reads only)
    → `Progress{space-home.catalog.validated}` (en+de preview) → job checkpoint (1 byte = stage) → stage `commit` (the one
    catalog write) → `change-catalog-generation`. Cancel between the stages writes nothing; restore resumes at commit; a
    committed job never writes twice. `import-space`/`bind-space-file` `handle` no longer does IO (text-less import =
    host `RequestFileOpen`; with text → named refusal `…requires-retained-job`). Errors are named faults
    (`s.home.import-space.{empty,not-a-studio,catalog-refused}`, `s.home.bind-space-file.{studio-invalid,path-invalid,
    unknown-studio,io-failed,filesystem-unavailable}`) — no more `let _ =` + empty success. Bind on wasm32 refuses by name
    (no filesystem in a guest; the shell's folder binding persists studios) — reachable, honest, NOT a browser file binding.
    Found + fixed on the way: bind (like persistLocally) wrote the catalog copy without tracking it, then discarded the
    draft → the studio vanished from Home; commit now admits the file-backed studio through `import_os_space_from_pack`
    (writes + tracks).
  - **Tombstone**: `deleteVirtualFileSystemNode` emits ONE config event `HomeConfigMutation::RetireLocalStudio{spaceId}` and
    does no IO (no `delete_os_space`, no `discard_draft`); `HomeConfig.retiredLocalStudioIds` (sorted, unique, ≤ 256,
    `#[value(default)]`) filters local rows in `home_space_rows` (editor + viewer); exact inverse
    `RestoreLocalStudio` (point-invertible, no-op retire inverts to itself); retained config preparation admits both;
    refusals `s.home.delete-vfs-node.{node-invalid,hub-space,already-retired,unknown-local-studio}`; label "Remove Studio
    from Home" / "Studio aus Home entfernen", destructive flag dropped (nothing is discarded). Config schema lane (rs/ts/
    graphql/json/proto) gains the field; the viewer's generated projection is regenerated with `surface-schema`.
  - Also: `[DEBUG]` prints removed from Home `openSpace`/`navigateVirtualFileSystemNode`; root `📜️script.ts`
    `home-host-panel-owner` check flipped (it demanded `batchOnlyPendingRewrite` for bind/import/delete/renameSpace — already
    stale for renameSpace since P8) → demands `migrated`; space `📜️script.ts` Ajv oracle covers the tombstone set (valid +
    duplicate / empty id / non-string / 257 ids refused).
  - Not hand-edited: the generated plugin descriptor `✏️s/🔌️plugins/🪐️space/🔣️.json` (carries `descriptorSha256`) — the
    three rows flip to `migrated` only with `describe` (rebuild-all); until then `interactiveJobCatalogOracle` and the root
    check report the stale descriptor, as designed.
