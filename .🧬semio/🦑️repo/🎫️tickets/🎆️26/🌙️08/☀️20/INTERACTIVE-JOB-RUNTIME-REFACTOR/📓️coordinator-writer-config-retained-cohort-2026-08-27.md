# Writer Config Retained Cohort

## Outcome

Writer now exposes all eighteen Writer-owned routes through one retained job factory: eight Config-only routes, five bounded Host-only document-load routes, two Artifact-only text routes, and three mixed/Artifact text-compute routes. No Writer-owned route remains `BatchOnly`. The framework-injected `recordTutorial` route is no longer duplicated in Writer's command vocabulary. Artifact and Config publication each use an app-owned `ArtifactStoreOneItemPreparationFactory` with exact operation/generation/base-revision checks, one-item advancement, Store-sealed edit digests, cancellation, and bounded terminal retirement.

## Exact Classification

- Migrated Config: `setCamera`, `requestCompletions`, `lintDocument`, `setEditorSelection`, `toggleLineNumbers`, `setEditorSetting`, `engagementInput`, `setLocale`.
- Migrated Host-only: `setActiveExample`, `setSnapshot`, `openDocument`, `setSnapshotJson`, `setFixtureJson`.
- Migrated Artifact: `textEdit`, `setText`, `commitRename`.
- Migrated Artifact + Config: `formatDocument`, `engagementSubmit`.
- BatchOnly: none.
- Framework-owned: `recordTutorial`, supplied once by `FrameworkRecordTutorialJobFactory` and shell interception rather than a no-op Writer command duplicate.
- Publication: all eight migrated routes declare the exact `Config` lane. The backing Store history lane is `Document`, matching the Store API.
- Preparation envelope: one work item and at most 8,192 retained string bytes per mutation/base.
- Artifact preparation envelope: one work item, 4,096 bytes per `EditText`, and a 32,768-byte admitted base root including its composed text owner.

## Evidence

- Language-neutral fixture: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧵️interactive-job-migration.json`.
- Rust source law covers exact forward/post/inverse parity and maximum-plus-one rejection.
- `jq` parsed the fixture successfully and `git diff --check -- ✏️s/🔌️plugins/✒️writer` was clean.
- Official verifier output: `📊️coordinator-official-tool-jobs-live-r13-working-writer-complete-2026-08-27.json`. It reports Writer remaining commands `[]`, Writer scan-then-monolith rows `[]`, and Writer process-global payload stores `[]`. Whole-repository debt in r13 remains 592 commands, 120 unlaned retained routes, 27 scan-then-monolith routes, 16 process-global candidates, four reserved routes, and 36 import-media routes.

## Pending Validation

Writer Cargo, Wasm, Nx, and rustfmt validation is queued behind the exclusive Flow compiler lease. This report does not claim those gates are green.
