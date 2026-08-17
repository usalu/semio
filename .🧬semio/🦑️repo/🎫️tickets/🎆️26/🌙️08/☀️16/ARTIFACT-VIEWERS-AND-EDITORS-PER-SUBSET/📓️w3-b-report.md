# W3-B Report — Mutation Roster Restoration + Dev Boot Role Wiring

Lane W3-B, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Two independent cleanup jobs.

## Job 1 — restore mutation rosters across the 31 other plugin roots

Source: `📓️w2-sdk2-report.md`'s "Follow-up owed to other W2 plugin packets" — lane W2-SDK2 split the
`SemanticMutation` bound off `PluginBuilder::editor`/`::viewer` into opt-in
`.editor_mutation_roster::<E>()`/`.viewer_mutation_roster::<V>()` calls, fixed stdio itself, and flagged
31 other plugin-root files that still call `.editor::<E>()`/`.viewer::<V>()` alone.

### Method

For every plugin named in that report, read the plugin root's `.editor::<E>(def)`/`.viewer::<V>(def)`
calls, traced each `E`/`V`'s `type Mutation = …` (via its `impl ArtifactEditor`/`impl ArtifactViewer`
block), then checked whether that `Mutation` type carries `#[derive(dsl::Mutations)]` or
`#[derive(protocol::Mutations)]` — confirmed both re-export the same derive macro
(`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1077`,
`impl ::protocol::SemanticMutation<#snapshot_ty> for #name`), so either spelling satisfies the bound.

**Finding: unlike stdio's codec-derived subsets (32/40 hand-rolled), every one of these 31 "native"
plugins' own artifact mutation enums already derives `Mutations`.** Verified with a script that walked
every editor/viewer's `Mutation` type and its nearest preceding `#[derive(...)]`, then cross-checked by
grepping for any hand-rolled `impl protocol::Mutation for`/`impl Mutation for` inside each plugin
directory — the only hit (💡️reasoning/🔌️wires) was a doc-comment mentioning the OLD pattern, not real
code. So every `.editor::<E>()`/`.viewer::<V>()` call across all 31 files gets the matching
`_mutation_roster` opt-in chained right after it — none were skipped.

### Files touched (32 `🦀️component.rs`, all plugin roots — no schema/io/surface file touched)

`✒️writer`, `➗️mathematical`, `🌀️procedural`, `🌊️flow`, `🌍️gis`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`,
`🎬️sequence`, `🏗️fem`, `🏛️architect`, `🏭️process`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📏️layout`,
`📐️cad`, `📕️norm` (15 dialects, all 15 pairs rostered), `📖️playbook`, `📜️imperative`, `📸️remodel`,
`🔋️energy`, `🔱️trinity`, `🕸️dag`, `🖍️draw`, `🖨️raster`, `🗒️note`, `🧩️puzzle`, `🧱️block`, `🪐️space`,
`🪵️sourcing`, `🎪️demonstrator/🛂️manifest/🎪️demonstrator`.

