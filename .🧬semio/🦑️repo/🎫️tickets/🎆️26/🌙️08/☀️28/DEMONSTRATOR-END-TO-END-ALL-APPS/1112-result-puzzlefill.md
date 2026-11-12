# puzzlefill (11/12)
Crate semio-s-artifact-puzzle-3d-precompute-fill: 117 errors, 3 root causes (45 distinct sites).
1. Arc<SceneConfig>/Arc<FillMeshMap> inside RetireOwned derives (FillPreparationRoots -> FillClosingOwners, FillRevalidateOwners): Arc is not RetireOwned. Fix: live structs keep Arc; closing bundles hold ArcLease (new FillPreparationLease via FillPreparationRoots::into_lease; revalidate stage wraps ArcLease::new), same pattern as brush suggestions-run.
2. outbox.close_step is effect-shaped: fill-run + fill-revalidation use close_original_effect_step(cx, demand, |effect| RetainedWorkPoll::Complete(outbox.close_step(effect))) as job_close_step does.
3. RetainedWorkOutcome E uninferable in the three staging closures: annotated <RetainedCloneStep, ValueError>.
Verified: W-puzzlefill-2 hub wasm -p semio-s-artifact-puzzle-3d-precompute-fill rc 0.
Rerun: "$C/1105b-tools/chk.sh" W-puzzlefill-<n> "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-puzzle-3d-precompute-fill
Next: -p semio-hub-puzzle.

Step 2: W-puzzlefill-3 (hub wasm -p semio-hub-puzzle) rc 101, 57 errors, ZERO in puzzle crates. Remaining red is other lanes only:
- stdio txt schema (38) + deflate examples (6) + semio snapshots brep/drawing/mesh/model/value (4-6 each): sqlite snapshot receiving port / admit_shared_retirement removal (stdiosnap/hubrest/oshost).
- semio-os-flow-host-session (5): E0283/E0277 on RetainedCloneStep result (flowhost).
puzzle-3d crates (precompute-fill, geometry, viewer, editor, mutations) all compile in this map.
Labels: W-puzzlefill-2 rc0 (fill crate), W-puzzlefill-3 rc101 (hub-puzzle, upstream only).
STATUS: FIXED-verified W-puzzlefill-2

Step 3 (remodel): terminal_frame_bound!(RemodelingWindowCommandWork) removed from remodeling editor/retained-commands; the work impl now declares `terminal_frame_release_bytes() -> Some(size_of::<Self>())` (the trait default is None; puzzle works do the same).
Check: W-puzzlefill-4 hub wasm -p semio-hub-remodel-component rc 101: 0 errors in remodel code, no terminal_frame_bound mentions. All reds are stdio crates (txt-schema, jpg/gif inferences, las/bmp/mp4/avi, semio-* snapshots/models, deflate-zlib): sqlite snapshot receiving port + OpBinaryReceiving/StoreMutationFor (stdiosnap/hubrest/oshost lanes).
Remodel verified only up to those blockers (remodel crates not reached or no errors); rerun after stdio lanes land.
STATUS: FIXED-verified W-puzzlefill-2 (fill); remodel edit unverified-to-completion, blocked on stdio reds (W-puzzlefill-4)
