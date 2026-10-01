#!/usr/bin/env python3
"""🧪️ S2-SPATIAL: converts the lowpoly editor root (`✏️editor/🦀️.rs`) from the bracketed scratch gestures to the tool
machines — deletes `transformBegin`/`transformEnd`/`paintStrokeBegin`/`paintStrokeEnd` and the retained paint-end
cursor, routes the gumball through one-shot transactions and every paint verb through the paint tool's retained step,
adds `canvasPointerUp`, previews open paint gestures from the transient, and re-declares the manifest. One read-modify-
write with anchored, count-checked replacements (peers edit the same file). Idempotent: an applied anchor is skipped.
"""
import re
import sys

PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"


def swap(text, old, new, where, count=1):
    if old not in text and new in text:
        return text
    found = text.count(old)
    assert found == count, f"{where}: expected {count} anchor(s), found {found}: {old[:160]!r}"
    return text.replace(old, new)


def span(text, start, end, new, where):
    """Replaces the text from the line holding `start` up to and including the line holding `end`."""
    if start not in text and (not new or new in text):
        return text
    a = text.index(start)
    a = text.rfind("\n", 0, a) + 1
    b = text.index(end, a)
    b = text.index("\n", b) + 1
    return text[:a] + new + text[b:]


RETAINED_WORK = '''struct LowpolyRetainedCommandWork {
    tool_id: &'static str,
    disposition: LowpolyCommandDisposition,
    operation_id: u64,
    generation: u64,
    base_revision: [u8; 32],
    context_identity: u64,
    stage: u8,
    replay_target: Option<u8>,
    complete: bool,
    closing: bool,
}

impl LowpolyRetainedCommandWork {
    fn new(tool_id: &'static str, disposition: LowpolyCommandDisposition, operation_id: u64, generation: u64, base_revision: [u8; 32], context_identity: u64) -> Self {
        Self { tool_id, disposition, operation_id, generation, base_revision, context_identity, stage: 0, replay_target: None, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<LowpolyPlayApp>> for LowpolyRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        lowpoly_tool_identity(self.tool_id) ^ self.operation_id.rotate_left(17) ^ self.generation.rotate_left(31) ^ self.context_identity.rotate_left(43) ^ (u64::from(self.disposition as u8) << 56)
    }

    fn extent(&self, command: &LowpolyCommand, _snapshot: &LowpolySnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>>) -> Option<usize> {
        (lowpoly_command_disposition(command.command_id()).is_some()).then_some(2)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LowpolyPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, history, interaction, hover: _hover, context, operation } = *input;
        if self.complete {
            return Err(Fault::from("lowpoly-retained-work-repeated"));
        }
        if self.closing {
            return Err(Fault::from("lowpoly-retained-work-closing"));
        }
        let context = context.ok_or_else(|| Fault::from("lowpoly-retained-context-absent"))?;
        if operation.operation_id != self.operation_id || operation.generation != self.generation || operation.canonical_base_revision != self.base_revision {
            return Err(Fault::from("lowpoly-retained-operation-freshness-drift"));
        }
        if context.identity_digest() != self.context_identity {
            return Err(Fault::from("lowpoly-retained-context-freshness-drift"));
        }
        if !lowpoly_command_admitted(command, snapshot, config) {
            return Err(Fault::from("lowpoly-retained-command-capacity"));
        }
        if self.stage == 0 {
            self.stage = 1;
            if let Some(target) = self.replay_target {
                if self.stage == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "lowpoly-command-workspace-replay", preview: b"{\\"en\\":\\"Restoring Lowpoly workspace\\",\\"de\\":\\"Lowpoly-Arbeitsbereich wird wiederhergestellt\\"}" });
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "lowpoly-command-scan", preview: b"{\\"en\\":\\"Preparing Lowpoly command\\",\\"de\\":\\"Lowpoly-Befehl wird vorbereitet\\"}" });
        }
        let step = lowpoly_retained_reduce(command, snapshot, config, history, interaction, context, operation)?;
        self.complete = true;
        Ok(step)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 72 {
            return Err(Fault::from("lowpoly-retained-checkpoint-capacity"));
        }
        target[..72].fill(0);
        target[..4].copy_from_slice(b"LPC3");
        target[4] = self.disposition as u8;
        target[5] = u8::from(self.complete);
        target[6] = self.stage;
        target[8..16].copy_from_slice(&lowpoly_tool_identity(self.tool_id).to_le_bytes());
        target[16..24].copy_from_slice(&self.operation_id.to_le_bytes());
        target[24..32].copy_from_slice(&self.generation.to_le_bytes());
        target[32..64].copy_from_slice(&self.base_revision);
        target[64..72].copy_from_slice(&self.context_identity.to_le_bytes());
        Ok(72)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 72 || &checkpoint[..4] != b"LPC3" || checkpoint[4] != self.disposition as u8 || checkpoint[5] > 1 || checkpoint[6] > 1 || checkpoint[7] != 0 {
            return Err(Fault::from("lowpoly-retained-checkpoint-invalid"));
        }
        let tool = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-tool"))?);
        let operation_id = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-operation"))?);
        let generation = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-generation"))?);
        let context_identity = u64::from_le_bytes(checkpoint[64..72].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-context"))?);
        if tool != lowpoly_tool_identity(self.tool_id) || operation_id != self.operation_id || generation != self.generation || checkpoint[32..64] != self.base_revision || context_identity != self.context_identity {
            return Err(Fault::from("lowpoly-retained-checkpoint-identity-mismatch"));
        }
        self.stage = 0;
        self.replay_target = (checkpoint[6] != 0).then_some(checkpoint[6]);
        self.complete = checkpoint[5] == 1;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if self.closing { InteractiveJobCloseStep::Complete } else { InteractiveJobCloseStep::Blocked }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}
'''


