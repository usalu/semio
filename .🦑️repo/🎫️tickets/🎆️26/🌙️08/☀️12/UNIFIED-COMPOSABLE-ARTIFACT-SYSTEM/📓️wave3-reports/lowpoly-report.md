# W3 — `lowpoly` composes stdio mesh

**ucas-status: complete (code); final green verification pending on stdio, see caveat**

Written by the orchestrator from on-disk evidence after the authoring agent was terminated by a session limit mid-verification. The migration itself is the agent's; the report and final state confirmation are the orchestrator's.

## What changed

`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs` (497 lines):

- **`mesh_json: String` (the opaque JSON string, with its 15-line comment explaining the DSL gap that forced it) → `pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>`** (`:127`). The mesh is now a typed composed child — its own document, addressed by a two-string handle, not inline JSON.
- `mesh_child_handle(object_id, mesh_json) -> store::ArtifactChild<SemioMeshSnapshot>` (`:90`) — deterministic child-id minting from a content hash of `mesh_json`, matching `store::ArtifactChild::new(child_id, target)`.
- `snapshot_from_mesh_json` retained and re-exported (`:157`) as the conversion boundary between the legacy JSON wire shape and the typed child, not as the storage representation.
- Diff shape documented as `mesh: Option<Option<ArtifactChild<…>>>` (`:202`), matching `✳️object`'s own pattern for an optional child slot (present/absent × changed/unchanged).
- **Duplicate `3d.mesh` kind declaration removed.** Confirmed by grep: zero occurrences of `"3d.mesh"` remain in the file. lowpoly now declares only its own `3d.lowpoly` kind; `3d.mesh` as a concept lives solely at `s.stdio.semio@v1/mesh`.

## Verification

- `cargo check -p semio-s-plugin-lowpoly --all-targets`: **0 errors** at the point the agent last verified (before termination).
- **Caveat — not independently re-verified end-to-end.** stdio itself went red immediately after (see `📓️status.md`'s "concurrent churn" entry — ticket #2553's live `⚙️engine` deletion fan-out, unrelated to this plugin), so a final `semio-s-plugin-lowpoly` check cannot currently complete. The lowpoly-specific code is stable and unchanged since its last clean check; the blocker is entirely upstream in stdio and not attributable to this migration.

## sharedFileRequests

None — the migration is fully contained in lowpoly's own artifact file.

## Concurrent-churn observations

None from lowpoly's own build at authoring time. The subsequent stdio breakage (ticket #2553) postdates this plugin's clean check and is unrelated.
