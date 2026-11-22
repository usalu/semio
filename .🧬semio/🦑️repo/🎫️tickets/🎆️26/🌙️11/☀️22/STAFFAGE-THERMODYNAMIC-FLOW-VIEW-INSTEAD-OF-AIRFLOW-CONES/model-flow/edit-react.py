"""One-shot editor of the React 3D pipeline: the stage's model carries no flow geometry, the downloads do."""
import sys, pathlib
R = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🧊️spatial")
def edit(rel, pairs):
    path = R / rel
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"{rel}: expected one match of {old[:80]!r}, found {text.count(old)}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")
edit("🏭️prepare/🟦️.ts", [
    ("the clip keeps its root motion exactly as the downloads do, the viewer follows the figure instead of cutting its travel.\n",
     "the clip keeps its root motion exactly as the downloads do, the viewer follows the figure instead of cutting its travel.\n * The air flow is shown once: the stage animates it live over the model (the flow overlay), so the model of the stage is built without flow geometry (`flowGeometry: false`); the downloads carry the developed flow as geometry instead.\n"),
    ("export function analyticOptionsOf(request: Pick<SpatialRequest, \"environment\" | \"thermalQuantity\" | \"forceQuantity\" | \"range\" | \"theme\" | \"flow\">): Pick<AssetModelOptions, \"environment\" | \"thermalSpanK\" | \"thermalQuantity\" | \"forceQuantity\" | \"theme\" | \"emitterMarkers\"> {\n  return {\n    ...((request.flow?.length ?? 0) > 0 ? { emitterMarkers: false } : {}),\n",
     "export function analyticOptionsOf(request: Pick<SpatialRequest, \"environment\" | \"thermalQuantity\" | \"forceQuantity\" | \"range\" | \"theme\">): Pick<AssetModelOptions, \"environment\" | \"thermalSpanK\" | \"thermalQuantity\" | \"forceQuantity\" | \"theme\"> {\n  return {\n"),
    ("/** 🌟️ The options `modelOfAsset` makes the model of a request from: {@link analyticOptionsOf}, the pose or the clip (its first instant is the static state) and every authored scale. A transition has no model. */",
     "/** 🌟️ The options `modelOfAsset` makes the model of the stage from: {@link analyticOptionsOf}, the pose or the clip (its first instant is the static state), every authored scale and no flow geometry (the stage animates the air flow itself). A transition has no model. */"),
    ("  const base = { ...analyticOptionsOf(request), ...(request.scales === undefined ? {} : { scales: request.scales }) };",
     "  const base = { ...analyticOptionsOf(request), flowGeometry: false, ...(request.scales === undefined ? {} : { scales: request.scales }) };"),
    ("a glyph (arrow, arc, emitter cone) is drawn only", "a glyph (arrow, arc, centre of mass) is drawn only"),
])
edit("🟦️.ts", [
    ("  /** 🌊️ The layers of the air flow shown over the thermal view; with any layer the model is built without emitter markers, since the flow itself shows where air leaves. */",
     "  /** 🌊️ The layers of the air flow the stage animates over the thermal view. They are not part of the model: the model of the stage never carries flow geometry, so the flow is shown once. */"),
    (", request.theme ?? null, request.scales ?? null, (request.flow?.length ?? 0) > 0]);", ", request.theme ?? null, request.scales ?? null]);"),
])
print("ok")