def main():
    text = open(PATH, encoding="utf-8").read()

    # region 🎮️ Command table and imports
    text = swap(text, '        "paintStrokeEnd" as "paint-stroke-end" => paint_stroke_end::PaintStrokeEnd,\n', "", "cmd paintStrokeEnd")
    text = swap(text, '        "transformEnd" as "transform-end" => transform_end::TransformEnd,\n', "", "cmd transformEnd")
    text = swap(text, '        "paintStrokeBegin" as "paint-stroke-begin" => paint_stroke_begin::PaintStrokeBegin,\n', "", "cmd paintStrokeBegin")
    text = swap(text, '        "transformBegin" as "transform-begin" => transform_begin::TransformBegin,\n', '        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,\n', "cmd canvasPointerUp")
    text = swap(text, "use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke, paint_stroke_begin, paint_stroke_end};", "use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke};", "use paint")
    text = swap(text, "use transform::{rotate_selection, scale_selection, transform_begin, transform_end, translate_selection};", "use transform::{rotate_selection, scale_selection, translate_selection};", "use transform")
    # endregion

    # region 🌉️ Action bridge
    text = swap(text, '            "paintStrokeEnd" => LowpolyCommand::PaintStrokeEnd(decode(action, none())?),\n', "", "bridge paintStrokeEnd")
    text = swap(text, '            "transformEnd" => LowpolyCommand::TransformEnd(decode(action, none())?),\n', "", "bridge transformEnd")
    text = swap(text, '            "paintStrokeBegin" => LowpolyCommand::PaintStrokeBegin(decode(action, none())?),\n', "", "bridge paintStrokeBegin")
    text = swap(text, '            "transformBegin" => LowpolyCommand::TransformBegin(decode(action, none())?),\n', '            "canvasPointerUp" => LowpolyCommand::CanvasPointerUp(decode(action, plain())?),\n', "bridge canvasPointerUp")
    # endregion

    # region 🧵️ Retained routing
    text = swap(text, '    "paintStrokeEnd",\n    "setActiveObject",\n', '    "setActiveObject",\n', "ids paintStrokeEnd")
    text = swap(text, '    "paintSample",\n    "paintStrokeBegin",\n    "transformBegin",\n    "extrude",\n', '    "paintSample",\n    "extrude",\n', "ids begins")
    text = swap(text, '    "fillBucket",\n    "transformEnd",\n    "engagementSubmit",\n', '    "fillBucket",\n    "engagementSubmit",\n', "ids transformEnd")
    text = swap(text, '    "canvasPointerMove",\n    "addPrimitive",\n];', '    "canvasPointerMove",\n    "canvasPointerUp",\n    "addPrimitive",\n];', "ids canvasPointerUp")
    text = swap(text, "    HostOnly = 3,\n    Transient = 4,\n    ConfigTransient = 5,\n    ArtifactTransient = 6,\n", "    HostOnly = 3,\n    ArtifactTransient = 6,\n", "disposition variants")
    text = span(text, '        "patchObject" | "addPaintLayer" | "paintFill" | "fillBucket" => LowpolyCommandDisposition::Artifact,', '        "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" => LowpolyCommandDisposition::ConfigTransient,',
                '''        // 🧲️ A fill commits one one-shot transaction; the gumball reads the live mesh cache to resolve its selection but
        // writes only its relative leaves, so its rehydrated cache is never republished.
        "patchObject" | "addPaintLayer" | "paintFill" | "fillBucket" | "translateSelection" | "rotateSelection" | "scaleSelection" => LowpolyCommandDisposition::Artifact,
        // 🕸️ Every one of these reaches `session::build_doc`/`mesh_edit`, which needs the session-local `mesh_workspace`
        // cache seeded from the LIVE persisted `LowpolyTransient` — `LowpolyDocument::reload_meshes` rejects a stale one
        // with `StaleMeshWorkspace` — so `lowpoly_retained_reduce`'s `threaded!` arms rehydrate scratch from
        // `context.transient` and publish the post-handle cache back: `Artifact` (the edit) `+ Transient` (the cache).
        "extrude" | "inset" | "bevel" | "loopCut" | "subdivide" | "triangulate" | "mirror" | "decimate" | "flipFaces" | "merge" | "dissolve" | "snap" | "toggleSmooth" | "unwrapActive" | "markUvSeam" | "clearSeam" | "engagementSubmit" | "deleteSelection" => LowpolyCommandDisposition::ArtifactTransient,
        "importSnapshotJson" | "replaceSnapshotJson" | "exportMesh" | "loadMeshRequest" | "importMeshFile" => LowpolyCommandDisposition::HostOnly,
        // 🖌️ Every paint verb drives the window's paint gesture in the transient, commits its transaction on release (or as a
        // one-shot) and, under the eyedropper, samples into config — the tick cannot tell statically which.
        "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => LowpolyCommandDisposition::ArtifactConfigTransient,
''', "dispositions")
    text = swap(text, "            LowpolyCommand::PaintStrokeEnd(_) => true,\n            LowpolyCommand::PaintStrokeBegin(_) | LowpolyCommand::TransformBegin(_) => true,\n", "", "admitted brackets")
    text = swap(text, "            | LowpolyCommand::ClearSeam(_)\n            | LowpolyCommand::TransformEnd(_) => true,\n", "            | LowpolyCommand::ClearSeam(_)\n            | LowpolyCommand::TranslateSelection(_)\n            | LowpolyCommand::RotateSelection(_)\n            | LowpolyCommand::ScaleSelection(_)\n            | LowpolyCommand::CanvasPointerUp(_) => true,\n", "admitted transforms")
    text = swap(text, "            LowpolyCommand::TranslateSelection(payload) => payload.mode.as_deref().is_none_or(field) && payload.ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),\n            LowpolyCommand::RotateSelection(payload) => payload.mode.as_deref().is_none_or(field) && payload.ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),\n            LowpolyCommand::ScaleSelection(payload) => payload.mode.as_deref().is_none_or(field) && payload.ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),\n", "", "admitted old transforms")
    text = swap(text, "            LowpolyCommand::PaintStroke(payload) => payload.object_id.as_deref().is_none_or(field),\n            LowpolyCommand::PaintAt(payload) => payload.object_id.as_deref().is_none_or(field),\n            LowpolyCommand::CanvasPointerDown(payload) => payload.object_id.as_deref().is_none_or(field),\n            LowpolyCommand::CanvasPointerMove(payload) => payload.object_id.as_deref().is_none_or(field),\n",
                "            LowpolyCommand::PaintStroke(payload) => payload.object_id.as_deref().is_none_or(field) && payload.phase.as_deref().is_none_or(field) && payload.reason.as_deref().is_none_or(field),\n            LowpolyCommand::PaintAt(payload) => payload.object_id.as_deref().is_none_or(field) && payload.phase.as_deref().is_none_or(field) && payload.reason.as_deref().is_none_or(field),\n            LowpolyCommand::CanvasPointerDown(payload) => payload.object_id.as_deref().is_none_or(field),\n            LowpolyCommand::CanvasPointerMove(payload) => payload.object_id.as_deref().is_none_or(field) && payload.samples.as_ref().is_none_or(|samples| samples.len() <= LOWPOLY_RETAINED_WORK_ITEMS),\n", "admitted paint")
    text = swap(text, "    macro_rules! threaded {\n        ($handle:expr) => {{\n            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone()).map_err(Fault::from)?;\n            threaded.set_selection_object_id(selection_object_id.clone());\n            threaded.set_selected_object_ids(selected_object_ids.clone());\n            let step_emit = ($handle)(&doc, &cfg, &mut threaded)?;\n            let transient = threaded.transient_snapshot().map_err(Fault::from)?;\n",
                "    macro_rules! threaded {\n        ($handle:expr) => {{\n            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone());\n            threaded.set_selection_object_id(selection_object_id.clone());\n            threaded.set_selected_object_ids(selected_object_ids.clone());\n            let step_emit = ($handle)(&doc, &cfg, &mut threaded)?;\n            let transient = threaded.transient_snapshot();\n", "threaded macro")
    text = swap(text, "            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step_emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } });\n        }};\n    }\n",
                "            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step_emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } });\n        }};\n    }\n    // 🧲️ The gumball reads the live mesh cache to resolve its selection and publishes nothing but its transaction.\n    macro_rules! read_threaded {\n        ($handle:expr) => {{\n            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone());\n            threaded.set_selection_object_id(selection_object_id.clone());\n            threaded.set_selected_object_ids(selected_object_ids.clone());\n            return Ok(ArtifactCommandWorkStep::Complete(($handle)(&doc, &cfg, &mut threaded)?));\n        }};\n    }\n", "read_threaded macro")
    text = span(text, "        LowpolyCommand::PaintStrokeBegin(_) => {", "        LowpolyCommand::TransformBegin(_) => {", "", "reduce begins head")
    text = swap(text, "            let transient = context.transient.begin_transform_drag();\n            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit::default(), ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } });\n        }\n", "", "reduce transform begin body")
    text = swap(text, "        LowpolyCommand::TranslateSelection(payload) => threaded!(|doc, cfg, ctx| translate_selection::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::RotateSelection(payload) => threaded!(|doc, cfg, ctx| rotate_selection::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::ScaleSelection(payload) => threaded!(|doc, cfg, ctx| scale_selection::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::TransformEnd(payload) => threaded!(|doc, cfg, ctx| transform_end::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::PaintStroke(payload) => threaded!(|doc, cfg, ctx| paint_stroke::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::PaintAt(payload) => threaded!(|doc, cfg, ctx| paint_at::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::CanvasPointerDown(payload) => threaded!(|doc, cfg, ctx| canvas_pointer_down::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::CanvasPointerMove(payload) => threaded!(|doc, cfg, ctx| canvas_pointer_move::handle(payload, doc, cfg, ctx)),\n",
                "        LowpolyCommand::TranslateSelection(payload) => read_threaded!(|doc, cfg, ctx| translate_selection::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::RotateSelection(payload) => read_threaded!(|doc, cfg, ctx| rotate_selection::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::ScaleSelection(payload) => read_threaded!(|doc, cfg, ctx| scale_selection::handle(payload, doc, cfg, ctx)),\n        LowpolyCommand::PaintStroke(payload) => return paint::lowpoly_paint_step(payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),\n        LowpolyCommand::PaintAt(payload) => return paint::lowpoly_paint_step(payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),\n        LowpolyCommand::CanvasPointerDown(payload) => return paint::lowpoly_paint_step(Some(\"stream\"), None, payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),\n        LowpolyCommand::CanvasPointerMove(payload) => return paint::lowpoly_paint_step(Some(\"stream\"), None, payload.object_id.as_deref(), &payload.points(), false, snapshot, config, context, operation),\n        LowpolyCommand::CanvasPointerUp(payload) => {\n            let (phase, reason) = payload.phase();\n            return paint::lowpoly_paint_step(Some(phase), reason, None, &[], false, snapshot, config, context, operation);\n        }\n", "reduce transforms and paint")
    text = swap(text, "        // 🪣️ `ctx.fill_at` reads/writes only `stroke_dirty` (a render-side texture-cache invalidation\n        // counter, never persisted, never read back for any semantic decision) — a single-shot fill\n        // needs no transient rehydration, unlike the drag-tick commands above.\n", "        // 🪣️ A one-shot fill reads only the committed layer, so it needs no transient rehydration.\n", "reduce fill comment")
    text = swap(text, "        // 🚧️ `PaintStrokeEnd` never reaches this reducer — `LowpolyRetainedCommandWork::step` intercepts\n        // it before calling `lowpoly_retained_reduce` and routes it to `paint_end_step`'s dedicated\n        // bounded-cursor machinery instead. This arm exists only so the match stays exhaustive.\n        LowpolyCommand::PaintStrokeEnd(_) => return Err(Fault::from(\"lowpoly-paint-stroke-end-routes-through-dedicated-step\")),\n", "", "reduce paint end")
    if RETAINED_WORK not in text:
        start = text.index("struct LowpolyRetainedCommandWork {")
        end = text.index("struct LowpolyCommandJobFactory {")
        text = text[:start] + RETAINED_WORK + "\n" + text[end:]
    assert text.count("impl ArtifactCommandWork<EditorApp<LowpolyPlayApp>> for LowpolyRetainedCommandWork {") == 1, "retained work impl"
    # endregion

    # region 📬️ Publication contracts and preparation
    lane = "semio_framework_plugin::ArtifactToolPublicationLane::"
    text = swap(text, f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "paintStrokeEnd", lanes: &[{lane}Artifact, {lane}Transient] }},\n', "", "contract paintStrokeEnd")
    text = swap(text, f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "paintStrokeBegin", lanes: &[{lane}Transient] }},\n        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "transformBegin", lanes: &[{lane}Transient] }},\n', "", "contract begins")
    text = swap(text, f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "transformEnd", lanes: &[{lane}Artifact, {lane}Transient] }},\n', "", "contract transformEnd")
    for verb in ("translateSelection", "rotateSelection", "scaleSelection"):
        text = swap(text, f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "{verb}", lanes: &[{lane}Artifact, {lane}Transient] }},\n', f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "{verb}", lanes: &[{lane}Artifact] }},\n', f"contract {verb}")
    paint_old = "".join(f'        semio_framework_plugin::ArtifactToolPublicationContract {{ tool_id: "{verb}", lanes: &[{lane}Config, {lane}Transient] }},\n' for verb in ("paintStroke", "paintAt", "canvasPointerDown", "canvasPointerMove"))
    paint_new = "".join(f'        semio_framework_plugin::ArtifactToolPublicationContract {{\n            tool_id: "{verb}",\n            lanes: &[{lane}Artifact, {lane}Config, {lane}Transient],\n        }},\n' for verb in ("paintStroke", "paintAt", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp"))
    text = swap(text, paint_old, paint_new, "contract paint")
    text = swap(text, "        LowpolyMutation::EditPaintLayer(_) => Err(\"Lowpoly paint edit exceeds its fixed run envelope\".into()),\n",
                "        LowpolyMutation::EditPaintLayer(_) => Err(\"Lowpoly paint edit exceeds its fixed run envelope\".into()),\n        LowpolyMutation::ApplyPaintStroke(payload) => Ok(payload.object_id.len().saturating_add(payload.points.len().saturating_mul(size_of::<[f32; 2]>())).saturating_add(16)),\n        LowpolyMutation::MoveSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(12)),\n        LowpolyMutation::RotateSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(28)),\n        LowpolyMutation::ScaleSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(24)),\n", "retained bytes arms")
    text = swap(text, "/// 📬️ Exact per-variant byte accounting for every one of `LowpolyMutation`'s 17 declared variants —", "/// 📬️ Exact per-variant byte accounting for every one of `LowpolyMutation`'s 21 declared variants —", "retained bytes doc")
    text = swap(text, "    // ↩️ One forward row plus its point inverse (which, for `create-mesh`, carries the prior content too).\n    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes.saturating_mul(2)))\n}\n\nfn prepare_lowpoly_artifact(",
                "    // ↩️ One forward row plus its point inverse. A relative leaf's inverse carries what the base held — a stroke's\n    // overwritten pixels, a selection motion's prior mesh content — which preflight never sees, so it declares the\n    // lane's whole one-item envelope; every other inverse is bounded by its forward twin.\n    let inverse_bytes = match mutation {\n        LowpolyMutation::ApplyPaintStroke(_) | LowpolyMutation::MoveSelection(_) | LowpolyMutation::RotateSelection(_) | LowpolyMutation::ScaleSelection(_) => store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES.saturating_sub(retained_bytes),\n        _ => retained_bytes,\n    };\n    Ok(store::ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes.saturating_add(inverse_bytes)))\n}\n\nfn prepare_lowpoly_artifact(", "artifact footprint")
    # endregion

    # region 🖼️ Render and request contexts
    text = swap(text, "    scratch: &mut LowpolyScratch,\n    interaction: Option<&InteractionView<'_>>,\n) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {\n    let projection = doc.snapshot;\n",
                "    scratch: &mut LowpolyScratch,\n    interaction: Option<&InteractionView<'_>>,\n    preview: Option<&LowpolySnapshot>,\n) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {\n    let projection = preview.unwrap_or(doc.snapshot);\n", "render signature")
    text = swap(text, "    let scratch_projection = scratch.transform_projection();\n    let texture_cache = scratch.textures().clone();\n    let render_projection = scratch_projection.as_ref().unwrap_or(projection);\n    let view = LowpolyView { snapshot: render_projection, config };\n",
                "    let texture_cache = scratch.textures().clone();\n    let view = LowpolyView { snapshot: projection, config };\n", "render projection")
    text = swap(text, "        lowpoly_render(body_key, doc, cfg, view_state, &mut LowpolyScratch::default(), None)\n", "        lowpoly_render(body_key, doc, cfg, view_state, &mut LowpolyScratch::default(), None, None)\n", "render call plain")
    text = swap(text, "        let mut scratch = LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default()).map_err(|error| semio_framework_plugin::PluginAssemblyError::new(\"lowpoly.transient\", error))?;\n        lowpoly_render(body_key, doc, cfg, view_state, &mut scratch, Some(interaction))\n",
                "        let mut scratch = LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default());\n        let preview = transient.snapshot.paint_preview(doc.snapshot);\n        lowpoly_render(body_key, doc, cfg, view_state, &mut scratch, Some(interaction), preview.as_ref())\n", "render call request")
    text = swap(text, "        let scratch = LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default()).map_err(|error| MediaError::Payload(port.into(), error))?;\n", "        let scratch = LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default());\n", "export scratch")
    text = swap(text, "        let mut scratch = LowpolyScratch::from_transient(&LowpolyTransient::default(), selection).map_err(Fault::from)?;\n", "        let mut scratch = LowpolyScratch::from_transient(&LowpolyTransient::default(), selection);\n", "handle scratch")
    # endregion

    # region 🧾️ Tool proofs
    proof = 'ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),\n'
    for verb in ("paintStrokeEnd", "paintStrokeBegin", "transformBegin", "transformEnd"):
        text = swap(text, f'            "{verb}" => {proof}', "", f"proof {verb}")
    text = swap(text, f'            "canvasPointerMove" => {proof}', f'            "canvasPointerMove" => {proof}            "canvasPointerUp" => {proof}', "proof canvasPointerUp")
    # endregion

    # region 🧱️ Manifest
    text = swap(text, '            .mutation("transformEnd", LocalizedLabel::native("Transform End", "Transformation beenden"))\n', "", "manifest transformEnd")
    text = swap(text, '            .mutation("paintStrokeEnd", LocalizedLabel::native("Paint Stroke End", "Malstrich beenden"))\n', "", "manifest paintStrokeEnd")
    text = swap(text, "            // 👁️ Ephemeral view state — selection, camera, hover, and the gesture drafts that emit no operations\n            // mid-drag (paint ticks, gumball scratch, eyedropper sample).\n", "            // 👁️ Ephemeral view state — selection, camera, hover and the eyedropper sample.\n", "manifest view comment")
    text = swap(text, '            .action_with(semio_framework_plugin::ActionDefinition::new("paintStrokeBegin", LocalizedLabel::native("Paint Stroke Begin", "Malstrich beginnen"), semio_framework_plugin::ActionKind::View, "paintbrush"))\n', "", "manifest paintStrokeBegin")
    paint_kinds = (
        ('paintStroke', 'LocalizedLabel::native("Paint Stroke", "Malstrich")', '"paintbrush"'),
        ('paintAt', 'LocalizedLabel::native("Paint At", "Malen bei")', '"paintbrush"'),
        ('canvasPointerDown', 'LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt")', '"mouse-pointer"'),
        ('canvasPointerMove', 'LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt")', '"mouse-pointer"'),
    )
    for verb, label, icon in paint_kinds:
        text = swap(text, f'            .action_with(semio_framework_plugin::ActionDefinition::new("{verb}", {label}, semio_framework_plugin::ActionKind::View, {icon}))\n', f'            .action_with(semio_framework_plugin::ActionDefinition::new("{verb}", {label}, semio_framework_plugin::ActionKind::Mutation, {icon}))\n', f"manifest kind {verb}")
    text = swap(text, '            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)\n', '            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)\n            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))\n            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)\n', "manifest canvasPointerUp")
    text = swap(text, '            .action_with(semio_framework_plugin::ActionDefinition::new("transformBegin", LocalizedLabel::native("Transform Begin", "Transformation beginnen"), semio_framework_plugin::ActionKind::View, "move"))\n', "", "manifest transformBegin")
    for verb in ("paintStrokeEnd", "transformEnd", "paintStrokeBegin", "transformBegin"):
        text = swap(text, f'            .action_interactive_job("{verb}", InteractiveJobClassification::Migrated)\n', "", f"job {verb}")
    text = swap(text, '            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)\n', '            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)\n            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)\n', "job canvasPointerUp")
    describe = {
        "translateSelection": ('Moves the mesh selection — the selected vertices, edges or faces of the active object, else every selected object — by dx, dy and dz as one undoable edit whose offset stays editable in history.', 'Verschiebt die Netzauswahl — die ausgewählten Punkte, Kanten oder Flächen des aktiven Objekts, sonst jedes ausgewählte Objekt — um dx, dy und dz als eine rückgängig machbare Änderung, deren Versatz im Verlauf bearbeitbar bleibt.'),
        "rotateSelection": ('Rotates the mesh selection by an angle in radians around the axis ax, ay, az through its centre as one undoable edit whose angle and axis stay editable in history.', 'Dreht die Netzauswahl um einen Winkel im Bogenmaß um die Achse ax, ay, az durch ihre Mitte als eine rückgängig machbare Änderung, deren Winkel und Achse im Verlauf bearbeitbar bleiben.'),
        "scaleSelection": ('Scales the mesh selection by sx, sy and sz about its centre as one undoable edit whose factors stay editable in history.', 'Skaliert die Netzauswahl um sx, sy und sz um ihre Mitte als eine rückgängig machbare Änderung, deren Faktoren im Verlauf bearbeitbar bleiben.'),
    }
    for verb, (en, de) in describe.items():
        text = re.sub(r'            \.action_describe\("' + verb + r'", LocalizedLabel::native\("[^"]*", "[^"]*"\)\)\n', lambda _m, verb=verb, en=en, de=de: f'            .action_describe("{verb}", LocalizedLabel::native("{en}", "{de}"))\n', text, count=1)
    text = re.sub(r'            \.action_describe\("transformBegin", LocalizedLabel::native\("[^"]*", "[^"]*"\)\)\n', "", text, count=1)
    text = re.sub(r'            \.action_describe\("transformEnd", LocalizedLabel::native\("[^"]*", "[^"]*"\)\)\n', "", text, count=1)
    if '.action_describe("paintAt"' not in text:
        text = swap(text, '            .action_describe("paintFill", ', '            .action_describe("paintAt", LocalizedLabel::native("Paints one dab of the active brush at the texture point u, v of the given or active object; a host drag streams its dabs into one stroke that the release writes as one undoable edit whose brush and dabs stay editable in history.", "Malt einen Tupfer des aktiven Pinsels am Texturpunkt u, v des angegebenen oder aktiven Objekts; ein Ziehen des Hosts sammelt seine Tupfer zu einem Strich, den das Loslassen als eine rückgängig machbare Änderung schreibt, deren Pinsel und Tupfer im Verlauf bearbeitbar bleiben."))\n            .action_describe("paintFill", ', "describe paintAt")
    for verb in ("paintStrokeBegin", "paintStrokeEnd", "transformBegin", "transformEnd"):
        text = swap(text, f'            .action_audience("{verb}", semio_framework_plugin::CapabilityAudience::Input)\n', "", f"audience {verb}")
    # endregion

    leftovers = [line for line in text.splitlines() if re.search(r"transformBegin|transformEnd|paintStrokeBegin|paintStrokeEnd|TransformBegin|TransformEnd|PaintStrokeBegin|PaintStrokeEnd|paint_end_step|paint_runs|stroke_diff_parts|finish_stroke_drag|begin_stroke_drag|begin_transform_drag|transform_projection", line)]
    assert not leftovers, "\n".join(leftovers)
    open(PATH, "w", encoding="utf-8").write(text)
    print("ok")


if __name__ == "__main__":
    main()
