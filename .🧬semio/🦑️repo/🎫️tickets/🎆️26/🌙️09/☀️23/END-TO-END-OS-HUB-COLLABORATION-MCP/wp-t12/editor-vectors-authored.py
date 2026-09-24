"""✍️ Hand-authored (before, mutation, after) vectors for the editor/viewer state lanes that carry no owner vectors,
merged into `wp-t12/editor-vectors.json`. Every `after` is written by hand from the leaf's own semantics (a setter
replaces exactly its field; a replace swaps the whole record); the harness then measures the production diff and
diagnostics, and any disagreement with a hand-written `after` is a finding, never an overwrite."""
import copy, json, os

root = "/Users/ueli/Documents/semio/"
path = root + ".tmp-ticket/wp-t12/editor-vectors.json"
spec = json.load(open(path, encoding="utf-8"))


def scenario(kind, before, mutation, change, story):
    after = copy.deepcopy(before)
    change(after)
    return {"kind": kind, "status": "applied", "before": before, "mutation": mutation, "after": after, "story": story}


def keep(applied, story):
    return {"kind": applied["kind"], "status": "no-op", "before": applied["after"], "mutation": applied["mutation"], "after": applied["after"], "story": story}


def put(index, scenarios):
    spec[str(index)] = {"scenarios": scenarios, "missingKinds": []}


