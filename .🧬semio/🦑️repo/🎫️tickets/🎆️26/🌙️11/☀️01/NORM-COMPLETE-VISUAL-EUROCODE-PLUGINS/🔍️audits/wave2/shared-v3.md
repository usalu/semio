# Shared agent v3, wave 2

Nothing compiled by me. Rebuild requested in integrate/latest.md and all build-en*/latest.md (compliance split + scene v3).

## Urgent
- Compliance split done: core crate `⚖️compliance/🧱️core` (1650 lines) + facade (2668 lines), see requests.md. Lock entry and build-owners entry hand-written; infra must run sync.
- Coordinator's scene-kit errors fixed: `Snapshot: 'static` (E0309), explicit imports (E0433).

## Blockers a/b/c solved (framework)
- (b) Owners: schema `Ownership` side table (kind, id, owner EntityRef) in `StructuralScene`/`SceneDelta`, twins, diff/apply, validation; `SceneBuilder::owned_by/annotate/track/absorb`; `Fragment.owners`. A side table keeps the 180 annotation constructors untouched.
- (a) `StructuralProjection::finish(doc, eval, ctx, &mut scene)`; `assemble_with` = merge, neutral description, finish, visibility, validate. Trait is now GAT `Document<'a>`, batch `fingerprints`, `entity_dependencies -> Vec<String>`.
- (c) `ReportIndex` (projection/🗃️report-index): rows by subject, versioned slots, `follow` returns changed subjects, `slice(subjects)` in report order (summary default), `stamp`. Fragment key includes the stamp of {entity, transitive dependencies, global rows}; `project_entity` receives only that slice; `finish` receives the summary. 5 unit tests.
- `projection/🧲️scene-kit`: `SceneKit` (5 methods) + `Kit<K>` implements the trait; `project_whole` (one unrestricted fragment, canonical order, finish) so full and incremental share code; generic fingerprints from `ToValue` rows (own rows, rows naming the entity in a string field, orphan rows are global) plus `layout_stamps`. 3 unit tests incl. incremental == full.

## EN 1990 pilot
`Out::scoped(Part)` (owner cursor, keeps/filters entities, annotations, tracks), loop guards (`out.wants`) in members, decks, actions, `out.global()` in badges and the combination track, `entities_of`, `En1990Kit`, `fragment`, `live_scene_of` in editor and viewer `scene_of`. Tests (`🎬️scene/🧪️tests/🔬️live`): incremental equals full on 60 random edits (span, rd, add/remove member, gk, qk, placements, active combination), only the edited fragment rebuilds, drag preview p95 < 12 ms (x20 in debug) on 200 members.

## Not done
- Nine other kits: add Out::scoped/owner cursor/guards/`finish` split/`SceneKit` impl, switch `scene_of`/`render_with` to `live_scene_of`. Recipe = the EN 1990 diff (kit.rs Out + finish, scene root fragment/project/kit impl, module loop guards, global emitters `out.global()`, tests copied). They currently emit empty `owners`.
- Viewport-config persistence: crate not reported green.
- Unverified by compiler: everything. Risks: fingerprint completeness (law test is the guard), summary fn reading rows other than `summary`, GAT lifetimes in macro.
