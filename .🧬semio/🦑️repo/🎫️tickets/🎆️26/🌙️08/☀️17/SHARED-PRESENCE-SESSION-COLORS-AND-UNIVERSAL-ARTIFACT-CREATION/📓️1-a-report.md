# Lane 1-A report — Manifest controls (C8.1) and PluginBuilder schema stamping (C8.2)

## Status: code written, gates NOT run — blocked by a host-level filesystem/sandbox outage (see below)

## Changed files

- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
  - `ActionArgControl` gained `ArtifactKind { roles: Vec<AppRole> }` and
    `SurfaceApp { roles: Vec<AppRole>, dialect_arg: String }` variants (right after the existing
    `IconSelect { classifier_kind }` variant it mirrors).
  - `ActionArgDef::artifact_kind(id, label, roles)` and
    `ActionArgDef::surface_app(id, label, roles, dialect_arg)` constructors added next to
    `::select`/`::vec3`.
  - `missing_required_args`'s doc comment updated to name `ArtifactKind`/`SurfaceApp` alongside
    `Text`/`Select`/`IconSelect` — no logic change was needed: the existing
    `Some(DslValue::String(text)) => text.is_empty()` arm already treats every String-typed control
    uniformly, and both new controls resolve to a JSON-string effective value exactly like `Select`.
  - New region `//#region 🔖️HostResolvedArgs` (placed right after `PluginManifest`, before
    `//#region 🔖️DependencyGraph`, since it depends on `PluginManifest`/`AppRole`/`AppRef`/
    `ArtifactDialect` all already being defined above that point):
    - `ArtifactKindChoice { kind_id, schema, dialect: ArtifactDialect, label: LocalizedLabel }`
    - `SurfaceAppChoice { app: AppRef, role: AppRole }`
    - `encode_artifact_kind_choice` / `decode_artifact_kind_choice` — frozen JSON shape
      `{"kindId":..,"schema":..,"dialect":{...ArtifactDialect's own camelCase Serialize...},
      "label":{"en":..,"de":..}}`. `label` resolves under `Terminology::Native` (the only
      terminology this wire shape carries) via `LocalizedLabel::resolve`/`::native`.
    - `encode_surface_app_choice` / `decode_surface_app_choice` — `{"pluginId":..,"appId":..,
      "role":"editor"|"viewer"}`.
    - `artifact_kind_choices(manifests: &[PluginManifest], roles: &[AppRole]) -> Vec<ArtifactKindChoice>`
      — pure, dedupes by `ArtifactDialect::to_coordinate()` via a `BTreeMap` (first
      manifest/app wins — owner-first ordering is the caller's responsibility, matching the
      contract's "callers pass owner manifests first" note), filters to apps whose `role ∈ roles`
      and whose `io.document_schema` is non-empty, sorted by coordinate for free via the
      `BTreeMap`'s key order.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (my full lease — the
  `PluginBuilder` struct/impls live here, not in the neighboring huge `🔌️plugin/🦀️component.rs`)
  - C8.2: `PluginBuilder::editor::<E>(def)` and `::viewer::<V>(def)` now stamp
    `def.io.document_schema = E::DOCUMENT_SCHEMA` / `V::DOCUMENT_SCHEMA` when `def.io.document_schema`
    is empty (both methods now take `mut def` and mutate before wrapping in `App { definition: def, .. }`).
    Left untouched when the app already set a different schema.
  - New `#[cfg(test)] mod schema_stamping_tests` appended at the end of the file with:
    - `SchemaStampEditorFixture: ArtifactEditor` / `SchemaStampViewerFixture: ArtifactViewer` — minimal
      fixtures built entirely from existing `NoConfig`/`NoConfigMutation`/`NoDraft`/`NoDraftMutation`/
      `NoPresence`/`NoPresenceMutation`/`NoTransient`/`NoTransientMutation` stand-ins (no new
      Snapshot/Mutation machinery needed).
    - `editor_stamps_document_schema_from_the_type_when_left_empty`
    - `editor_does_not_overwrite_an_explicitly_set_document_schema`
    - `viewer_stamps_document_schema_from_the_type_when_left_empty`
    - Each builds a minimal `AppDefinition` via `Editor::builder(dialect)...build_definition()` /
      `Viewer::builder(...)` (which leaves `io.document_schema` empty by construction — asserted as a
      fixture precondition), registers it through `Plugin::builder(...).editor::<Fixture>(def)` /
      `.viewer::<Fixture>(def)`, calls `try_build()`, and asserts
      `plugin.manifest.apps.iter().find(|a| a.role == AppRole::Editor/Viewer).unwrap().io.document_schema`
      equals the fixture's `DOCUMENT_SCHEMA` (or the explicitly-set value in the "does not overwrite" case).

## NOT done — TS twin, ts-rs regen, and ALL gates, due to an environment outage (see below)

- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`: **not edited**. I could not read its current
  content (see blocker) to match its existing conventions (how it already mirrors `ActionArgOption`/
  `ActionArgControl`, whether `LocalizedLabel`-like fields already have a hand-rolled
  `{en,de}`-resolution helper I should reuse). A full draft of the intended additions
  (`ArtifactKindChoice`, `SurfaceAppChoice`, `encodeArtifactKindChoice`/`decodeArtifactKindChoice`,
  `encodeSurfaceAppChoice`/`decodeSurfaceAppChoice`, `artifactKindChoices`) is staged at
  `/private/tmp/claude-501/-Users-ueli-Documents-semio/aaa11980-6dbf-43d6-994d-293a2b1cc4de/scratchpad/artifact-kind-choice-ts-draft.ts`
  for whoever resumes this lane — it is NOT applied to the repo and NOT verified against the real
  file's existing shapes/imports.
- `bun nx run @semio-tech/framework:generate` / `:check` — not run.
- Rust tests: NOT added-and-run for `ArtifactKindChoice` JSON round-trip / `artifact_kind_choices`
  dedupe+ordering+role-filtering in `🛂️manifest/🦀️component.rs` (the brief's "Tests" section item 1) —
  I wrote the PluginBuilder-side tests (see above) but ran out of usable session time on the outage
  before adding the manifest-side round-trip/resolver tests. **This is real unfinished scope**, not
  just "blocked" — flagging explicitly so it is not missed.
- `cargo test -p semio-framework --lib`, `cargo test -p semio-framework-plugin --lib`,
  `bun nx run @semio-tech/framework:check` — **none were run to completion**. Every attempt failed
  before reaching real compilation/test execution (see Blocker below). I am not claiming any pass/fail
  counts because I never observed one — per CLAUDE.md I will not claim a test passes (or fails) when I
  did not run it.

## Blocker (host-level, not lane-specific — affects the whole ticket, likely every lane)

Starting partway through this session, this session's tools began failing with
`EPERM: operation not permitted` on essentially every access (Read tool, `cat`/`stat`/`touch` via Bash,
and every subprocess — `cargo`, `python3`, `perl` — failing at their own `getcwd()` call with
`error: Unable to proceed. Could not locate working directory.: Operation not permitted (os error 1)`)
against **pre-existing** files under `/Users/ueli/Documents/semio`, while **brand-new** files I create
in the exact same directories (e.g. `.../🛂️manifest/🧪️1a-outage-probe2.txt`) open/read/write fine
immediately. `xattr -l` on an old file also fails with the same EPERM; on a new file it shows
`com.apple.macl` / `com.apple.provenance` present — i.e. this looks like a stale/corrupted macOS
sandbox-extension ACL (`com.apple.macl`) on already-touched files, not a POSIX-permission problem
(`stat` shows completely normal `-rw-r--r-- ueli:staff` bits and a plausible mtime on the affected
files; `ls`/ordinary Unix permission checks are not the failing layer).

Verified NOT session-specific: a **freshly spawned subagent** (its own process/session) hit the
identical `EPERM` reading `🛂️manifest/🦀️component.rs` and the identical `cargo check` "Could not
locate working directory" failure on its very first attempt — so this is not something wrong with my
own session's state; it is host/environment-wide right now. I also could not `ls`/`cd` into the repo
root or the ticket folder itself (directory *listing* denied) even though opening an exact known path
inside them still worked for new files. `CLAUDE.md` (a symlink to `AGENTS.md`) and `AGENTS.md` itself
are both affected too, so this is not scoped to files this lane touched — it looks like it could be
blocking every concurrent lane's verification the same way.

I polled repeatedly (one 2-minute loop, one 5-minute loop, ~9 minutes of continuous polling total plus
manual probing before and after) with zero recoveries on any pre-existing file, though *some* recovery
did happen partway through (bare filesystem access came back enough to write/read new files, whereas
earlier in the outage even that failed). I did not find any workaround available from inside a sandboxed
session — this needs the host/harness's sandbox layer restarted or repaired by whoever operates it.

## sharedFileRequests

- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
  **Region**: `fn validate_arg_defs` (around line 641, not inside any named `//#region` — it sits
  between the end of `🔖️InteractionArgs` (521) and the start of `🔖️ArtifactKind` (693); NOT inside my
  leased `PluginBuilder`/testkit regions, so I did not touch it).
  **Exact change requested**: mirror the existing `Select` non-empty-options assertion for the two new
  controls — i.e. add, alongside the existing
  `if let semio_framework::ActionArgControl::Select { options } = &arg.control { assert!(!options.is_empty(), ...) }`,
  an equivalent arm for `ActionArgControl::ArtifactKind { roles }` and `ActionArgControl::SurfaceApp { roles, .. }`
  asserting `!roles.is_empty()` (message on the pattern of `"app {} {} arg {} is a {ArtifactKind|SurfaceApp} with no roles"`).
  **Why**: contract §C8.1 / worker brief item 3 says `validate_arg_defs` must "treat both exactly like
  Select" — for `Select` that means "at least one option"; the structural analogue for these two
  host-resolved controls is "at least one role". This file's `validate_arg_defs`/testkit regions are
  owned by lane P3 this wave per the ownership table, so I am not editing it myself.

## What is NOT done (summary)

- TS twin (`🟦️component.ts`) — not written to the repo (draft only, in scratchpad).
- ts-rs regeneration/check — not run.
- Manifest-side round-trip/resolver tests (`ArtifactKindChoice` JSON round-trip,
  `artifact_kind_choices` dedupe/ordering/role-filtering) — not written.
- `validate_arg_defs` roles-non-empty check — out of lease, filed as a sharedFileRequest above, not
  applied.
- All three gate commands — not run to completion; no pass/fail counts to report, and I am not
  fabricating any.
