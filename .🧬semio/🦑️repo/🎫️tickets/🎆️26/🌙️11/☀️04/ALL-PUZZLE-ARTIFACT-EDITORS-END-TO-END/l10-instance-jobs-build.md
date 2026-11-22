# L10 instance-jobs / typed-ops / board-host build diagnosis

## instance-jobs (3 errors) -- fixed, `cargo check` green
- `ReservedCommitMetadata` and `FrameworkImportNaturalSourceJobFactory` landed in 👷️job/🏛️reserved/🦀️.rs (mtime 02:34) after the consumer edits (02:12-02:15): half-landed, no action needed.
- `OriginalConstructionAdmission::retire_born_candidate_step` (used by instance-jobs reserved staging, render reserved-jobs, action-bus wire tests) never existed (`git log -S` empty). Added in 🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs as `Result<bool,OriginalTurnFailure>` delegating to `close_step` (true once terminal).
- Check: `l10-check.sh <Cargo.toml>` (unshare tmpfs 3g, --jobs 2): instance-jobs Finished (l10-check3.log).

## typed-ops E0061 -- fixed
- 🪆️child-group (02:13) now takes `remaining:RetainedCloneGrant`; pump (🏃️mounted-run-and-pump) already accounts the owned-child route into `spent`. Pump now passes `remaining_plugin_turn_grant(turn,*spent)`. typed-ops compiles.

## instance-commands (9 errors) -- fixed
- `ActionIngressSource` lacked `original_press_selected:bool` (added to stores action-ingress struct + constructor + artifact-app-laws fixture).
- reserved-routes: `&commit.meta` -> `commit.meta.get()` (5 sites).
- actions: reserved dispatch now receives the `&mut Option<DslValue>` slot (assembled/prepared/args).

## Remaining blocker (NOT fixed): semio-plugin-instance-app, 4 errors (l10-check5.log)
Another agent's unfinished refactor (working tree vs HEAD: 46 files in 🧵️instance, plugin-app-impl being split into 🪢️members etc.; edits stopped ~02:18):
- 🔌️plugin-app-impl/🎟️picker-publication: calls `DispatchCtx::advance_original_reserved_picker`; removed from 📨️dispatch producers (HEAD had it plus render `advance_reserved_picker`/`original_picker`).
- 🔌️plugin-app-impl/🎟️natural-publication (new file): calls `advance_original_reserved_natural_source(input: &mut Option<NaturalFilePagedSource>, meta, &mut operation, cx)`; not implemented anywhere (whole natural-source producer path: render side + `NATURAL_SOURCE_TOOL_*` mount).
- plugin-app-impl/🦀️.rs:270 `self.tools.public_tool_contracts()` missing on `ToolOperations` (HEAD had `VcsArtifactApp::tool_public_contracts` delegating to it; the inherent method moved/removed).
- plugin-app-impl/🦀️.rs:379 `TimeTravelLedger::original_close_diagnostic` is `pub(crate)`; needs `pub`.
These need the owner's natural-source design; further crates downstream (board-host) not yet reached.
