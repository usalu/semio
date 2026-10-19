import { toolJobRustBlock } from "../../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { interactivityProductionSource } from "../../🗄️database/🟦️.ts";

/** 🧭️ Extracts one uniquely declared Rust item through the shared literal/comment-aware block scanner. */
export function interactivityCheckedRustBlockScope(source: string, marker: string, name: string, failures: string[]): string | undefined {
  const escaped = marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const declarations = [...source.matchAll(new RegExp(`^[\\t ]*${escaped}`, "gmu"))];
  const stale = `P5a policy boundary is stale: ${name}`;
  if (declarations.length !== 1) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  const start = source.indexOf(marker, declarations[0]!.index);
  const headerEnd = start + marker.length;
  const open = source.indexOf("{", headerEnd);
  const semicolon = source.indexOf(";", headerEnd);
  const block = open < 0 || (semicolon >= 0 && semicolon < open) ? undefined : toolJobRustBlock(source, open);
  if (!block) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  return policyStripRustCommentsAndStrings(source.slice(start, block.end));
}

export function interactivityMountedFrameTransactionFailures(
  glueSource: string,
  frameJobSource: string,
  hostSource: string,
  snapshotSource: string,
  uiRuntimeGlueSource: string,
  shellSource: string,
  engineCanvasSource: string,
  world3dSource: string,
  preparedSource: string,
  gpuSource: string,
  drawSource: string,
  interpreterSource: string,
  uiEngineSource: string,
  paintSource: string,
  sceneSlotsSource: string,
  scenesSource: string,
  servicesSource: string,
): string[] {
  const glue = interactivityProductionSource(glueSource);
  const frameJob = interactivityProductionSource(frameJobSource);
  const host = interactivityProductionSource(hostSource);
  const snapshot = interactivityProductionSource(snapshotSource);
  const uiRuntimeGlue = interactivityProductionSource(uiRuntimeGlueSource);
  const shell = interactivityProductionSource(shellSource);
  const engineCanvas = interactivityProductionSource(engineCanvasSource);
  const world3d = interactivityProductionSource(world3dSource);
  const prepared = interactivityProductionSource(preparedSource);
  const gpu = interactivityProductionSource(gpuSource);
  const draw = interactivityProductionSource(drawSource);
  const interpreter = interactivityProductionSource(interpreterSource);
  const uiEngine = interactivityProductionSource(uiEngineSource);
  const paint = interactivityProductionSource(paintSource);
  const sceneSlots = interactivityProductionSource(sceneSlotsSource);
  const scenes = interactivityProductionSource(scenesSource);
  const services = interactivityProductionSource(servicesSource);
  const failures: string[] = [];
  const scope = (source: string, marker: string, name: string) => interactivityCheckedRustBlockScope(source, marker, name, failures);
  const scopes = (...parts: (string | undefined)[]) => parts.every((part) => part !== undefined) ? parts.join("\n") : undefined;
  const transaction = glue.slice(glue.indexOf("pub(crate) struct FrameTransaction"), glue.indexOf("pub(crate) struct AppFramePresentation"));
  const frameCursors = glue.slice(glue.indexOf("struct FrameBuildCursor"), glue.indexOf("struct FrameWheelCursor"));
  const buildBoundary = glue.slice(glue.indexOf("fn frame_before_input_step"), glue.indexOf("fn frame_after_input_step"));
  const finishBoundary = glue.slice(glue.indexOf("fn frame_after_input_step"), glue.indexOf("//#region 🔖️OsHostDecomposition — SemioApp deletion"));
  const deferredBoundary = glue.slice(glue.indexOf("fn start_frame_deferred"), glue.indexOf("fn apply_step", glue.indexOf("fn start_frame_deferred")));
  const maintenanceOwnerBoundary = glue.slice(glue.indexOf("struct FrameDeferredCursor"), glue.indexOf("enum RuntimeApply"));
  const maintenanceNativeOwnerBoundary = glueSource.slice(glueSource.indexOf("struct FrameDeferredCursor"), glueSource.indexOf("enum RuntimeApply"));
  const maintenanceCloseBoundary = scope(maintenanceNativeOwnerBoundary, "fn begin_close(&mut self)", "native maintenance close");
  const maintenanceExecutionBoundary = glue.slice(glue.indexOf("struct FrameMaintenanceExecutionRegistry"), glue.indexOf("impl<R: MailboxRuntime> RuntimeMailbox<R> {"));
  const maintenanceRegistryBoundary = glue.slice(glue.indexOf("struct FrameMaintenanceExecutionRegistry"), glue.indexOf("struct FrameMaintenanceAuthority"));
  const maintenanceSpawnBoundary = glue.slice(glue.indexOf("fn try_spawn_frame_maintenance_reserved"), glue.indexOf("#[cfg(target_arch = \"wasm32\")]", glue.indexOf("fn try_spawn_frame_maintenance_reserved")));
  const maintenanceTerminalBoundary = glue.slice(glue.indexOf("fn frame_maintenance_terminal_fault"), glue.indexOf("enum RuntimeApply"));
  const maintenanceWorkerTerminalStart = maintenanceSpawnBoundary.indexOf("if let Some(fault) = frame_maintenance_terminal_fault");
  const maintenanceWorkerTerminalBoundary = maintenanceSpawnBoundary.slice(maintenanceWorkerTerminalStart, maintenanceSpawnBoundary.indexOf("            } else {", maintenanceWorkerTerminalStart));
  const chromeBoundary = scopes(
    scope(shell, "pub(crate) struct ShellChromeFrameCursor", "Shell chrome cursor"),
    scope(shell, "enum ShellChromeFramePhase", "Shell chrome phase"),
    scope(shell, "pub(crate) fn render_chrome_step", "Shell chrome step"),
  );
  const engineBoundary = engineCanvas.slice(engineCanvas.indexOf("const ENGINE_CANVAS_FRAME_PACKET_CAPACITY"), engineCanvas.indexOf("struct EngineGpuSurface"));
  const worldBoundary = world3d.slice(world3d.indexOf("const WORLD3D_FRAME_RESOURCE_CAPACITY"), world3d.indexOf("//#endregion 📦️PreparedWorldResources"));
  const atlasBoundary = prepared.slice(prepared.indexOf("pub const PREPARED_ATLAS_PAGE_BYTES"), prepared.indexOf("impl PreparedRasterPages"));
  const atlasGpuBoundary = gpu.slice(gpu.indexOf("struct PreparedAtlasUploadCursor"), gpu.indexOf("pub fn finish_prepared"));
  const atlasDrawBoundary = draw.slice(draw.indexOf("upload_glyph_atlas_page"), draw.indexOf("pub fn render_frame"));
  const chromeMainBoundary = scope(shell, "fn render_main_window_step", "Shell main window");
  const chromePanelBoundary = scope(shell, "fn render_panel_step", "Shell panel");
  const chromeNavbarBoundary = scope(shell, "fn render_navbar_step", "Shell navbar");
  const chromeTutorialBoundary = scope(shell, "fn render_tutorial_bar_step", "Shell tutorial");
  const chromeFooterBoundary = scope(shell, "fn render_footer_step", "Shell footer");
  const chromeOverlayStepBoundary = scope(shell, "fn render_overlay_step", "Shell overlay");
  const chromeContextMenuBoundary = scope(shell, "fn render_context_menu_step", "Shell context menu");
  const chromeTooltipBoundary = scope(shell, "fn render_chrome_tooltip_step", "Shell tooltip");
  const chromeDialogBoundary = scope(shell, "fn render_chrome_dialog_step", "Shell dialog");
  const chromeTourBoundary = scope(shell, "fn render_chrome_tour_step", "Shell tour");
  const chromeOverlayBoundary = scopes(chromeOverlayStepBoundary, chromeContextMenuBoundary, chromeTooltipBoundary, chromeDialogBoundary, chromeTourBoundary);
  const shellChildren = scopes(chromeMainBoundary, chromePanelBoundary, chromeNavbarBoundary, chromeTutorialBoundary, chromeFooterBoundary, chromeOverlayBoundary);
  const documentBoundary = interactivityProductionSource(interpreterSource.slice(interpreterSource.indexOf("pub struct UiDocumentFrameCursor"), interpreterSource.indexOf("pub fn render_ui_document(")));
  const uiFrameBoundary = uiEngine.slice(uiEngine.indexOf("const RETAINED_PAINT_DEPTH_CREDITS"), uiEngine.indexOf("pub fn frame<H"));
  const paintNodeBoundary = scopes(
    scope(paint, "pub struct RetainedGlyphCursor", "paint glyph cursor"),
    scope(paint, "pub(crate) struct RetainedNodePaintCursor", "paint node cursor"),
    scope(paint, "impl RetainedNodePaintCursor", "paint node cursor implementation"),
    scope(paint, "fn paint_retained_glyph_step_inner", "paint retained glyph step"),
    scope(paint, "fn retained_fixed_output_budgeted", "paint output credit"),
    scope(paint, "fn retained_text_node_step_weighted", "paint retained text"),
    scope(paint, "fn retained_control_is_bounded", "paint bounded control"),
    scope(paint, "fn retained_tree_node_step", "paint retained tree"),
    scope(paint, "pub(crate) fn paint_node_step(", "paint node step"),
    scope(paint, "pub(crate) fn paint_node_step_with_driver", "paint node driver step"),
    ...["text", "stack", "separator", "button", "input", "select", "toggle", "key_value", "slider", "number_stepper", "ring", "progress", "icon_select", "field", "section", "group", "image", "component_scene", "external_slot"].map((kind) => scope(paint, `fn paint_${kind}_node_step(`, `paint ${kind} node step`)),
  );
  const interactiveSyncBoundary = scopes(
    scope(paint, "struct RetainedSyncTreeFrame", "interactive sync tree frame"),
    scope(paint, "struct RetainedSyncTreeRecord", "interactive sync tree record"),
    scope(paint, "pub(crate) struct RetainedInteractiveSyncCursor", "interactive sync cursor"),
    scope(paint, "impl RetainedInteractiveSyncCursor", "interactive sync cursor implementation"),
    scope(paint, "pub(crate) fn sync_interactive_state_node_step", "interactive sync step"),
  );
  const sceneNodeBoundary = scopes(
    scope(sceneSlots, "pub struct ScenePaintCursor", "scene paint cursor"),
    scope(sceneSlots, "impl ScenePaintCursor", "scene paint cursor implementation"),
    scope(sceneSlots, "pub trait SceneHost", "scene host contract"),
    scope(sceneSlots, "pub(crate) fn scene_slot_for_node", "scene slot node resolver"),
  );
  const sceneHostBoundary = interpreter.slice(interpreter.indexOf("impl ui_wgpu::wgpu::SceneHost for FrameworkSceneHost"), interpreter.indexOf("fn drive_mounted_layout_text_one"));
  const retainedImageBoundary = interpreter.slice(interpreter.indexOf("fn render_ui_image_step"), interpreter.indexOf("fn render_ui_image("));
  const retainedSceneBoundary = scenes.slice(scenes.indexOf("pub fn render_component_scene_step"), scenes.indexOf("pub fn render_component_scene("));
  const findBoundary = shell.slice(shell.indexOf("pub struct ShellFindItems"));
  const maintenanceBoundary = shell.slice(shell.indexOf("struct ShellChromeMaintenance"), shell.indexOf("fn body_rect"));
  const chromeTextBoundary = scopes(
    scope(shell, "fn chrome_text_step", "Shell chrome text step"),
    scope(shell, "fn chrome_text_complete_step", "Shell chrome text completion"),
  );
  const chromeGroupWidthBoundary = scope(shell, "fn retained_chrome_group_item_width", "Shell chrome group width");
  const chromeGroupStepBoundary = scopes(
    scope(shell, "enum RetainedChromeGroupStep", "Shell chrome group phase"),
    scope(shell, "fn render_retained_chrome_group_item_step(", "Shell chrome group step"),
    scope(shell, "fn render_retained_chrome_group_item_step_with_trailing", "Shell chrome group trailing step"),
  );
  const chromeGroupBoundary = scopes(chromeGroupWidthBoundary, chromeGroupStepBoundary);
  const prefsBoundary = shell.slice(shell.indexOf("fn native_pref_field_path"), shell.indexOf("//#endregion 🗄️PrefsStore"));
  const nativePrefsBoundary = prefsBoundary.slice(0, prefsBoundary.indexOf("fn prefs_get("));
  const fixedFilePageBoundary = services.slice(services.indexOf("//#region 📄️FixedFilePage"), services.indexOf("//#endregion 📄️FixedFilePage"));
  const requireAll = (source: string | undefined, needles: readonly string[], label: string) => {
    if (source === undefined) return;
    if (needles.some((needle) => !source.includes(needle))) failures.push(label);
  };
  const requireExactlyOne = (source: string | undefined, needle: string, label: string) => {
    if (source === undefined) return;
    if (source.split(needle).length - 1 !== 1) failures.push(label);
  };
  requireAll(transaction, [
    "DrainProjectionDeltas",
    "RouteIntents",
    "FlushEffects",
    "PresentSurface",
    "ReconcileTree",
    "BuildRenderPackets",
    "PublishSnapshot",
    "context.set_stage(self.stage_label())",
  ], "P5a exact seven-stage mounted transaction is incomplete");
  requireAll(glue, [
    "struct FrameActionOwners",
    "slots: [Option<ActionDescriptor>; WORLD3D_DEADLINE_CAPACITY]",
    "fn try_push(&mut self, action: ActionDescriptor) -> Result<(), ActionDescriptor>",
    "fn pop_front(&mut self) -> Option<ActionDescriptor>",
    "EFFECT_STORM_BUDGET: u32 = 64",
  ], "P5a action/effect admission is not fixed, fallible, and identity preserving");
  requireAll(glueSource, ["frame_action_max_plus_one_returns_the_exact_owner", "assert_eq!(rejected.controller_id.as_ptr(), identity)"], "P5a exact MAX + 1 action owner law is missing");
  requireAll(transaction, [
    "operation: semio_framework_trace::OperationId",
    "generation: semio_framework_trace::Generation",
    "base_witness: Option<RuntimePresentationWitness>",
    "context.operation() != self.operation",
    "context.generation() != self.generation",
    "context.is_cancelled()",
    "context.deadline_exceeded()",
    "base_witness != current_witness",
    "checked_add(1)",
  ], "P5a opportunity freshness, revision, generation, cancellation, deadline, or effect guards are incomplete");
  for (const forbidden of ["VecDeque", "std::vec::IntoIter", "wrapping_add", "saturating_add", ".clear()", ".expect(", ".unwrap(", "loop {", "while "]) if (transaction.includes(forbidden)) failures.push(`P5a mounted transaction contains forbidden ${forbidden}`);
  requireAll(frameCursors, [
    "struct FrameBuildCursor",
    "previous_draw: Option<DrawList>",
    "previous_overlay: Option<DrawList>",
    "struct FrameFinishCursor",
    "icon_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>",
    "glyph_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>",
  ], "P5a mounted build/finish cursors do not retain draw, overlay, and atlas page ownership");
  requireAll(glue, ["struct FrameEnginePackets", "engine_packets: FrameEnginePackets", "fn try_push(&mut self, packet: engine_canvas::EngineCanvasPacket) -> Result<(), engine_canvas::EngineCanvasPacket>"], "P5a mounted packet collector is not fixed and identity-preserving");
  if (frameCursors.includes("engine_packets: Vec<") || frameCursors.includes("Vec<engine_canvas::EngineCanvasPacket>")) failures.push("P5a mounted frame cursor retains a dynamic engine packet workset");
  requireAll(buildBoundary, [
    "FrameBuildPhase::PreparedGpuAbandonment",
    "PreparedGpuPresentCursor::close_abandoned_step()",
    "FrameBuildPhase::PreparedInputAbandonment",
    "PreparedRenderInput::next_abandoned_close_demands(4096)",
    "PreparedRenderInput::close_abandoned_step(grant)",
    "FrameBuildPhase::PreparedJobAbandonment",
    "PreparedRenderJob::next_abandoned_close_demands(4096)",
    "PreparedRenderJob::close_abandoned_step(grant)",
    "FrameBuildPhase::PreparedAbandonment",
    "PreparedRenderReceiver::next_abandoned_close_demands(4096)",
    "PreparedRenderReceiver::close_abandoned_step(grant)",
    "FrameBuildPhase::PacketAbandonment",
    "PreparedRenderPacket::next_abandoned_close_demands(4096)",
    "PreparedRenderPacket::close_abandoned_step(grant)",
    "FrameBuildPhase::RetireDraw",
    "previous.retire_step()",
    "FrameBuildPhase::RetireOverlay",
    "render_chrome_step",
    "resources.append_step(input)",
    "resources.take_packet_step()",
    "PreparedAtlasPages::try_new",
    "pages.push_page(&self.icons.pixels, pages.next_row())",
    "PreparedRenderUpload::IconAtlasPages",
  ], "P5a pre-input boundary does not advance retained draw, chrome, packet, and upload owners one step at a time");
  requireAll(finishBoundary, [
    "FrameFinishPhase::GlyphUpload",
    "pages.push_page(&self.atlas.pixels, pages.next_row())",
    "PreparedRenderUpload::GlyphAtlasPages",
    "FrameFinishPhase::Draw",
    "FrameFinishPhase::Overlay",
    "self.pending_frame_deferred = Some",
  ], "P5a post-input boundary does not page atlas work and retain draw/overlay/deferred transfers");
  for (const forbidden of ["frame_before_input(", "frame_after_input(", "drive_pending_frame_deferred", "take_packets(", "append_to(", "self.draw.clear()", "self.overlay.clear()", "self.icons.pixels.clone()", "self.atlas.pixels.clone()"])
    if (glue.includes(forbidden)) failures.push(`P5a mounted call graph retains opaque whole-work callee ${forbidden}`);
  requireAll(deferredBoundary, ["cursor_value.take_next()", "spawn_frame_deferred_reserved", "try_spawn_frame_maintenance_reserved", "pending_frame_maintenance_refusal", "FrameDeferredCursor::close_step", "runtime.return_interaction(interaction)"], "P5a deferred boundary does not take, refuse, close, and resume exactly one retained work owner");
  for (const forbidden of [".expect(", ".unwrap(", "loop {", "while "])
    if (deferredBoundary.includes(forbidden)) failures.push(`P5a deferred boundary retains panicking or run-to-completion ${forbidden}`);
  requireAll(maintenanceOwnerBoundary, [
    "generation: u64",
    "cancel: semio_framework_async::CancelToken",
    "closing: bool",
    "struct FrameMaintenanceOwner",
    "deadline_ms: Option<u64>",
    "struct FrameMaintenanceOwnerCell<T>",
    "state: std::sync::atomic::AtomicU8",
    "compare_exchange(0, 1",
    "fn try_restore(&self, owner: T) -> Result<(), T>",
    "struct FrameMaintenanceRefusal",
    "struct FrameMaintenanceAuthority",
    "fn is_live(&self, generation: u64) -> bool",
    "fn release(&self, generation: u64) -> bool",
    "self.generation.store(0, Ordering::Release)",
    "compare_exchange(1, 0",
  ], "P5a maintenance authority is not generation/cancel/deadline-qualified with nonblocking exact-owner refusal handback");
  requireAll(maintenanceNativeOwnerBoundary, ["#[cfg(any(not(target_arch = \"wasm32\"), test))]"], "P5a native maintenance close is not available in production");
  requireAll(maintenanceCloseBoundary, ["self.closing = true"], "P5a native maintenance close does not retain its closing state");
  requireAll(maintenanceExecutionBoundary, [
    "struct FrameMaintenanceExecutionRegistry",
    "const QUEUED: u8 = 2",
    "const RUNNING: u8 = 3",
    "const ABANDONED: u8 = 4",
    "fn try_publish(&self, generation: u64",
    "fn try_begin(&self, generation: u64)",
    "fn abandon(&self, generation: u64) -> bool",
    "fn try_take_abandoned(&self)",
    "struct FrameMaintenanceExecutionEnvelope",
    "impl Drop for FrameMaintenanceExecutionEnvelope",
    "if self.armed && self.registry.abandon(self.generation)",
    "RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }",
    "struct FrameMaintenanceExecutionGuard",
    "impl Drop for FrameMaintenanceExecutionGuard",
    "self.owner_cell.restore_taken(owner)",
    "frame_maintenance_executions: Arc<FrameMaintenanceExecutionRegistry>",
  ], "P5a accepted maintenance execution lacks generation-qualified external Drop handback");
  for (const forbidden of ["Mutex", ".lock()", "std::mem::take", "Vec<", "loop {", "while "])
    if (maintenanceRegistryBoundary.includes(forbidden)) failures.push(`P5a maintenance execution registry retains blocking, dynamic, or recursive recovery ${forbidden}`);
  requireAll(maintenanceSpawnBoundary, [
    "fn try_spawn_frame_maintenance_reserved",
    "frame_maintenance.try_reserve(generation)",
    "renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job)",
    "error.into_job()",
    "FrameMaintenanceRefusal { owner: reclaimed, reservation_live: true }",
    "cursor.cancel.is_cancelled_now()",
    "presentation_authority.witness_for(generation).is_none()",
    "worker_pool.now_ms() > deadline",
    "frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded)",
    "cursor.begin_close()",
    "frame_maintenance.release(generation)",
    "execution.finish()",
  ], "P5a maintenance submission does not preserve exact owners across refusal/cancel/stale/deadline/close");
  requireAll(maintenanceTerminalBoundary, ["if cancelled", "else if stale", "else if deadline_exceeded", "frame maintenance cancelled", "frame maintenance generation became stale", "frame maintenance deadline expired"], "P5a maintenance terminal classifier is incomplete for cancel/stale/deadline");
  requireAll(maintenanceWorkerTerminalBoundary, ["frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded)", "cursor.begin_close()", "interaction.frame_fault = Some(fault.to_string())"], "P5a maintenance terminal outcome does not enter retained close before handback");
  for (const forbidden of ["KernelPoolFuture::spawn", "let _ = renderer_worker_pool().try_submit", "renderer_worker_pool().submit(", "Lane::Interactive", ".lock()", "loop {", "while "])
    if (maintenanceSpawnBoundary.includes(forbidden)) failures.push(`P5a maintenance submission retains discarded, blocking, interactive-lane, or whole-work ${forbidden}`);
  if (maintenanceOwnerBoundary.includes("self.actions = FrameActionOwners::default()")) failures.push("P5a deferred close bulk-drops the populated action owner");
  requireAll(findBoundary, [
    "slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>",
    "payload_bytes: usize",
    "generation: u64",
    "fn try_push_at(&mut self, generation: u64, item: ShellFindItem) -> Result<(), ShellFindItem>",
    "fn pop_front(&mut self) -> Option<ShellFindItem>",
    "fn close_step(&mut self) -> bool",
    "std::cell::Cell<Option<ActiveShellFindItems>>",
    "std::ptr::NonNull<ShellFindItems>",
    "pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem>",
    "collector.try_push_at(active.generation, item)",
  ], "P5a find producer is not fixed, generation-qualified, nonblocking, and exact-owner fallible");
  for (const forbidden of ["Arc<Mutex<Vec<ShellFindItem>>>", "Vec<ShellFindItem>", ".lock()", "std::mem::take", "take_find_items", "ACTIVE_FIND_ITEM_SINKS"])
    if (findBoundary.includes(forbidden)) failures.push(`P5a find producer retains blocking, dynamic, or whole-owner ${forbidden}`);
  requireAll(shellSource, ["find_item_max_plus_one_returns_the_exact_owned_item", "stale_find_generation_returns_the_exact_owned_item", "assert_eq!(rejected.id.as_ptr(), identity)"], "P5a find MAX + 1, stale-generation, close, or exact-owner law is missing");
  requireAll(maintenanceBoundary, [
    "load_requested: bool",
    "load_phase: u8",
    "introduction_read: Option<String>",
    "introduction_write: Option<String>",
    "persist_phase: u8",
    "request_chrome_preferences_load",
    "request_introduction_read",
    "request_panel_layout_persist",
    "request_presence_preview",
    "request_chrome_preferences_persist",
    "pub(crate) fn advance_chrome_maintenance_step",
    "read_ui_preferences()",
    "stored_field_get(",
    "stored_field_set(",
    "persist_ui_preferences(",
  ], "P5a Shell persistence/presence work is not a retained one-field/page child");
  requireAll(prefsBoundary, [
    "fn native_pref_field_path",
    "key.len() > SHELL_CHROME_IO_FIELD_BYTES",
    "fn native_pref_read_page",
    "semio_framework_os_services::storage_worker_read_fixed_file_document(&path, OS_SHELL_CONFIG_MAX_BYTES)",
    "semio_framework_os_services::storage_worker_read_fixed_file_page(&path, SHELL_CHROME_IO_FIELD_BYTES)",
    "String::from_utf8(page)",
    "fn native_pref_write_page",
    "semio_framework_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), SHELL_CHROME_IO_FIELD_BYTES)",
    "fn prefs_get(",
    "fn prefs_set(",
  ], "P5a native preference maintenance does not delegate document and field pages to the host-storage service");
  requireAll(fixedFilePageBoundary, [
    "pub const STORAGE_FIXED_FILE_PAGE_BYTES: usize = 16 * 1024",
    "pub fn storage_worker_read_fixed_file_page",
    "maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES",
    "logical_bytes > maximum_bytes",
    "let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES]",
    "file.read_exact(&mut page[..logical_bytes])",
    "if file.read(&mut trailing)? != 0",
    "pub fn storage_worker_write_fixed_file_page",
    "bytes.len() > maximum_bytes",
    "std::fs::create_dir_all(parent)?",
    "file.write_all(bytes)",
  ], "P5a host-storage service does not own exact 16 KiB page admission/read/write and growth rejection");
  if (!servicesSource.includes("fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page")) failures.push("P5a host-storage fixed page exact/max-plus-one/oracle preservation law is missing");
  for (const forbidden of ["PREFS_STORE", "Mutex", ".lock()", "serde_json::from_", "serde_json::to_", "OS_SHELL_CONFIG_STORAGE_KEY", "Vec::with_capacity", "vec![", "loop {", "while ", "std::fs::", "std::io::"])
    if (nativePrefsBoundary.includes(forbidden)) failures.push(`P5a native preference field page retains blocking, dynamic, or whole-config ${forbidden}`);
  for (const forbidden of ["load_ui_prefs_once()", "read_stored_introduction_seen(", "persist_panel_layout_if_changed()", "write_stored_introduction_seen(", "persist_ui_prefs_if_changed()", "publish_presence_heartbeat()"])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a chrome frame still reaches synchronous maintenance ${forbidden}`);
  requireAll(glue, ["FrameDeferredWork::ShellMaintenance", "shell_maintenance: bool", "semio_framework_async::Lane::Io", "PreparedAtlasPages::next_abandoned_close_demands(4096)", "PreparedAtlasPages::close_abandoned_step(grant)"], "P5a Shell I/O or abandoned atlas close is not mounted on the shared retained boundary");
  requireAll(chromeBoundary, [
    "ShellChromeFramePhase::MainWindow",
    "ShellChromeFramePhase::PaneOverlayHits",
    "ShellChromeFramePhase::Panels",
    "ShellChromeFramePhase::AgentChatHeader",
    "ShellChromeFramePhase::Navbar",
    "ShellChromeFramePhase::TutorialBar",
    "ShellChromeFramePhase::Footer",
    "ShellChromeFramePhase::Overlay",
    "ShellChromeFramePhase::IntroductionWrite",
    "introduction_seen_writes.pop()",
  ], "P5a shell chrome boundary is not a retained one-child cursor");
  requireAll(chromeBoundary, [
    "render_main_window_step(&mut cursor.child",
    "render_mobile_panel_step(&mut cursor.child",
    "render_panel_step(&mut cursor.child, anchor",
    "render_agent_chat_header_step(&mut cursor.child",
    "render_navbar_step(&mut cursor.child",
    "render_tutorial_bar_step(&mut cursor.child",
    "render_footer_step(&mut cursor.child",
    "render_overlay_step(&mut cursor.child",
    "cursor.advance(ShellChromeFramePhase::PaneOverlayHits)",
  ], "P5a shell chrome phases do not remain parked on retained child cursors");
  for (const forbidden of ["self.render_main_window(", "self.render_left_panel(", "self.render_right_panel(", "self.render_navbar(", "self.render_tutorial_bar(", "self.render_footer(", "self.render_overlay(", "self.render_tree_drag_overlay(", "render_tutorial_gesture_overlay("])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a shell chrome cursor reaches whole child ${forbidden}`);
  for (const forbidden of ["render_chrome(", "render_chrome_build(", ".clear()", "std::mem::take(&mut self.chrome_build.introduction_seen_writes)", "for app_id in", "while "])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a shell chrome cursor retains whole-work ${forbidden}`);
  requireAll(shellChildren, [
    "fn render_main_window_step",
    "render_ui_document_step(",
    "fn render_panel_step",
    "fn render_navbar_step",
    "fn render_tutorial_bar_step",
    "fn render_footer_step",
    "fn render_overlay_step",
    "fn render_context_menu_step",
    "fn render_chrome_tooltip_step",
    "fn render_chrome_dialog_step",
    "fn render_chrome_tour_step",
  ], "P5a live Shell children are not retained node/widget cursors");
  for (const forbidden of ["render_main_window(", "render_left_panel(", "render_right_panel(", "render_navbar(", "render_tutorial_bar(", "render_footer(", "render_overlay(", "render_ui_document(", "render_context_menu(", "render_chrome_tour(", "measure_chrome_group_item(", "render_chrome_group("])
    if (shellChildren?.includes(forbidden)) failures.push(`P5a live Shell child body reaches whole subtree ${forbidden}`);
  requireAll(chromeTextBoundary, [
    "fn chrome_text_step",
    "paint_retained_glyph_step",
    "fn chrome_text_complete_step",
    "RetainedGlyphStep::Pending",
    "RetainedGlyphStep::Complete",
    "RetainedGlyphStep::Fault",
    "cursor.reset()",
  ], "P5a Shell text callee is not a retained per-scalar/glyph authority");
  requireAll(chromeGroupBoundary, [
    "enum RetainedChromeGroupStep",
    "fn retained_chrome_group_item_width",
    "label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES",
    "fn render_retained_chrome_group_item_step",
    "group_phase: &mut u8",
    "glyph: &mut RetainedGlyphCursor",
    "chrome_text_complete_step",
    "RetainedChromeGroupStep::Pending",
    "RetainedChromeGroupStep::Complete",
    "RetainedChromeGroupStep::Fault",
  ], "P5a Shell chrome-group callee is not a fixed retained output/glyph authority");
  requireAll(shellSource, [
    "#[cfg(test)]\nfn measure_chrome_group_item",
    "#[cfg(test)]\nfn render_chrome_group",
  ], "P5a legacy whole-string chrome-group oracles are not explicitly test-only");
  for (const forbidden of ["fn measure_chrome_group_item", "fn render_chrome_group"])
    if (shell.includes(forbidden)) failures.push(`P5a production Shell retains legacy whole-string chrome-group reachability ${forbidden}`);
  for (const forbidden of ["draw_text(", "chrome_text(", "measure_text(", "for ch in", "for item in", "loop {", "while "])
    if (chromeGroupStepBoundary?.includes(forbidden)) failures.push(`P5a retained Shell chrome group reaches whole work ${forbidden}`);
  requireAll(chromeDialogBoundary, ["chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false", "cursor.scalar += 1"], "P5a live dialog does not park on the retained glyph cursor");
  requireAll(chromeTourBoundary, ["chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false", "cursor.scalar += 1"], "P5a live tour does not park on the retained glyph cursor");
  requireAll(chromeTutorialBoundary, ["fn render_tutorial_bar_step", "chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false"], "P5a mounted tutorial bar does not park dynamic text on the retained glyph cursor");
  requireAll(chromeTutorialBoundary, ["render_retained_chrome_group_item_step", "retained_chrome_group_item_width", "RetainedChromeGroupStep::Pending => return false"], "P5a mounted tutorial groups do not park each output and glyph opportunity");
  requireAll(chromeFooterBoundary, ["render_retained_panel_chrome_item_step", "render_footer_status_step", "Ok(false) => return false"], "P5a mounted footer does not park on retained dynamic utility and sync labels");
  requireAll(chromeOverlayBoundary, ["fn render_overlay_step", "fn render_context_menu_step", "fn render_chrome_tooltip_step", "fn render_chrome_dialog_step", "fn render_chrome_tour_step", "chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false"], "P5a mounted overlay children do not park dynamic text on the retained glyph cursor");
  requireAll(chromeBoundary, ["ShellChromeFramePhase::Error", "chrome_text_complete_step(draw, atlas, error", "&mut cursor.child.glyph", "Ok(false) => return false"], "P5a mounted Shell error path does not park on the retained glyph cursor");
  for (const [boundary, label] of [[chromeTextBoundary, "text callee"], [chromeDialogBoundary, "dialog"], [chromeTourBoundary, "tour"]] as const)
    for (const forbidden of ["draw_text(", "chrome_text(", "for ch in", "for character in", ".chars().for_each", ".chars().count()", "loop {", "while "])
      if (boundary?.includes(forbidden)) failures.push(`P5a Shell ${label} retains whole-string work ${forbidden}`);
  for (const [boundary, label] of [[chromeNavbarBoundary, "navbar"], [chromeTutorialBoundary, "tutorial"], [chromeFooterBoundary, "footer"], [chromeOverlayBoundary, "overlay"]] as const)
    for (const forbidden of ["draw_text(", "chrome_text(", ".chars().count()", "for ch in", "for character in", "loop {", "while "])
      if (boundary?.includes(forbidden)) failures.push(`P5a mounted Shell ${label} reaches whole-string work ${forbidden}`);
  requireAll(shellSource, ["dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant", "dialog_and_tour_text_max_plus_one_fails_without_output", "dynamic_chrome_group_retains_every_glyph_and_border_opportunity", "dynamic_chrome_group_max_plus_one_is_identity_preserving"], "P5a Shell multi-megabyte one-scalar, chrome-group, or MAX + 1 law is missing");
  requireAll(documentBoundary, [
    "pub struct UiDocumentFrameCursor",
    "UiDocumentFramePhase::Ingress",
    "UiDocumentFramePhase::Layout",
    "UiDocumentFramePhase::Paint",
    "engine.apply_document_page",
    "drive_mounted_layout_text_one",
    "engine.frame_into_step",
  ], "P5a document consumer does not retain ingress/layout/node progress");
  for (const forbidden of ["engine.frame(", "composite_retained_draw_list(", "dispatch_pointer_events("])
    if (documentBoundary.includes(forbidden)) failures.push(`P5a live document consumer reaches whole work ${forbidden}`);
  requireAll(uiFrameBoundary, [
    "RETAINED_PAINT_DEPTH_CREDITS: usize = 64",
    "visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS]",
    "enum RetainedPaintWalkStep",
    "pub fn frame_into_step",
    "sync_interactive_state_node_step",
    "sync_node: Option<",
    "node_sync: RetainedInteractiveSyncCursor",
    "frame.node_sync.close_step()",
    "RetainedInteractiveSyncStep::Pending",
    "RetainedInteractiveSyncStep::Complete",
    "RetainedInteractiveSyncStep::Fault",
    "paint_node_step",
    "paint_node: Option<(",
    "node_paint: RetainedNodePaintCursor",
    "scene_node: Option<(",
    "scene_paint: ScenePaintCursor",
    "host.paint_slot_step",
    "scene_slot_for_node",
    "RetainedPaintPhase::Publish",
  ], "P5a retained UI frame does not advance one fixed visit/node/scene/publication unit");
  for (const forbidden of ["paint_tree(", "sync_interactive_state_node(&mut", "collect_scene_slots(", "window.draw.clear()", "for slot in"])
    if (uiFrameBoundary.includes(forbidden)) failures.push(`P5a live UI frame reaches whole subtree ${forbidden}`);
  requireAll(paint, [
    "RETAINED_SYNC_COLLECTION_ITEMS: usize = 256",
    "RETAINED_SYNC_OUTPUTS: usize = RETAINED_SYNC_COLLECTION_ITEMS * 2",
    "RETAINED_SYNC_DEPTH: usize = 64",
    "RETAINED_SYNC_KEY_BYTES: usize = 256",
  ], "P5a mounted interactive synchronization fixed limits are missing");
  requireAll(interactiveSyncBoundary, [
    "struct RetainedSyncTreeFrame",
    "items_pointer: usize",
    "items_len: usize",
    "tree_frames: [Option<RetainedSyncTreeFrame>; RETAINED_SYNC_DEPTH]",
    "tree_records: [Option<RetainedSyncTreeRecord>; RETAINED_SYNC_OUTPUTS]",
    "pub(crate) fn sync_interactive_state_node_step",
    "RetainedInteractiveSyncPhase::SelectItem",
    "RetainedInteractiveSyncPhase::SelectScan",
    "RetainedInteractiveSyncPhase::SelectWrite",
    "RetainedInteractiveSyncPhase::TreeItem",
    "RetainedInteractiveSyncPhase::TreeApplyScan",
    "RetainedInteractiveSyncPhase::TreeApplyWrite",
    "RetainedInteractiveSyncPhase::TreeClose",
    "pub(crate) fn close_step(&mut self) -> bool",
  ], "P5a mounted interactive synchronization lacks fixed retained item/depth/output ownership");
  for (const forbidden of ["Vec<", ".clone()", ".collect::<Vec", "tree.children(id)", "sync_tree_row_layout", "sync_tree_item_layout", "for index in", "for child in", "while ", "loop {"])
    if (interactiveSyncBoundary?.includes(forbidden)) failures.push(`P5a mounted interactive synchronization retains whole collection/depth work ${forbidden}`);
  requireAll(paint, ["RETAINED_NODE_TEXT_MAX_BYTES: usize = 4 * 1024 * 1024"], "P5a retained node text bound is missing");
  requireAll(paintNodeBoundary, [
    "pub(crate) struct RetainedNodePaintCursor",
    "glyph: RetainedGlyphCursor",
    "phase: u16",
    "item: usize",
    "path: [usize; RETAINED_TREE_DEPTH]",
    "byte: usize",
    "line: usize",
    "pen_x: f32",
    "pub(crate) fn paint_node_step",
    "draw.begin_retained_output(strikes, strikes * size_of::<crate::wgpu::draw::UiInstance>())",
    "draw.finish_retained_output()",
    "value[cursor.byte..].chars().next()",
    "cursor.byte = next_byte",
    "atlas.ensure_glyph(ch, size)",
    "cursor.line.checked_add(1)",
    "match &node.spec.0",
    "UiNode::Select(select)",
    "UiNode::KeyValue(key_value)",
    "UiNode::Tree(tree_node) => retained_tree_node_step",
    "fn retained_tree_node_step",
    "retained_control_is_bounded",
  ], "P5a paint child authority is not retained by byte, glyph, line, output credit, and close state");
  for (const forbidden of ["wrap_text(", ".collect::<Vec", "for (index, line)", "for ch in", "for character in", "paint_stack(tree", "paint_node(tree", "paint_node_self(", "paint_control(", "for child in", "loop {", "while "])
    if (paintNodeBoundary?.includes(forbidden)) failures.push(`P5a paint node recursively reaches child work ${forbidden}`);
  requireAll(paintSource, [
    "retained_text_paint_emits_at_most_one_glyph_per_grant",
    "retained_multi_megabyte_input_advances_one_scalar_per_grant",
    "retained_select_max_plus_one_refuses_before_output_without_moving_tree_owner",
    "retained_select_sync_writes_at_most_one_row_per_grant",
    "retained_select_sync_max_plus_one_fault_closes_exact_cursor_owner",
    "retained_tree_sync_abandonment_releases_one_record_or_depth_owner_per_grant",
    "retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity",
    "retained_text_cancel_close_preserves_exact_terminal_witness",
  ], "P5a retained node text/non-text one-scalar, collection MAX + 1, identity, or close law is missing");
  requireAll(glueSource, [
    "frame_deferred_cancel_token_closes_one_exact_owner_per_grant",
    "frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation",
    "frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking",
    "frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant",
    "accepted_frame_maintenance_queue_drop_publishes_exact_generation_owner",
    "interrupted_frame_maintenance_execution_restores_before_incremental_recovery",
  ], "P5a maintenance cancel/ABA/refusal exact-owner laws are missing");
  requireAll(sceneNodeBoundary, [
    "pub struct ScenePaintCursor",
    "node: Option<NodeId>",
    "phase: u16",
    "item: usize",
    "page: usize",
    "byte: usize",
    "pub fn advance_byte(&mut self) -> Result<(), ScenePaintCursorError>",
    "fn paint_slot_step",
    "pub(crate) fn scene_slot_for_node",
    "UiNode::ComponentScene",
    "UiNode::Image",
  ], "P5a scene child authority is not one-node and producer-cursor addressable");
  if (sceneNodeBoundary?.includes("collect_scene_slots(")) failures.push("P5a one-node scene authority reaches whole scene collection");
  requireAll(sceneSlotsSource, ["scene_paint_cursor_rejects_stale_node_without_consuming_owner", "scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner"], "P5a scene stale-owner, scalar, or close law is missing");
  requireAll(sceneHostBoundary, ["fn paint_slot_step", "cursor.bind(slot.node)", "render_component_scene_step", "render_ui_image_step"], "P5a Framework scene host does not resume exact retained scene/image consumers");
  for (const forbidden of ["render_component_scene(", "render_ui_image(", "fn paint_slot("])
    if (sceneHostBoundary.includes(forbidden)) failures.push(`P5a Framework scene host reaches complete leaf renderer ${forbidden}`);
  requireAll(retainedSceneBoundary, ["cursor.phase()", "cursor.byte()", "cursor.advance_byte()", "try_reserve_retained_items(1)", "cursor.advance_item()", "cursor.finish()"], "P5a component scene consumer is not retained by scalar/item phase");
  requireAll(retainedImageBoundary, ["cursor.phase()", "cursor.byte()", "cursor.advance_byte()", "queue_ui_image_url_fetch", "try_reserve_retained_items(1)", "cursor.advance_item()", "cursor.finish()"], "P5a image consumer is not retained by scalar, asset request, and output item phase");
  if ((retainedSceneBoundary.match(/cursor\.advance_byte\(\)/g)?.length ?? 0) !== 2 || (retainedImageBoundary.match(/cursor\.advance_byte\(\)/g)?.length ?? 0) !== 2)
    failures.push("P5a component scene and image consumers must each retain both scalar byte phases");
  for (const forbidden of ["resolve_ui_image(", "resolve_ui_image_svg(", "queue_canvas_image_upload_sized("])
    if (retainedImageBoundary.includes(forbidden)) failures.push(`P5a retained image consumer reaches complete decode ${forbidden}`);
  requireAll(atlasBoundary, [
    "PREPARED_ATLAS_PAGE_BYTES: usize = 16 * 1024",
    "PREPARED_ATLAS_PAGE_CAPACITY: usize = 2_048",
    "PREPARED_ATLAS_PROCESS_ITEMS: usize = 64",
    "PREPARED_ATLAS_PROCESS_PAGES: usize = 4_096",
    "PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES: usize = 64 * 1024 * 1024",
    "PREPARED_ATLAS_PROCESS_BACKING_BYTES: usize = 96 * 1024 * 1024",
    "static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64",
    "prepared_atlas_field(current, PREPARED_ATLAS_ITEM_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_PAGE_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_PAYLOAD_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_BACKING_SHIFT",
    "struct PreparedAtlasPermit",
    "items: usize",
    "pages: usize",
    "payload_bytes: usize",
    "backing_bytes: usize",
    "PREPARED_ATLAS_PROCESS_PERMITS.compare_exchange(current, next",
    "slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>",
    "let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)",
    "Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])",
    "Box::new([0; PREPARED_ATLAS_PAGE_BYTES])",
    "pub fn close_step(&mut self) -> bool",
    "slots[index] = None",
    "permit.release_step()",
    "pub fn close_abandoned_step() -> bool",
    "impl Drop for PreparedAtlasPages",
    "PreparedAtlasAbandonment",
  ], "P5a atlas authority lacks nonblocking exact item/page/payload/backing permits or retained abandonment close");
  const atlasCredit = atlasBoundary.indexOf("let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)");
  const atlasSlots = atlasBoundary.indexOf("Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])");
  const atlasPage = atlasBoundary.indexOf("Box::new([0; PREPARED_ATLAS_PAGE_BYTES])");
  if (atlasCredit < 0 || atlasSlots < atlasCredit || atlasPage < atlasCredit) failures.push("P5a atlas backing allocates before process credit transfer");
  for (const forbidden of ["PREPARED_ATLAS_PROCESS_LEDGER", "Mutex<usize>", ".lock()", "Vec<PreparedAtlasPage>", "Vec::with_capacity(byte_len)", "Vec::with_capacity(source.len())", ".truncate(", ".fill(None)", "while ", "loop {"])
    if (atlasBoundary.includes(forbidden)) failures.push(`P5a atlas authority retains whole-backing work ${forbidden}`);
  requireAll(preparedSource, [
    "atlas_process_item_max_plus_one_is_nonblocking_and_recovers_every_permit",
    "abandoned_atlas_schedules_the_same_incremental_close_authority",
    "interrupted_atlas_close_rejoins_the_same_abandonment_authority",
    "atlas_allocation_refusal_preserves_the_packed_permit_ledger",
    "atlas_contended_permit_attempts_are_nonblocking_and_poison_free",
  ], "P5a atlas MAX + 1, allocation refusal, contention, abandonment, or interrupted-close law is missing");
  requireAll(atlasGpuBoundary, ["struct PreparedAtlasUploadCursor", "GlyphAtlasPages", "IconAtlasPages", "upload_glyph_atlas_page", "upload_icon_atlas_page", "cursor.page.checked_add(1)"], "P5a GPU atlas upload is not one retained page per grant");
  if ((atlasGpuBoundary.match(/cursor\.page\.checked_add\(1\)/g)?.length ?? 0) !== 2) failures.push("P5a glyph and icon atlas consumers must each advance exactly one page per grant");
  requireAll(atlasDrawBoundary, ["upload_glyph_atlas_page", "upload_icon_atlas_page", "origin: wgpu::Origin3d", "rows_per_image: Some(rows)"], "P5a atlas page upload boundary is incomplete");
  for (const forbidden of ["Vec::with_capacity(self.icons.pixels.len())", "Vec::with_capacity(self.atlas.pixels.len())", "keys().next().cloned()"])
    if (glue.includes(forbidden) || shell.includes(forbidden)) failures.push(`P5a mounted child authority retains uncredited whole owner ${forbidden}`);
  requireAll(engineBoundary, [
    "ENGINE_CANVAS_FRAME_PACKET_CAPACITY: usize = 256",
    "packets: Box<[Option<EngineCanvasPacket>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>",
    "take_packet_step",
    "Result<Option<EngineCanvasPacket>, EngineCanvasPacket>",
    "terminal_is_empty",
  ], "P5a EngineCanvas packet producer is not fixed, fallible, and one-owner resumable");
  for (const forbidden of ["packets: Vec<EngineCanvasPacket>", "take_packets(", "std::mem::take(&mut self.packets)"])
    if (engineBoundary.includes(forbidden)) failures.push(`P5a EngineCanvas boundary retains dynamic or bulk packet staging ${forbidden}`);
  requireAll(worldBoundary, [
    "WORLD3D_FRAME_RESOURCE_CAPACITY: usize = 256",
    "uploads: Box<[Option<PreparedRenderUpload>; WORLD3D_FRAME_RESOURCE_CAPACITY]>",
    "evictions: Box<[Option<PreparedRenderEviction>; WORLD3D_FRAME_RESOURCE_CAPACITY]>",
    "pub fn append_step",
    "Result<bool, World3dBuildRejected>",
    "checked_add(input.uploads.len())",
    "pub fn terminal_is_empty",
  ], "P5a World3D resource producer is not fixed, fallible, and one-owner resumable");
  for (const forbidden of ["uploads: Vec<", "evictions: Vec<", "mesh_requests: HashSet", "raster_requests: HashSet", "append_to(", ".append(", ".clear()"])
    if (worldBoundary.includes(forbidden)) failures.push(`P5a World3D boundary retains dynamic or bulk resource staging ${forbidden}`);
  requireAll(transaction, [
    "pub(crate) fn close_step(&mut self) -> bool",
    "pub(crate) fn terminal_is_empty(&self) -> bool",
  ], "P5a retained close and terminal witness are incomplete");
  requireAll(frameJob, [
    "WorkerJobSession::try_admit_owned(&mut self.original_job,&mut self.original_params,cx)",
    "FrameTransaction::new",
    "session.try_step_on_original_caller(cx)",
    "session.take_terminal()",
    "session.take_outcome(ticket)",
    "owner.take_retained_step_receipt()",
    "owner.acknowledge_outcome(",
    "session.has_original_cancel_alias_witness(cx.original_cancel_token())",
    "session.return_original_cancel_alias_step(cx.original_cancel_token(),grant)",
    "session.close_step(cx)",
    "transaction.close_step(cx)",
    "generation_is_fresh(presentation_generation,job.presentation_generation)",
  ], "P5a mounted original-caller admission/take/acknowledge/close/terminal laws are incomplete");
  if ((frameJob.match(/session\.take_rejected\(\)/g)?.length ?? 0) !== 0 || (frameJob.match(/session\.take_terminal\(\)/g)?.length ?? 0) !== 1 || (frameJob.match(/session\.take_outcome\(ticket\)/g)?.length ?? 0) !== 1)
    failures.push("P5a mounted frame owner must admit through try_admit_owned and keep exactly one take-terminal and one take-outcome receipt protocol");
  for (const forbidden of ["try_step_on_caller", "WorkerPool::new(", "thread::spawn", "loop {", "WorkerJobSession::try_new", "WorkerJobSessionAdmissionRejected", "close_step(self.retained)", "poll_runtime_and_resubmit(&mut self"]) if (frameJob.includes(forbidden)) failures.push(`P5a mounted worker path contains forbidden ${forbidden}`);
  requireAll(host, [
    "fn advance_frame_generation",
    "generation.checked_add(1)",
    "observe_presentation_input_generation(build_generation.0)",
    "AppPresentStep::Complete { generation, cursor, fullscreen, cursor_wake }",
    "if generation.0 != self.frame_generation",
    "self.snapshot_sink.publish",
  ], "P5a host does not fail closed on exhaustion/stale publication or atomically publish the accepted frame");
  requireAll(hostSource, ["mounted_pointer_storm_callback_p99_stays_below_two_milliseconds", "mounted_resize_storm_callback_p99_stays_below_two_milliseconds", "mounted_frame_generation_exhaustion_is_permanent_and_non_wrapping"], "P5a mounted input/resize/latency/exhaustion laws are missing");
  if (host.includes("frame_generation.wrapping_add") || host.includes("snapshot_sink.publish(crate::render_snapshot::RenderSnapshot::new(revision, generation, timestamp_us, cursor")) failures.push("P5a host retains wrapping generation or callback-local snapshot publication");
  requireAll(snapshot, ["pub fn next_revision(&self) -> Option<u64>", "current.checked_add(1)?", "compare_exchange(current, next"], "P5a snapshot revision does not permanently fail closed at exhaustion");
  if (!snapshotSource.includes("revision_exhaustion_is_permanent_and_preserves_last_valid_snapshot")) failures.push("P5a last-valid snapshot exhaustion law is missing");
  if (uiRuntimeGlue.includes("mod transaction") || uiRuntimeGlue.includes("pub use transaction::*")) failures.push("P5a dormant headless transaction is production reachable beside the mounted authority");
  return failures;
}
