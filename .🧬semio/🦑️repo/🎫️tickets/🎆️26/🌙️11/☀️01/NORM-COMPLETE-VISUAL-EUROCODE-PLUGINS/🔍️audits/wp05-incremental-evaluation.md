# WP-05 Incremental Evaluation

Location: `NORM/⚖️compliance/⚡️incremental-evaluation/` (mounted as `pub mod incremental_evaluation` in `⚖️compliance/🦀️.rs`). `IncrementalNormFamily` lives in `🏛️norm-error-and-family/🦀️.rs`.

## API
- `⚙️units`: `UnitId::new(check_family, entity_id)`, `EntityRef{kind,id}`, `CostTier{T0,T1,T2}`, `CheckUnit<D>{id, subjects, reads, tier, run}` (`CheckUnit::new` for plain closures, `::streaming` for progress plus `CancelToken`), `Progress`, `CancelHandle`/`CancelToken` (generation based, cooperative), `EntityEdge`, `UnitSource{units(doc), references(doc)}`, `evaluate_units::<S>(doc)` (full = ordered fold of units).
- `IncrementalNormFamily: NormFamily + UnitSource<Document = ..>` with `evaluate_incrementally` (the reference fold; a family's `evaluate` should call it).
- `🔍️read-tracker`: `ReadSet` (dot paths, `*` wildcard, prefix covers), `read_hash(DslValue, ReadSet)`, `audit_reads(unit, doc)` which perturbs every leaf outside the read set and reports paths whose change alters output (undeclared reads).
- `🕸️entity-graph`: `EntityGraph::from_edges`, `used_by`, transitive `closure`.
- `📋️planner`: `ChangedSet{paths, entities, everything}`, `plan_units` -> `Plan{units, token}` ordered tier first, then focus proximity, then document order. Selects units by read-path overlap, entity closure, stale status.
- `🧵️session`: `EvaluationSession<S>`: `open`, `update(doc, changed, focus) -> Plan` (supersedes in-flight runs, latest wins), `plan`, `run(plan, sink)`, `run_async` (yields between units, no runtime dependency), `report`, `progress` (`EvalProgress{running,done,total,cancelled}`), `revision`, `stale`, `cancel_handle`. Memo per unit keyed by read hash; cancelled units stay stale and are re-planned next time. `SharedEvaluation<S>`: the single path for editor and viewer: `commit(doc, changed, focus)` and `report_for(doc)` (catches up automatically; no manual evaluate).

## Adopting it in an artifact
1. Split `evaluate` into units keyed `(check family, entity id)`; give each its `ReadSet` (entity path, referenced entities' used fields, plus global paths such as annex, design situation, NDP overrides) and a `CostTier`.
2. Implement `UnitSource` for the family type (`units`, `references` from the EntityRef fields) and `IncrementalNormFamily`; make `evaluate` = `evaluate_incrementally`.
3. Add a test per unit calling `audit_reads` on the example document, and a full-vs-incremental equality test.
4. Replace the artifact's per-paint `NormHost::from_artifact` and the global report cache by one `SharedEvaluation<Family>` per open artifact: the editor mutation path calls `commit(doc, ChangedSet::at(paths, entities), focus)` from the mutation diff; the viewer and results windows call `report_for(&doc)`. Remove the `evaluate` command from the UI flow (WP-12 removes `document_revision_key`, `report_cache`, `NormHost` caching; nothing depends on them from the new module, so deletion needs no compatibility layer).
5. For T2 units run `run_async` on a worker and call `cancel_handle().cancel()` (or `update`) on the next edit; forward `SessionEvent::Progress` into the transient `EvalProgress` lane.

## Coordinator finding (cache-only evaluation)
`SharedEvaluation::report_for` and `commit` always return a settled report for exactly the given document, so the empty "No checks computed." state cannot occur after an edit and editor and viewer read the same result. Test: `render_right_after_edit_has_checks_without_evaluate`.

## Tests (`🧪️tests`)
- `🔬️unit`: law incremental == full over 40 seeds x 60 random mutations (field edits, reference changes, inserts, removals, cancellations); minimal re-run; hash skip; plan order; cancellation keeps stale units; progress; entity graph; read set; read audit (accepts declared, flags undeclared); async == sync; render-after-edit.
- `🥒️scenarios/🥒️.feature`: language-agnostic scenarios; oracle is the full-evaluation fold.
- `⏱️latency`: one-entity commit on 1000 members, p95 < 400 ms (section 5.7). No `repo bench` objective was added: bench_list shows no norm objectives and the bench registry is outside this crate; the latency test is the gate until a bench objective is registered.