`🎪️demonstrator`'s manifest also chains the roster for its four **foreign** editor contributions
(`Procedural3dPlayApp`, `CadPlayApp`, `Puzzle3dPlayApp`, `Gis2dPlayApp`) — the same types already
rostered from their own owning plugin, so this is an idempotent re-registration
(`commit_owner_mutation_roster`'s documented discipline), not a new roster row. Its two still-
`.document_app::<X>()`-registered foreign apps (`SourcingCurateApp` via `sourcing::apps::curate`,
`Process3dPlayApp` via `process::apps::process3d` — note: different module path than the rostered
`process::editor::process3d`, i.e. a different, still-un-migrated surface) were left untouched:
`document_app` is a separate, not-yet-deleted builder method that already auto-registers its roster
unconditionally (contract §2.1's `document_app` deletion is scoped elsewhere, not this job).

Verified with a script (`analyze2.py`, scratchpad) that re-parsed every edited file's
`.editor::<X>(`/`.viewer::<X>(` calls against `.editor_mutation_roster::<X>(`/
`.viewer_mutation_roster::<X>(` calls: exact 1:1 match in every file, zero missing, zero extra (doc-
comment mentions of bare `.editor::<E>(…)`/`.viewer::<V>(…)` in four files' module docs were confirmed
as false positives, not code).

### Verification actually run

`RUSTC_WRAPPER="" cargo check -p <crate> --all-targets --keep-going`, serial, output in
`🧪️w3-b-cargo.txt`:

- `semio-framework-plugin` — **0 errors**.
- `semio-s-plugin-mathematical`, `semio-s-plugin-gis`, `semio-s-plugin-fem`, `semio-s-plugin-norm`,
  `semio-s-plugin-block`, `semio-s-plugin-demonstrator` — every one of these six depends on
  `semio-s-plugin-stdio` (mathematical/demonstrator directly via `Cargo.toml`; the others transitively
  through the workspace). Every run failed to compile **stdio** (80–165 errors across runs, count
  moving between runs), but **zero errors in any run carried a primary `-->` location inside any file I
  touched** — confirmed by filtering every `-->` line against `🗄️stdio` and finding nothing left.
  `stdio` is explicitly called out as "known-broken by live peers" in this lane's brief; confirmed live
  with `git status --porcelain` (`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` etc. all uncommitted `M`/`D`)
  and `stat` mtimes ~17:37, i.e. during this session — the FULL-STDIO peer ticket actively growing it,
  same class of churn `📓️w2-sdk2-report.md` and prior lanes already documented. Not my regression, not
  my lease.

## Job 2 — dev boot role + docs

### Dev boot role wiring

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🟦️component.ts:19+` — added
  `const appRole: "viewer" | "editor" = import.meta.env.VITE_SEMIO_APP_ROLE === "viewer" ? "viewer" : "editor"`
  next to the existing `appId` resolution, passed into `bootFrameworkOs({ …, appRole, … })`. Mirrors
  `resolveBootAppRole`'s own validation (contract §5: `viewer`/`editor`, default `editor`) so dev boots
  the editor unless the env var explicitly says viewer.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🧩️multi.tsx:24+` — same resolution as
  `MULTI_HARNESS_APP_ROLE` (shared by every harness pane, since the harness has no existing per-pane
  role selector), threaded into `<FrameworkOsShell … appRole={MULTI_HARNESS_APP_ROLE} … />` alongside
  the existing `appId={boot.defaultAppId}`.
- Confirmed `FrameworkOsShellProps.appRole?: AppRole` already exists
  (`ShellHost/🟦️component.tsx:680`, landed by lane 1-C) and that `establishPrimarySession`'s pick order
  is `(appId ? apps.find(id match) : undefined) ?? apps.find(role match) ?? apps[0]`
  (`ShellHost/🟦️component.tsx:1021`) — so the role preference activates correctly as a fallback even
  before the stale-`defaultAppId` fix below landed (an unmatched `appId` yields `undefined`, falling
  through to the role match).

### `defaultAppId: "cad-play"` staleness — regenerated through its generator

`🤖️generated/🟦️session.ts` was stale: `defaultAppId: "cad-play"`, an old pre-role-split id, while the
source-of-truth `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`
`[[package.metadata.semio.playground]]` row for `koordinator` already declared
`app = "s.cad.cad@1/*#editor"`. Per instruction, did **not** hand-edit the generated file — ran its
generator (`bun ./📜️script.ts plugin registry koordinator` from the dev TS package, which calls
`ensurePluginRegistry` → `writePlaygroundSession`).

First attempt hit a real, live bug: `🧧framework/…/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️script.ts`'s
`discoverExamplesForPlayground` still read `TAXONOMY.appsDirName` (`APPS_DIRNAME`), which had already
gone `undefined` — the taxonomy's `appsDirName` key was already deleted (`git log` confirms
`🔣️taxonomy.json` had an uncommitted edit at 17:49:49, i.e. mid-session, ~schemaVersion 5 already
current) by the live W3 dissolve-`🎛️apps` lane. This is a foreign registry file, not in this lane's
lease, so it was not touched. Re-ran a few minutes later: the same peer lane had landed its own fix to
`discoverExamplesForPlayground` (git diff shows `APPS_DIRNAME`/`appsDir` replaced by
`surfaceDirsForPlugin`/`surfaceStripEmoji`, mtime moved to 18:11:54) in the interim, and the generator
then succeeded cleanly. Regenerated `🤖️generated/🟦️session.ts` now reads
`defaultAppId: "s.cad.cad@1/*#editor"` and its `plugins` list also refreshed (demonstrator's crate
dependency graph had grown since the file was last generated — `cad`/`gis`/`process`/`stdio` etc. now
listed as first-class entries, not just their extension crates).

### `🧰️framework/🛍️products/💻️os/AGENTS.md`

Rewrote the `🔌️Plugin`/`🎛️App` sections (the only two describing the retired manifest+apps model) to
the real one:

- `# 🔌 Plugin` — "A plugin is a manifest and a collection of artifacts." (was "…apps").
- `# 🎛️ App` → retitled `# 🎭 Surface` (the runtime `App`/`AppDefinition` types persist, but the
  user-facing *concept* this section defined — "one thing per app" — is now "one of two roles per
  subset"): "A surface is a role-carrying app over one artifact subset: an editor (mutation-capable) or
  a viewer (read-only). Every artifact subset registers one of each. A surface is addressed as
  `<kind>@<standard>/<subset>#<role>`." (was the unfinished stub "An app has a engine, ").

No other `AGENTS.md` file was opened or touched, per instruction and CLAUDE.md's blanket rule.

### READMEs under `💻️os/**`

Enumerated every non-`node_modules`/non-`dist` `README.md` under `🧰️framework/🛍️products/💻️os/**`:
`🧫️fixtures/README.md`, `🔨️modules/♾️infinite/README.md`, `🔨️modules/🧠️neural/README.md`. Grepped each
for `🎛️apps`, `document_app`, `apps/`, `ArtifactApp` and app/surface/editor/viewer language generally —
none reference the retired plugin/app model (`♾️infinite`/`🧠️neural` are 6-line stubs; `🧫️fixtures`'s
`app-1`/`app-2` are generic DSL example node names, unrelated to the plugin surface concept). No README
edits made.

## Verification actually run (job 2)

- `bun nx run @semio-tech/plugin-registry:check` — failed on the first run ("plugin registry catalog is
  stale") before I ran `generate`; **passed clean on the re-run** after regeneration (some
  pre-existing, unrelated `manifest-without-marker` warnings for other plugins' package.json/Cargo.toml
  — not something this lane's files trigger). `git status --porcelain .vscode/launch.json` — empty,
  confirming the regenerated launch.json is byte-identical to what's already checked in (this lane
  registered no new executable command, so no manual `📇️registry/🖥️launch.ts` entry was needed).
- Dev package's own test target, read from its `📋️project.json` (`@semio-tech/framework-os-dev`,
  target `test` → `bun ./📜️script.ts test`), run twice (before and after the registry regen): **16/16
  passed** both times, including the two `import.meta.vitest`-guarded in-source test files that cover
  `🟦️component.ts` and the root `📜️script.ts` module graph.

## Not done, and why

Nothing outstanding from either job. The one live blocker encountered (job 2's registry-generator bug)
self-resolved mid-session when the concurrent W3 dissolve-`🎛️apps` lane landed its own fix to the same
file; re-running the generator afterward produced the correct, fresh output, so no workaround or
hand-edit was needed or left behind.

## Files touched

Job 1 (`✏️s/🔌️plugins/**` plugin-root `🦀️component.rs` only, 32 files):
`✒️writer`, `➗️mathematical`, `🌀️procedural`, `🌊️flow`, `🌍️gis`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`,
`🎬️sequence`, `🏗️fem`, `🏛️architect`, `🏭️process`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📏️layout`,
`📐️cad`, `📕️norm`, `📖️playbook`, `📜️imperative`, `📸️remodel`, `🔋️energy`, `🔱️trinity`, `🕸️dag`,
`🖍️draw`, `🖨️raster`, `🗒️note`, `🧩️puzzle`, `🧱️block`, `🪐️space`, `🪵️sourcing`,
`🎪️demonstrator/🛂️manifest/🎪️demonstrator`.

Job 2:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🟦️component.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🧩️multi.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🤖️generated/🟦️session.ts` (regenerated via its
  generator, not hand-edited)
- `🧰️framework/🛍️products/💻️os/AGENTS.md`
- `.vscode/launch.json` (regenerated as a side effect of the catalog regen; content unchanged from
  what was already checked in)

Not touched: any other `AGENTS.md`; any README (none needed changes);
`🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` (foreign lease, fixed live by a peer);
`🔣️taxonomy.json` (foreign lease, live peer edit).

Scratch (ticket folder): `🧪️w3-b-cargo.txt`.