POSE = {"position": [10.0, -10.0, 8.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0}
POSE2 = {"position": [4.0, -6.0, 5.0], "target": [1.0, 2.0, 0.5], "zoom": 2.0}
for index in (0, 2):
    applied = scenario("set-camera", {"camera": POSE}, {"kind": "set-camera", "camera": POSE2}, lambda s: s.update(camera=POSE2), "Orbiting the 3d model window to a new pose replaces the camera and nothing else.")
    put(index, [applied, keep(applied, "Setting the camera to the pose it already holds is the leaf's warned no-op.")])

put(4, [scenario("set-playback-clock", {"clock": None}, {"SetPlaybackClock": {"clock": {"phase": 0.25, "reverse": False}}}, lambda s: s.update(clock={"phase": 0.25, "reverse": False}), "Starting playback from a stopped results window sets the clock to a quarter phase, running forward.")])

ASSIGN2 = [{"slotId": "r0c0", "tileId": "grass"}, {"slotId": "r0c1", "tileId": "water"}]
for index in (13, 16):
    put(index, [scenario("set-solve", {"assignments": [], "contradiction": False}, {"SetSolve": {"assignments": ASSIGN2, "contradiction": False}}, lambda s: s.update(assignments=ASSIGN2), "A finished solve replaces the empty assignment table with its two slot assignments.")])

CFG = {"cameraX": 0.0, "cameraY": 0.0, "cameraZoom": 1.0, "activeTileId": ""}
CFG2 = {"cameraX": 12.5, "cameraY": -3.0, "cameraZoom": 1.5, "activeTileId": "water"}
for index in (14, 17):
    put(index, [
        scenario("change-active-tile", CFG, {"mutation": "changeActiveTile", "payload": {"tileId": "grass"}}, lambda s: s.update(activeTileId="grass"), "Choosing the grass tile makes it the tile a pin gesture assigns in this pane."),
        scenario("change-camera", CFG, {"mutation": "changeCamera", "payload": {"x": 12.5, "y": -3.0, "zoom": 1.5}}, lambda s: s.update(cameraX=12.5, cameraY=-3.0, cameraZoom=1.5), "Panning and zooming the pane moves its camera and leaves the active tile alone."),
        scenario("replace-config", CFG, {"mutation": "replaceConfig", "payload": {"config": CFG2}}, lambda s: s.update(CFG2), "Replacing the pane config swaps the camera and the active tile in one step."),
    ])

put(15, [scenario("set-solve", {"outputPixels": None, "contradiction": False, "outputWidth": 0, "outputHeight": 0}, {"SetSolve": {"outputPixels": "AAECAw==", "contradiction": False, "outputWidth": 2, "outputHeight": 2}}, lambda s: s.update(outputPixels="AAECAw==", outputWidth=2, outputHeight=2), "A finished 2x2 solve caches its four palette indices and the extent they were solved for.")])

MAP = {"layerVisibility": {"parcels": True}, "cameraJson": "{\"x\":0,\"y\":0,\"zoom\":1}", "renderMode": "combined", "vectorStyle": "colored", "lodMode": "automatic", "layerStrokeScale": {"parcels": 1.0}}
map_rows = [
    scenario("set-layer-visibility", MAP, {"operation": "setLayerVisibility", "layerId": "parcels", "visible": False}, lambda s: s["layerVisibility"].update(parcels=False), "Hiding the parcels layer flips its visibility entry and nothing else."),
    scenario("set-camera", MAP, {"operation": "setCamera", "cameraJson": "{\"x\":120,\"y\":-40,\"zoom\":3}"}, lambda s: s.update(cameraJson="{\"x\":120,\"y\":-40,\"zoom\":3}"), "Panning the map replaces the camera JSON."),
    scenario("set-render-mode", MAP, {"operation": "setRenderMode", "value": "vector"}, lambda s: s.update(renderMode="vector"), "Switching to vector rendering replaces the render mode."),
    scenario("set-vector-style", MAP, {"operation": "setVectorStyle", "value": "figureGround"}, lambda s: s.update(vectorStyle="figureGround"), "Switching to the figure-ground style replaces the vector style."),
    scenario("set-lod-mode", MAP, {"operation": "setLodMode", "value": "fine"}, lambda s: s.update(lodMode="fine"), "Pinning the fine LOD tier replaces the automatic one."),
    scenario("set-layer-stroke-scale", MAP, {"operation": "setLayerStrokeScale", "layerId": "parcels", "value": 2.5}, lambda s: s["layerStrokeScale"].update(parcels=2.5), "Thickening the parcels strokes sets their multiplier to 2.5."),
]
put(18, map_rows + [keep(r, "Setting a map window field to the value it already holds is the leaf's warned no-op.") for r in map_rows])

for index in (19, 20):
    put(index, [scenario("set-generation-preview", {"generationPreviewText": None}, {"SetGenerationPreview": {"preview_text": "generation 1: 12 parts"}}, lambda s: s.update(generationPreviewText="generation 1: 12 parts"), "The first generation preview caches its text where there was none.")])

PREVIEW = {"position": [6.0, 6.0, 6.0], "target": [0.0, 0.0, 0.0], "fov": 45.0}
PREVIEW2 = {"position": [2.0, -8.0, 3.0], "target": [0.0, 0.0, 1.0], "fov": 35.0}
G3 = {"lodMode": "coarse", "showMode": "shaded", "camera": {"x": 0.0, "y": 0.0, "zoom": 1.0}, "previewCamera": PREVIEW, "sunJson": "{\"azimuth\":135,\"elevation\":45}", "selectedGenerationId": None}
G3B = {"lodMode": "fine", "showMode": "wireframe", "camera": {"x": 40.0, "y": 25.0, "zoom": 0.75}, "previewCamera": PREVIEW2, "sunJson": "{\"azimuth\":90,\"elevation\":30}", "selectedGenerationId": "generation-2"}
g3 = [
    scenario("set-snapshot", G3, {"SetSnapshot": {"config": G3B}}, lambda s: s.update(copy.deepcopy(G3B)), "Restoring a saved config replaces every field at once."),
    scenario("set-sun", G3, {"SetSun": {"json": "{\"azimuth\":90,\"elevation\":30}"}}, lambda s: s.update(sunJson="{\"azimuth\":90,\"elevation\":30}"), "Moving the sun replaces its JSON."),
    scenario("set-show-mode", G3, {"SetShowMode": {"value": "wireframe"}}, lambda s: s.update(showMode="wireframe"), "Switching to wireframe replaces the preview shading mode."),
    scenario("set-preview-camera", G3, {"SetPreviewCamera": {"camera": PREVIEW2}}, lambda s: s.update(previewCamera=PREVIEW2), "Orbiting the 3d preview replaces its camera."),
    scenario("set-lod-mode", G3, {"SetLodMode": {"value": "fine"}}, lambda s: s.update(lodMode="fine"), "Choosing the fine tessellation replaces the LOD tier."),
    scenario("set-camera", G3, {"SetCamera": {"camera": {"x": 40.0, "y": 25.0, "zoom": 0.75}}}, lambda s: s.update(camera={"x": 40.0, "y": 25.0, "zoom": 0.75}), "Panning the flow-graph canvas replaces its camera."),
    scenario("set-selected-generation", G3, {"SetSelectedGeneration": {"selectedGenerationId": "generation-2"}}, lambda s: s.update(selectedGenerationId="generation-2"), "Selecting the second generation records its id."),
]
put(21, g3 + [keep(r, "Setting the camera to the one the config already holds is the leaf's warned no-op.") for r in g3 if r["kind"] in ("set-preview-camera", "set-camera")])

put(22, [scenario("set-preview-eval", {"previewEvalText": None}, {"SetPreviewEval": {"evalText": "{\"meshes\":1}"}}, lambda s: s.update(previewEvalText="{\"meshes\":1}"), "The first flow evaluation caches its JSON for the preview window.")])

VIEW = {"position": [6.0, 6.0, 6.0], "target": [0.0, 0.0, 0.0], "fov": 45.0}
VIEW2 = {"position": [2.0, -8.0, 3.0], "target": [0.0, 0.0, 1.0], "fov": 35.0}
put(23, [
    scenario("set-show-mode", {"previewCamera": VIEW, "showMode": "shaded"}, {"SetShowMode": {"value": "points"}}, lambda s: s.update(showMode="points"), "Switching this viewer to points replaces the shading mode it broadcasts."),
    scenario("set-preview-camera", {"previewCamera": VIEW, "showMode": "shaded"}, {"SetPreviewCamera": {"camera": VIEW2}}, lambda s: s.update(previewCamera=VIEW2), "Orbiting this viewer's preview replaces the camera it broadcasts."),
])

VC = {"lodMode": "", "showMode": "shaded", "previewCamera": VIEW, "sunJson": "{\"azimuth\":135,\"elevation\":45}", "activeExampleId": None}
put(24, [
    scenario("set-sun", VC, {"SetSun": {"json": "{\"azimuth\":90,\"elevation\":30}"}}, lambda s: s.update(sunJson="{\"azimuth\":90,\"elevation\":30}"), "Moving the sun replaces its JSON."),
    scenario("set-active-example", VC, {"SetActiveExample": {"value": "box-fillet-preview"}}, lambda s: s.update(activeExampleId="box-fillet-preview"), "Picking the box-fillet example makes it what this read-only surface shows."),
    scenario("set-show-mode", VC, {"SetShowMode": {"value": "shaded+edges"}}, lambda s: s.update(showMode="shaded+edges"), "Switching to shaded with edges replaces the shading mode."),
    scenario("set-preview-camera", VC, {"SetPreviewCamera": {"camera": VIEW2}}, lambda s: s.update(previewCamera=VIEW2), "Orbiting the preview replaces its camera."),
    scenario("set-lod-mode", VC, {"SetLodMode": {"value": "coarse"}}, lambda s: s.update(lodMode="coarse"), "Choosing the coarse tessellation replaces the LOD tier."),
])

WT = {"editorSelection": None, "lintGeneration": 0, "engagementInput": ""}
put(27, [
    scenario("set-engagement-input", WT, {"kind": "set-engagement-input", "value": "shorten the second paragraph"}, lambda s: s.update(engagementInput="shorten the second paragraph"), "Typing an engagement prompt replaces the engagement input."),
    scenario("set-editor-selection", WT, {"kind": "set-editor-selection", "selection": {"start": 4, "end": 17}}, lambda s: s.update(editorSelection={"start": 4, "end": 17}), "Selecting characters 4 to 17 records the editor selection."),
    scenario("set-lint-generation", WT, {"kind": "set-lint-generation", "value": 3}, lambda s: s.update(lintGeneration=3), "A third lint pass bumps the lint generation to 3."),
])

WC = {"camera": {"x": 0.0, "y": 0.0, "zoom": 1.0}, "editorSettings": {"showLineNumbers": False, "fontPx": 14, "lineHeight": 20, "tabSize": 4}}
put(28, [
    scenario("set-editor-settings", WC, {"kind": "set-editor-settings", "settings": {"showLineNumbers": True, "fontPx": 16, "lineHeight": 24, "tabSize": 2}}, lambda s: s.update(editorSettings={"showLineNumbers": True, "fontPx": 16, "lineHeight": 24, "tabSize": 2}), "Turning on line numbers with a larger font replaces the editor settings."),
    scenario("set-camera", WC, {"kind": "set-camera", "camera": {"x": 0.0, "y": 320.0, "zoom": 1.25}}, lambda s: s.update(camera={"x": 0.0, "y": 320.0, "zoom": 1.25}), "Scrolling and zooming the main window replaces its camera."),
])

cam = scenario("set-camera", {"camera": {"x": 0.0, "y": 0.0, "zoom": 1.0}}, {"kind": "set-camera", "camera": {"x": -80.0, "y": 45.0, "zoom": 0.8}}, lambda s: s.update(camera={"x": -80.0, "y": 45.0, "zoom": 0.8}), "Panning the canvas replaces the window camera.")
put(30, [cam, keep(cam, "Setting the canvas camera to the one it already holds is the leaf's warned no-op.")])

put(33, [scenario("set-brush-preview", {"brushPreview": None}, {"SetBrushPreview": {"preview": {"position": [3.0, 1.0, -2.0], "direction": [0.0, 1.0, 0.0]}}}, lambda s: s.update(brushPreview={"position": [3.0, 1.0, -2.0], "direction": [0.0, 1.0, 0.0]}), "Hovering a block face shows the brush preview at that position, facing up.")])

json.dump(spec, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(sorted(int(k) for k in spec))
