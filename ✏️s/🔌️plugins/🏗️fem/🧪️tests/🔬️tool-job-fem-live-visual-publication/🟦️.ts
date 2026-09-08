import { toolJobFemLiveVisualPublicationExact } from "../../../../../📜️script.ts";

/** 🧪️ Executes tool job fem live visual publication policy assertions. */
export function toolJobFemLiveVisualPublicationSelfTests(
  fem2dModel: string,
  fem2dSession: string,
  fem3dSession: string,
  fem3dEditor: string,
  fem3dModel: string,
  fem3dResults: string,
  fem3dViewer: string,
  fem3dViewerModel: string,
  femPluginRoot: string,
  femGlue: string,
  femSparse: string,
  frameworkPlugin: string,
  frameworkWorld: string,
  worldSnapshot: string,
  canvasSnapshot: string,
  canvasRenderer: string,
  femAnalyses: string,
  femMesh: string,
): number {
  const sources = { fem2dModel, fem2dSession, fem3dSession, fem3dEditor, fem3dModel, fem3dResults, fem3dViewer, fem3dViewerModel, femPluginRoot, femGlue, femSparse, frameworkPlugin, frameworkWorld, worldSnapshot, canvasSnapshot, canvasRenderer, femAnalyses, femMesh };
  const exact = (change: Partial<typeof sources> = {}) => {
    const value = { ...sources, ...change };
    return toolJobFemLiveVisualPublicationExact(
      value.fem2dModel,
      value.fem2dSession,
      value.fem3dSession,
      value.fem3dEditor,
      value.fem3dModel,
      value.fem3dResults,
      value.fem3dViewer,
      value.fem3dViewerModel,
      value.femPluginRoot,
      value.femGlue,
      value.femSparse,
      value.frameworkPlugin,
      value.frameworkWorld,
      value.worldSnapshot,
      value.canvasSnapshot,
      value.canvasRenderer,
      value.femAnalyses,
      value.femMesh,
    );
  };
  const mutate = (source: string, from: string, to: string) => {
    if (!source.includes(from)) throw new Error("[verify interactivity tool-jobs p6i] mutation source missing: " + from);
    const changed = source.replace(from, to);
    if (changed === source) throw new Error("[verify interactivity tool-jobs p6i] mutation was a no-op: " + from);
    return changed;
  };
  const mutateLast = (source: string, from: string, to: string) => {
    const index = source.lastIndexOf(from);
    if (index < 0) throw new Error("[verify interactivity tool-jobs p6i] mutation source missing: " + from);
    return source.slice(0, index) + to + source.slice(index + from.length);
  };
  const mutateSequence = (source: string, changes: [string, string][]) => changes.reduce((current, [from, to]) => mutate(current, from, to), source);
  if (!exact()) throw new Error("[verify interactivity tool-jobs p6i] valid live visual sources were rejected.");
  let hardenedCount = 0;
  {
    const hardened: [string, boolean][] = [
      ["2D mounted whole reconstruction", exact({ fem2dModel: mutate(fem2dModel, "layers_json: String::new(), snapshot: progress.map(Fem2dMountedVisualLease::snapshot)", "layers_json: render(_doc, camera).to_string(), snapshot: None") })],
      ["2D immutable packet handoff", exact({ fem2dModel: mutate(fem2dModel, "snapshot: progress.map(Fem2dMountedVisualLease::snapshot)", "snapshot: None") })],
      ["2D renderer snapshot bypass", exact({ canvasRenderer: mutate(canvasRenderer, "if let Some(snapshot) = canvas.snapshot", "if let Some(snapshot) = None") })],
      ["2D one-page backing close", exact({ canvasSnapshot: mutateLast(canvasSnapshot, "slot.pages[usize::from(slot.admitted_pages)] = None;", "slot.pages = std::array::from_fn(|_| None);") })],
      ["3D production numerical caller", exact({ fem3dSession: mutate(fem3dSession, "let step = self.numerical.as_mut().map(|numerical| numerical.step(snapshot, solver, &mut self.backing, freshness, operation, &mut cx));", "let step = Some(Ok(true));") })],
      ["3D monolithic solver backing", exact({ fem3dSession: mutate(fem3dSession, "scalars: [Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; FEM3D_SOLVER_FIELDS_PER_PAGE]>>; FEM3D_SOLVER_PAGE_COUNT]", "scalars: Option<Box<[std::mem::MaybeUninit<Fem3dSolverScalar>; MAXIMUM_FIELDS]>>") })],
      ["3D solver page admission", exact({ fem3dSession: mutate(fem3dSession, "solver.admit_page(self.solver_page_cursor, self.solver_page_lane, backing)", "true") })],
      ["3D sparse readiness", exact({ fem3dSession: mutate(fem3dSession, "self.initialized_count != self.len || total != self.len", "completed != 0") })],
      ["3D solid MeshJob owner", exact({ fem3dSession: mutate(fem3dSession, "MeshJob::new_mounted_bounded", "MeshJob::new_bounded") })],
      ["3D solid Tet4 production", exact({ fem3dSession: mutate(fem3dSession, "Tet4 {", "Bar3 {") })],
      ["3D reaction force identity", exact({ fem3dSession: mutate(fem3dSession, "self.reaction_accumulator - self.full_rhs[row]", "self.reaction_accumulator") })],
      ["3D physical Tet4 modal mass", exact({ fem3dSession: mutate(fem3dSession, "self.modal_lumped_mass[analysis * 6 + self.mass_update_cursor % 3] += self.pending_tet_mass", "self.modal_lumped_mass[analysis * 6 + self.mass_update_cursor % 3] += 1.0") })],
      ["3D physical modal mass transfer", exact({ femSparse: mutate(femSparse, "self.stage = ModalInputStage::CopyMountedMass", "self.stage = ModalInputStage::BuildMass") })],
      ["3D fixed node owner", exact({ femAnalyses: mutate(femAnalyses, "nodes: [Option<Node>; MOUNTED_ANALYSIS_NODE_SLOTS]", "nodes: Vec<Node>") })],
      ["3D fixed element owner", exact({ femAnalyses: mutate(femAnalyses, "elements: [Option<Elements>; MOUNTED_ANALYSIS_ELEMENT_SLOTS]", "elements: Vec<Elements>") })],
      ["3D fixed support owner", exact({ femAnalyses: mutate(femAnalyses, "supports: [Option<MountedAnalysisSupport>; MOUNTED_ANALYSIS_SUPPORT_SLOTS]", "supports: Vec<MountedAnalysisSupport>") })],
      ["3D fixed analysis ids", exact({ fem3dSession: mutate(fem3dSession, "analysis_node_ids: FixedSlots<String, MAXIMUM_FIELDS>", "analysis_node_ids: Vec<String>") })],
      ["3D fixed meshed solids", exact({ fem3dSession: mutate(fem3dSession, "meshed_solids: FixedSlots<Fem3dMeshedSolid, MAXIMUM_REGIONS>", "meshed_solids: Vec<Fem3dMeshedSolid>") })],
      ["3D fixed outline points", exact({ femMesh: mutate(femMesh, "points: [[f64; 2]; MOUNTED_DOMAIN_POINT_SLOTS]", "points: Vec<[f64; 2]>") })],
      ["3D fixed hole owner", exact({ femMesh: mutate(femMesh, "holes: [MountedPlanarPolygon; MOUNTED_DOMAIN_HOLE_SLOTS]", "holes: Vec<MountedPlanarPolygon>") })],
      ["3D fixed solid points", exact({ fem3dSession: mutate(fem3dSession, "solid_points: FixedSlots<[f64; 2], MAXIMUM_FIELDS>", "solid_points: Vec<[f64; 2]>") })],
      ["3D fixed solid triangles", exact({ fem3dSession: mutate(fem3dSession, "solid_tris: FixedSlots<[u32; 3], MAXIMUM_ELEMENTS>", "solid_tris: Vec<[u32; 3]>") })],
      ["3D fixed solid node ids", exact({ fem3dSession: mutate(fem3dSession, "solid_node_ids: FixedSlots<String, MAXIMUM_FIELDS>", "solid_node_ids: Vec<String>") })],
      ["3D fixed solid node indices", exact({ fem3dSession: mutate(fem3dSession, "solid_node_analysis_indices: FixedSlots<usize, MAXIMUM_FIELDS>", "solid_node_analysis_indices: Vec<usize>") })],
      ["3D fixed rhs", exact({ fem3dSession: mutate(fem3dSession, "rhs: MountedScalarSlots,", "rhs: VecD,") })],
      ["3D fixed modal mass", exact({ fem3dSession: mutate(fem3dSession, "modal_free_mass: MountedScalarSlots,", "modal_free_mass: VecD,") })],
      ["3D fixed slot maximum guard", exact({ fem3dSession: mutate(fem3dSession, "if target > N {", "if false {") })],
      ["3D fixed analysis maximum guard", exact({ femAnalyses: mutate(femAnalyses, "if target > maximum {", "if false {") })],
      ["3D fixed domain maximum guard", exact({ femMesh: mutate(femMesh, "if target > MOUNTED_DOMAIN_POINT_SLOTS {", "if false {") })],
      ["3D fixed scalar maximum guard", exact({ femSparse: mutate(femSparse, "if target > MOUNTED_SCALAR_SLOTS {", "if false {") })],
      ["3D fixed owner maximum law", exact({ fem3dSession: mutate(fem3dSession, "fem3d_numerical_fixed_owner_maximum_plus_one_refuses_unchanged_and_closes_one_slot", "fem3d_numerical_fixed_owner_smoke") })],
      ["3D post-work model fuel", exact({ fem3dSession: mutateSequence(fem3dSession, [["            context.consume_fuel(1);\n", ""], ["            self.step_model(doc)?;\n", "            self.step_model(doc)?;\n            context.consume_fuel(1);\n"]]) })],
      ["3D post-construction mesh fuel", exact({ fem3dSession: mutateSequence(fem3dSession, [["            context.consume_fuel(1);\n", ""], ["            self.stage = Fem3dNumericalStage::SolidMesh;\n", "            self.stage = Fem3dNumericalStage::SolidMesh;\n            context.consume_fuel(1);\n"]]) })],
      ["3D whole fixed-owner close", exact({ fem3dSession: mutate(fem3dSession, "self.analysis_node_ids.pop();", "self.analysis_node_ids = FixedSlots::new();") })],
      ["3D whole normal solid-index close", exact({ fem3dSession: mutate(fem3dSession, "self.solid_node_analysis_indices.pop()", "self.solid_node_analysis_indices = FixedSlots::new(); None") })],
      ["3D refused element owner", exact({ fem3dSession: mutateLast(fem3dSession, "self.pending_element = Some(built);", "drop(built);") })],
      ["3D refused Tet4 owner", exact({ fem3dSession: mutate(fem3dSession, "self.pending_tet = Some(tet);", "drop(tet);") })],
      ["3D refused meshed-solid owners", exact({ fem3dSession: mutate(fem3dSession, "self.solid_node_ids = retained.node_ids;", "drop(retained.node_ids);") })],
      ["3D refused PCG matrix owner", exact({ fem3dSession: mutate(fem3dSession, "self.rejected_pcg_matrix = Some(matrix);", "drop(matrix);") })],
      ["3D genuine modal result", exact({ fem3dSession: mutate(fem3dSession, "subspace.visual_mode_scalar(0, equation)", "self.pcg.as_ref().and_then(|pcg| pcg.visual_scalar(equation)).map(|value| (value.displacement, value.displacement))") })],
      ["3D numerical close delegation", exact({ fem3dSession: mutate(fem3dSession, "if let Some(numerical) = self.numerical.as_mut()", "if let Some(numerical) = None") })],
      ["3D completed child retained", exact({ fem3dSession: mutate(fem3dSession, "self.numerical_done = true", "self.numerical = None") })],
      ["3D prepared field consumer", exact({ frameworkWorld: mutate(frameworkWorld, "World3dSnapshotPageKind::Status if (10..=14).contains(&item.flags)", "World3dSnapshotPageKind::Status if false") })],
      ["3D prepared status consumer", exact({ frameworkWorld: mutate(frameworkWorld, "World3dSnapshotPageKind::Status if item.flags == 20", "World3dSnapshotPageKind::Status if false") })],
      ["3D editor mounted authority", exact({ fem3dEditor: mutate(fem3dEditor, "crate::artifacts::fem3d::live_visual::with_live_visual", "crate::app_surface::without_live_visual") })],
      ["3D viewer mounted authority", exact({ fem3dViewer: mutate(fem3dViewer, "crate::artifacts::fem3d::live_visual::with_live_visual", "crate::app_surface::without_live_visual") })],
      ["3D production solid law", exact({ fem3dSession: mutate(fem3dSession, "fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized", "fem3d_production_smoke") })],
      ["3D correspondence law", exact({ fem3dSession: mutate(fem3dSession, "fem3d_production_field_correspondence_rejects_sparse_and_zero_aliases", "fem3d_correspondence_smoke") })],
      ["3D initialized page backing admission", exact({ fem3dSession: mutate(fem3dSession, "backing.claim(FEM3D_SOLVER_INITIALIZED_PAGE_BYTES)", "true") })],
      ["3D scalar page backing admission", exact({ fem3dSession: mutate(fem3dSession, "backing.claim(FEM3D_SOLVER_SCALAR_PAGE_BYTES)", "true") })],
      ["3D order backing admission", exact({ fem3dSession: mutate(fem3dSession, "backing.claim(bytes)", "true") })],
      ["3D mounted process reservation", exact({ fem3dSession: mutate(fem3dSession, "if !registry.reserve_credit(shell)", "if false") })],
      ["3D process item credit", exact({ fem3dSession: mutate(fem3dSession, "self.credit_items[slot] = FEM3D_PROCESS_BACKING_ITEMS", "self.credit_items[slot] = 0") })],
      ["3D process byte credit", exact({ fem3dSession: mutate(fem3dSession, "self.credit_bytes[slot] = FEM3D_PROCESS_BACKING_BYTES", "self.credit_bytes[slot] = 0") })],
      ["3D draw descriptor reservation", exact({ fem3dSession: mutate(fem3dSession, "draw_count: 1", "draw_count: 0") })],
      ["3D global draw byte reservation", exact({ worldSnapshot: mutate(worldSnapshot, "store.reserved_draw_bytes = reserved_draw_bytes", "store.reserved_draw_bytes = store.reserved_draw_bytes") })],
      ["3D prepared draw permit", exact({ frameworkWorld: mutate(frameworkWorld, "world3d_snapshot_claim_draw_permit(cursor.lease, 1, instance_count, draw_bytes)", "Err(World3dSnapshotFault::Capacity)") })],
      ["3D fault clone", exact({ fem3dSession: mutate(fem3dSession, "{ detail } else { b\"fem3d.visual-fault-capacity\".to_vec() }", "{ detail.clone() } else { b\"fem3d.visual-fault-capacity\".to_vec() }") })],
      ["3D retained payload clone", exact({ fem3dSession: mutate(fem3dSession, "self.fault_payload = Some(payload)", "self.fault_payload = Some(payload.clone())") })],
      ["3D solver ordinary-drop recovery", exact({ fem3dSession: mutate(fem3dSession, "impl Drop for Fem3dSolverView", "impl Reclaim for Fem3dSolverView") })],
      ["3D lease ordinary-drop recovery", exact({ fem3dSession: mutate(fem3dSession, "impl Drop for Fem3dPageVisualLease", "impl Reclaim for Fem3dPageVisualLease") })],
      ["3D candidate ordinary-drop recovery", exact({ fem3dSession: mutate(fem3dSession, "impl Drop for Fem3dPageVisualJob", "impl Reclaim for Fem3dPageVisualJob") })],
      ["3D state Drop owner transfer", exact({ fem3dSession: mutate(fem3dSession, "recovery.publish_owner(identity, owner)", "drop(owner); Ok(())") })],
      ["3D job Drop registry publication", exact({ fem3dSession: mutate(fem3dSession, "self.recovery.publish(self.identity, MountedRecoveryPublication::Recover)", "true") })],
      ["3D job Drop state transfer", exact({ fem3dSession: mutate(fem3dSession, "self.recovery.publish_owner(self.identity, state)", "Err(state)") })],
      ["3D job Drop refused-transfer restoration", exact({ fem3dSession: mutate(fem3dSession, "*shell = Some(state)", "drop(state)") })],
      ["3D recovery authority reservation", exact({ fem3dSession: mutate(fem3dSession, "recovery.reserve(identity)", "true") })],
      ["3D abandoned state incremental close", exact({ fem3dSession: mutate(fem3dSession, "state.close_step(maximum_bytes)", "PluginCloseStep::Complete") })],
      ["3D abandoned state restore", exact({ fem3dSession: mutate(fem3dSession, "recovery.restore_owner(identity, state)", "Err(state)") })],
      ["3D abandoned state credit release", exact({ fem3dSession: mutate(fem3dSession, "registry.release_credit(shell as u16)", "registry.credit_items[shell] = 0") })],
      ["3D mounted recovery maintenance drain", exact({ fem3dSession: mutate(fem3dSession, "recover_abandoned_one(&mut registry.borrow_mut(), app_instance_id, maximum_bytes)", "None") })],
      ["3D queued running state Drop law", exact({ fem3dSession: mutate(fem3dSession, "fem3d_queued_running_and_state_drop_publish_exact_identity_and_drain_one_owner", "fem3d_drop_smoke") })],
      ["3D whole recovered backing close", exact({ fem3dSession: mutate(fem3dSession, "let owner = owner.take()?;", "let owner = owner.take()?;\n        recovery.owners = [const { None }; FEM3D_RECOVERED_BACKING_CAPACITY];") })],
      ["3D whole recovered snapshot page close", exact({ worldSnapshot: mutate(worldSnapshot, "store.recovered_page_count -= 1;", "store.recovered_page_count = 0;\n        store.recovered_pages = [const { None }; WORLD3D_SNAPSHOT_RECOVERED_PAGE_CAPACITY];") })],
      ["3D orphan snapshot whole close", exact({ worldSnapshot: mutate(worldSnapshot, "slot.admitted_pages -= 1;\n        slot.pages[usize::from(slot.admitted_pages)] = None;\n        return Some((1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY));", "slot.admitted_pages = 0;\n        slot.pages = Box::new([const { None }; WORLD3D_SNAPSHOT_PAGE_CAPACITY]);\n        return Some((1, WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY));") })],
      ["3D draw and orphan recovery law", exact({ worldSnapshot: mutate(worldSnapshot, "draw_permit_is_reserved_before_publication_and_orphan_close_is_one_page", "draw_permit_smoke") })],
      ["3D backing and drop recovery law", exact({ fem3dSession: mutate(fem3dSession, "fem3d_process_permits_precede_solver_order_allocation_and_drop_handoff_closes_one_backing", "fem3d_process_permits_smoke") })],
    ];
    for (const [name, accepted] of hardened) if (accepted) throw new Error("[verify interactivity tool-jobs p6i] mutation " + name + " was falsely accepted.");
    hardenedCount = hardened.length;
  }
  const mutations: [string, boolean][] = [
    ["2D monolithic output backing", exact({ fem2dModel: mutate(fem2dModel, "pages: [Option<Canvas2dSnapshotPage>; FEM2D_MOUNTED_VISUAL_PAGE_COUNT]", "bytes: String") })],
    ["2D dynamic order backing", exact({ fem2dModel: mutate(fem2dModel, "slots: [Option<usize>; N]", "slots: Vec<usize>") })],
    ["2D whole backing close", exact({ fem2dModel: mutate(fem2dModel, "*page = None;", "self.pages = std::array::from_fn(|_| None);") })],
    ["2D packet page admission", exact({ fem2dModel: mutate(fem2dModel, "self.output.admit_page(usize::from(self.reserve_lane) - 1)", "Ok(())") })],
    ["2D pre-work fuel", exact({ fem2dSession: mutate(fem2dSession, "        cx.consume_fuel(1);\n        match candidate.step_one(snapshot, &self.visual, freshness) {", "        match candidate.step_one(snapshot, &self.visual, freshness) {") })],
    ["2D solver reaction identity", exact({ femSparse: mutate(femSparse, "reaction: -residual", "reaction: residual") })],
    ["3D monolithic page backing", exact({ fem3dSession: mutate(fem3dSession, "pages: [Option<World3dSnapshotPage>; FEM3D_VISUAL_PAGES]", "pages: Vec<World3dSnapshotPage>") })],
    ["3D dynamic order backing", exact({ fem3dSession: mutate(fem3dSession, "slots: Box<[Option<usize>; N]>", "slots: Vec<usize>") })],
    ["3D zero-initialized solver backing", exact({ fem3dSession: mutate(fem3dSession, "std::mem::MaybeUninit<Fem3dSolverScalar>", "Fem3dSolverScalar") })],
    ["3D solver generation qualification", exact({ fem3dSession: mutate(fem3dSession, "freshness != self.freshness || index >= self.len", "index >= self.len") })],
    ["3D numerical page correspondence", exact({ fem3dSession: mutate(fem3dSession, "assert_eq!(&reaction[..3], &scalar.reaction)", "assert_eq!(&reaction[..3], &scalar.residual)") })],
    ["3D one-page close", exact({ fem3dSession: mutate(fem3dSession, "*page = None;", "self.pages = std::array::from_fn(|_| None);") })],
    ["3D page maximum law", exact({ fem3dSession: mutate(fem3dSession, "fem3d_snapshot_preflight_page_maximum_plus_one_returns_exact_producer", "fem3d_snapshot_preflight_smoke") })],
    ["3D mounted atomic swap", exact({ fem3dSession: mutate(fem3dSession, "self.displaced = self.current.replace(lease)", "self.current = Some(lease)") })],
    ["3D pre-work fuel", exact({ fem3dSession: mutate(fem3dSession, "        cx.consume_fuel(1);\n        let step = self.candidate.as_mut().map(|candidate| candidate.step_one(snapshot, solver, &mut self.backing, freshness));", "        let step = self.candidate.as_mut().map(|candidate| candidate.step_one(snapshot, solver, &mut self.backing, freshness));") })],
    ["editor model lease handoff", exact({ fem3dModel: mutate(fem3dModel, "scene.snapshot = visual.map(", "let snapshot = visual.map(") })],
    ["editor results lease handoff", exact({ fem3dResults: mutate(fem3dResults, "scene.snapshot = visual.map(", "let snapshot = visual.map(") })],
    ["viewer mounted authority", exact({ fem3dViewer: mutate(fem3dViewer, "crate::artifacts::fem3d::live_visual::with_live_visual(doc.render_operation(), model::render)", "model::render(None)") })],
    ["viewer lease handoff", exact({ fem3dViewerModel: mutate(fem3dViewerModel, "scene.snapshot = visual.map(", "let snapshot = visual.map(") })],
    ["viewer mounted hook forwarding", exact({ frameworkPlugin: mutate(frameworkPlugin, "V::mounted_job_prepare_snapshot_read(operation, snapshot)", "false") })],
    ["viewer pending effect forwarding", exact({ frameworkPlugin: mutate(frameworkPlugin, "V::pending_effects(doc, cfg)", "Vec::new()") })],
    ["prepared instance admission", exact({ frameworkWorld: mutate(frameworkWorld, "world3d_draw_rebuild_admit_instance(state, 0, id, model, color, false, false)", "Ok(())") })],
    ["prepared atomic seal", exact({ frameworkWorld: mutate(frameworkWorld, "world3d_draw_rebuild_seal(state)", "Ok(())") })],
    ["prepared lease swap", exact({ frameworkWorld: mutate(frameworkWorld, "state.snapshot_lease = Some(cursor.lease)", "state.snapshot_lease = None") })],
  ];
  for (const [name, accepted] of mutations) if (accepted) throw new Error("[verify interactivity tool-jobs p6i] mutation " + name + " was falsely accepted.");
  return hardenedCount + mutations.length;
}
