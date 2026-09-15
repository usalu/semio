# Document → Artifact Goal Audit (2026-09-15)

## Objective

Remove legacy **document** terminology wherever the product domain means **artifact**, without touching technical uses (retained UI tree, DOM, print/LaTeX, SPR `AppFrame::Document`, pack helpers).

## Completed waves

| Area | Outcome |
|------|---------|
| Framework manifest | `artifact_schema`, `artifact:in/out`, `artifacts.read/write`, tutorial `artifact` track |
| Framework wgpu chrome | `FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL` = **Artifact** |
| Actor / kernel | `ColdArtifactPair*`, `artifactJson`, `artifact-read/write`, presence `surface: artifact` |
| OS / plugin host | AppIo call sites, spawn/cold-pair bridges, space creation `artifactId` |
| Tool-run | Lifecycle copy + settings pointer `artifact` |
| Procedural generation3d | IO surface fixture `artifact-surface`, Import/Export Artifact labels |
| Puzzle | `ArtifactCapacity`, scope `Artifact`, editor `artifact_*` session symbols |
| All plugins | `*.play.artifact` body keys (167 files); manifest panel labels Artifact/Artefakt (24 manifests) |
| Play terminology | writer/layout/note/vcs `artifact` label keys |
| Hub | Native receipt `artifact_schema` |

## Verification run

- `bun nx run @semio-tech/framework-rs:check --skip-nx-cache` — **pass**
- `@semio-tech/framework-tool-run-rs:test-quick` — **pass**
- `cargo test -p semio-s-artifact-puzzle-3d` — 285 pass (1 pre-existing wire guard)
- `rg '\.play\.document' ✏️s` — **0** matches

## Intentionally retained

- **`UiDocument*`**, `begin_document`, document tree reconcile fixtures (retained UI batch)
- **Wire variants** `AppFrame::Document`, backbone document port effect bridge to kernel `DocumentRead`
- **Command module paths** `import-document` / `export-document` (internal ids may still say `importDocument`)
- **`encode_document` / `parent_document_id`** at framework IO boundaries until a dedicated pack rename
- **Layout product docs** in `📏️layout/AGENTS.md` still describe layout-domain “document” as an editorial product (PDF pages); code paths use artifact panel/body keys
- **Hub admin** `admin.documents.*` strings (connection catalog UI, not semio artifact model)
- **Historical tickets** under `.🧬semio/🎫️tickets` (audit records)

## Follow-ups (optional)

- Rename import/export command directories and tool ids to `*artifact*`
- Rename `parent_document_id` on host request types
- Command category id `document` in engine-contract (not artifact panel)

## Subagent reports

Parallel exploration and implementation used 14 subagents; detailed notes also in:

- `📓️play-body-artifact-rename-2026-09-15.md`
- `📓️puzzle3d-editor-document-to-artifact-2026-09-15.md`
