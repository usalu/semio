"""🔍️ Writes `wp-t12/editor-decisions.json`: per editor/viewer state-lane vocabulary, the one-sentence summary of what
the lane holds (read from its snapshot struct) and the surveyed no-oracle decision (rationale, ecosystems, candidates,
why none qualifies). Candidates are the general state containers a reviewer would reach for first; each is judged
against what the case actually asserts (typed per-kind change, no-op guard, computed inverse)."""
import json
root = "/Users/ueli/Documents/semio/"
SUMMARY = {
    0: "the 3d model window's camera pose (position, target, zoom).",
    1: "the simulation settings (zone and system timesteps, warm-up days) and the per-surface result field the 3d window colours by.",
    2: "the viewer's 3d model window camera pose (position, target, zoom).",
    3: "the host-pushed block-kind contribution list (`contributionsJson`) that hot-swaps `playbook.blockKind` installs.",
    4: "the results window's playback clock (phase and direction), or none while playback is stopped.",
    5: "the equation graph window's camera (x, y, zoom).",
    6: "the shot selection and live camera this collaborator broadcasts to peers.",
    7: "the default shot and asset formats, the shot selection, the centre-model toggle, the viewport fit revision, the camera-label draft and the live camera.",
    8: "the active register, the adjacency-kind filter and the graph camera this collaborator broadcasts to peers.",
    9: "the search query and history, the last action result and the last analysis result.",
    10: "the host-declared contribution list (`contributionsJson`) projected into the forms app.",
    11: "the canvas camera (x, y, zoom) this collaborator broadcasts to peers.",
    12: "the last run's output scope and the host-pushed module contribution list.",
    13: "the last solve's slot assignments and whether it ended in a contradiction.",
    14: "this pane's camera (x, y, zoom) and the tile a pin gesture assigns.",
    15: "the last solve's palette-index pixels, its output extent and whether it ended in a contradiction.",
    16: "the last solve's slot assignments and whether it ended in a contradiction.",
    17: "this pane's camera (x, y, zoom) and the tile a pin gesture assigns.",
    18: "per-layer visibility and stroke scale, the map camera, the render mode, the vector style and the LOD tier.",
    19: "the text of the last generation preview, or none before the first one.",
    20: "the text of the last generation preview, or none before the first one.",
    21: "the LOD tier, the preview shading mode, the flow-graph canvas camera, the 3d preview camera, the sun and the selected generation.",
    22: "the flow evaluation of the current document the preview window renders, or none before the first evaluation.",
    23: "the 3d preview camera and shading mode this viewer broadcasts to peers.",
    24: "the LOD tier, the preview shading mode, the 3d preview camera, the sun and which bundled example the read-only surface shows.",
    25: "nothing yet: the presentation editor broadcasts no app-specific presence, so the record is empty.",
    26: "the engagement input text of the presentation editor.",
    27: "the editor selection, the lint generation counter and the engagement input of the main window.",
    28: "the main window's camera and editor settings.",
    29: "the in-progress node drag on the canvas (node, start and last pointer position, zoom).",
    30: "the canvas window's camera.",
    31: "the graph camera (x, y, zoom) this collaborator broadcasts to peers.",
    32: "the graph camera (x, y, zoom).",
    33: "the brush preview under the pointer in the 3d world window, or none.",
}
LANE_CANDIDATES = {
    "config": [
        ("electron-store", "npm", "A persisted key-value store for desktop settings; it has no model of this surface's typed config record, its per-kind setters, their no-op guard or the inverse each kind computes."),
        ("confy", "crates.io", "Loads and stores a whole configuration struct on disk; it defines no typed config mutations and neither their no-op nor their inverse behaviour."),
    ],
    "presence": [
        ("y-protocols (awareness)", "npm", "Broadcasts an opaque per-client awareness object; it has no typed presence record, no replace-presence mutation and no inverse to compare against."),
        ("liveblocks (presence)", "npm", "Hosted per-user presence objects merged by key; it neither models this surface's presence record nor defines a mutation vocabulary with inverses, and it is a cloud service this local-first repository does not depend on."),
    ],
    "transient": [
        ("zustand", "npm", "An in-memory state container with arbitrary setter functions; it has no typed transient record, no per-kind mutation vocabulary and no computed inverse."),
        ("im", "crates.io", "Persistent immutable collections; they version values structurally but define no mutation kinds, no-op guards or inverses over this surface's record."),
    ],
}
survey = json.load(open(root + ".tmp-ticket/wp-t12/generated/editor-survey.json", encoding="utf-8"))
out = {}
for i, v in enumerate(survey):
    if i not in SUMMARY: continue
    lane = v["surface"].split("/")[-1].lstrip("🎚️👥️🫧️")
    lane = {"config": "config", "presence": "presence", "transient": "transient"}[[k for k in ("config", "presence", "transient") if k in v["surface"].split("/")[-1]][0]]
    kinds = ", ".join(f"`{l['kind']}`" for l in v["leaves"])
    out[str(i)] = {
        "summary": f"it holds {SUMMARY[i]}",
        "rationale": f"`{v['aggregate']}` is the {lane} state lane of `{v['artifact']}`'s {v['surface']} surface: it holds {SUMMARY[i]} This is this repository's own editor state record, not a published format, so no third party implements it. What is under test is that each kind ({kinds}) changes exactly its own part of `{v['snapshot']}`, that a kind declaring `no-op` leaves a record that already holds its value untouched with a warned `mutation.no-op`, and that every kind's computed inverse, read off the pre-mutation record, restores it exactly. Confidence comes from one committed, handcrafted (before, mutation, after, diff, outcome) vector per kind and declared outcome class under `../🧫️fixtures/`, exercised end to end through the production dispatch bridge, and from the inverse law as a metamorphic property.",
        "ecosystems": ["npm", "crates.io"],
        "candidates": [{"package": p, "ecosystem": e, "verdict": "cannot-express-the-mutation", "reason": r} for p, e, r in LANE_CANDIDATES[lane]],
        "whyNone": f"`{v['snapshot']}` is a state record defined by this repository with its own typed mutation vocabulary; generic state containers can hold its values but cannot adjudicate what each kind must change, when it is a no-op, or what its inverse is.",
    }
json.dump(out, open(root + ".tmp-ticket/wp-t12/editor-decisions.json", "w"), ensure_ascii=False, indent=1)
print(len(out))
