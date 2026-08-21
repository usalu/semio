#!/usr/bin/env python3
"""Add the committed `🔺️diff/🔣️component.json` to all 45 layout+cad cases and append the three
diff assertions to each case's `🦀️component.rs`. Each diff is transcribed by hand from that
mutation's own `🔺️diff/🦀️component.rs`; each assertion is worded for that mutation."""
import copy, json, os, pathlib, re

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
LMUT = ROOT / "✏️s/\U0001f50c️plugins/\U0001f4cf️layout/\U0001f5ff️artifacts/\U0001f4cf️layout/\U0001f3c5️standards/\U0001f516️1/\U0001fa86️subsets/✳️any/\U0001f9ec️schema/\U0001f9ec️mutations"
CMUT = ROOT / "✏️s/\U0001f50c️plugins/\U0001f4d0️cad/\U0001f5ff️artifacts/\U0001f4d0️cad/\U0001f3c5️standards/\U0001f516️1/\U0001fa86️subsets/✳️any/\U0001f9ec️schema/\U0001f9ec️mutations"

# ── exact serde field order, extracted from the two 🔺️diff/🦀️component.rs schemas ──────────────
LAYOUT_DIFF_FIELDS = ['artifact', 'schema', 'name', 'grid', 'paragraphStyles', 'characterStyles',
    'stories', 'links', 'parentPages', 'spreads', 'pages', 'printTarget', 'dataFieldsJson',
    'backgroundDrawing', 'referencedModel', 'selectedIds', 'activePageId', 'engagementInput',
    'cameraX', 'cameraY', 'cameraZoom', 'previewCameraX', 'previewCameraY', 'previewCameraZoom',
    'dropPreview', 'locale', 'hoveredId']
CAD_DIFF_FIELDS = ['artifact', 'schema', 'id', 'shapeModel', 'buildingModel', 'energyModel',
    'structureClassicModel', 'drawings', 'referencesByModelDefinitionId', 'nodes',
    'activeModelDefinitionId', 'selectedObjectIds', 'selectedNodeIds', 'activeObjectId',
    'componentSelection', 'selectedReferenceModelDefinitionId', 'selectedReferenceId',
    'selectedPrimitiveId', 'selectedPrimitiveKind', 'activeUtilityId', 'activeExampleId',
    'selectionMethod', 'engagementInput', 'engagementStep', 'engagementPane',
    'engagementSessionJson', 'lastFinalizedInteractionId', 'sunEnabled', 'sunAzimuth',
    'sunElevation', 'sunIntensity', 'sunColor', 'camera', 'cameraBuilding', 'cameraEnergy',
    'cameraStructureClassic', 'dislocateShape', 'dislocateBuilding', 'dislocateEnergy',
    'dislocateStructureClassic', 'locale', 'terminology', 'contributionsJson', 'hoveredObjectId',
    'hoveredTargetObjectId', 'hoveredTargetMode', 'hoveredTargetId']

def skeleton(fields, **overrides):
    d = {f: None for f in fields}
    for k, v in overrides.items():
        assert k in d, k
        d[k] = v
    return d

def ldiff(**o): return skeleton(LAYOUT_DIFF_FIELDS, **o)
def cdiff(**o): return skeleton(CAD_DIFF_FIELDS, **o)

# ── nested defaults (every field emitted; the *Patch/*Added/*Patched records are snake_case) ─────
def delta(added=None, removed=None, patched=None, reordered=None):
    return {"added": added or [], "removed": removed or [], "patched": patched or [], "reordered": reordered}

PAGE_PATCH_EMPTY = {"name": None, "width": None, "height": None, "margin_top": None,
    "margin_right": None, "margin_bottom": None, "margin_left": None, "columns_count": None,
    "columns_gutter": None, "frame_added": None, "frame_removed": None, "frame_patched": None}
FRAME_PATCH_EMPTY = {"x": None, "y": None, "width": None, "height": None, "fill": None,
    "stroke": None, "wrap_mode": None, "columns": None}

def page_patch(**o):
    p = copy.deepcopy(PAGE_PATCH_EMPTY)
    for k, v in o.items():
        assert k in p, k
        p[k] = v
    return p

def frame_patch(**o):
    p = copy.deepcopy(FRAME_PATCH_EMPTY)
    for k, v in o.items():
        assert k in p, k
        p[k] = v
    return p

def page_entry(page_id, patch): return {"id": page_id, "patch": patch}

# ── the same records the ⬅️before/➡️after snapshots use ──────────────────────────────────────────
RECT = {"kind": "rect", "id": "frame-rect", "layerId": "layer-1",
    "bounds": {"x": 20.0, "y": 30.0, "w": 60.0, "h": 40.0, "rotation": 0.0},
    "locked": None, "visible": None, "fill": [1.0, 1.0, 1.0, 1.0], "stroke": None}
BADGE = {"kind": "rect", "id": "frame-badge", "layerId": "layer-1",
    "bounds": {"x": 120.0, "y": 30.0, "w": 40.0, "h": 40.0, "rotation": 0.0},
    "locked": None, "visible": None, "fill": [0.0, 0.5, 1.0, 1.0], "stroke": None}
PAGE_3 = {"id": "page-3", "name": "Back", "spreadId": "spread-2", "parentPageId": None,
    "width": 200.0, "height": 300.0,
    "margins": {"top": 5.0, "right": 5.0, "bottom": 5.0, "left": 5.0},
    "columns": {"count": 2, "gutter": 6.0}, "guides": [], "layerIds": ["layer-3"],
    "layers": [{"id": "layer-3", "name": "Content", "visible": True, "locked": False, "objectIds": []}],
    "frames": [], "overrides": []}
STORY_3 = {"id": "story-3", "content": "Caption.", "styleRuns": []}
LINK_3 = {"id": "link-3", "path": "caption.png", "hash": "hash-caption", "width": 200,
    "height": 150, "dpi": 144, "colorProfile": None, "state": None, "proxyDataUrl": None}

def child(child_id, artifact_id, subset):
    return {"childId": child_id, "target": {"artifactId": artifact_id,
        "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": subset}}}

def ref1(**o):
    r = {"id": "ref-1", "sourceUrl": "https://example.test/plan.png", "mediaKind": "image",
         "origin": [0.0, 0.0, 0.0], "orientation": None, "scale": 1.5, "widthWorld": 8.0,
         "hidden": False, "locked": True, "opacity": 0.5}
    r.update(o)
    return r
REF_2 = {"id": "ref-2", "sourceUrl": "https://example.test/site.png", "mediaKind": "image",
    "origin": [5.0, 0.0, 0.0], "orientation": None, "scale": None, "widthWorld": 16.0,
    "hidden": False, "locked": False, "opacity": None}
NODE_3 = {"id": "node-3", "label": "Column", "kind": "solid"}

# ── per-case: the committed diff + the wording of the three new assertions ───────────────────────
# fields: diff, note (doc line for produces_committed_diff), claim (assert message)
LAYOUT = {
 "rename-layout": dict(
   diff=ldiff(name="Renamed Fixture"),
   note="the ONLY populated field is the root `name` scalar — no collection delta at all",
   claim="rename-layout must emit a diff whose sole populated field is the root `name` scalar"),
 "change-print-target": dict(
   diff=ldiff(printTarget="cmyk-coated"),
   note="the ONLY populated field is `printTarget`, and it carries the string rather than the doubly-optional cleared arm",
   claim="change-print-target must emit a diff whose sole populated field is `printTarget`"),
 "change-data-fields": dict(
   diff=ldiff(dataFieldsJson="{\"client\":\"acme\"}"),
   note="the ONLY populated field is `dataFieldsJson`, carrying the opaque blob verbatim",
   claim="change-data-fields must emit a diff whose sole populated field is `dataFieldsJson`"),
 "create-page": dict(
   diff=ldiff(pages=delta(added=[PAGE_3])),
   note="only `pages.added` is populated — the whole `Page` record travels in the delta, and `removed`/`patched`/`reordered` stay empty",
   claim="create-page must emit a pages delta whose only populated arm is `added`"),
 "delete-page": dict(
   diff=ldiff(pages=delta(removed=["page-2"])),
   note="only `pages.removed` is populated, and it carries the bare id — the removed record itself lives in the INVERSE, never in the forward diff",
   claim="delete-page must emit a pages delta whose only populated arm is `removed`, carrying the bare id"),
 "rename-page": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(name="Title Page"))])),
   note="only `pages.patched[0].patch.name` is populated — every geometry field of the page patch stays null",
   claim="rename-page must emit a page patch in which only `name` is populated"),
 "change-page-width": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(width=240.0))])),
   note="only `pages.patched[0].patch.width` is populated — `height` stays null, proving the single-axis contract at the diff level",
   claim="change-page-width must emit a page patch in which only `width` is populated"),
 "change-page-height": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(height=360.0))])),
   note="only `pages.patched[0].patch.height` is populated — `width` stays null",
   claim="change-page-height must emit a page patch in which only `height` is populated"),
 "update-page-margins": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(margin_top=12.0, margin_right=18.0, margin_bottom=24.0, margin_left=6.0))])),
   note="all four `margin_*` fields are populated together and the two `columns_*` fields stay null — the atomic-facet boundary, visible in the diff",
   claim="update-page-margins must emit a page patch populating all four margin fields and no column field"),
 "update-page-columns": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(columns_count=3, columns_gutter=12.0))])),
   note="both `columns_*` fields are populated together and every `margin_*` field stays null",
   claim="update-page-columns must emit a page patch populating both column fields and no margin field"),
 "reorder-pages": dict(
   diff=ldiff(pages=delta(reordered=["page-2", "page-1"])),
   note="only `pages.reordered` is populated, and it is the COMPLETE final id order — never a from/to index pair",
   claim="reorder-pages must emit a pages delta whose only populated arm is `reordered`, holding the complete final order"),
 "create-story": dict(
   diff=ldiff(stories=delta(added=[STORY_3])),
   note="only the `stories` delta is populated — `pages` and `links` stay null, so no frame is rethreaded",
   claim="create-story must emit a stories delta only, leaving the pages and links deltas null"),
 "delete-story": dict(
   diff=ldiff(stories=delta(removed=["story-2"])),
   note="only `stories.removed` is populated — the absence of a `pages` delta is what proves there is no cascade into the text frame",
   claim="delete-story must emit a stories delta only, proving no cascade into the frames that thread stories"),
 "edit-story": dict(
   diff=ldiff(stories=delta(patched=[{"id": "story-1", "patch": {"content": "Alpha body, revised."}}])),
   note="only `stories.patched[0].patch.content` is populated — `TextStoryPatch` cannot express a style-run edit at all",
   claim="edit-story must emit a story patch carrying the replacement body and nothing else"),
 "create-link": dict(
   diff=ldiff(links=delta(added=[LINK_3])),
   note="only the `links` delta is populated, carrying the whole `ImageLink` record including its hash and dpi",
   claim="create-link must emit a links delta whose only populated arm is `added`"),
 "delete-link": dict(
   diff=ldiff(links=delta(removed=["link-2"])),
   note="only `links.removed` is populated — the absent `pages` delta is what proves there is no cascade into image frames",
   claim="delete-link must emit a links delta only, proving no cascade into the frames that reference links"),
 "change-link-path": dict(
   diff=ldiff(links=delta(patched=[{"id": "link-1", "patch": {"path": "alpha-v2.png"}}])),
   note="`ImageLinkPatch` has exactly one field, so the diff structurally CANNOT re-derive the hash or pixel size",
   claim="change-link-path must emit a link patch carrying the new path and nothing else"),
 "create-frame": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_added={"frame": BADGE, "index": 1, "layer_id": "layer-1"}))])),
   note="the frame insert rides inside `pages.patched[0].patch.frame_added` — a NESTED page patch, never a top-level frames collection",
   claim="create-frame must emit the insert as a nested `frame_added` fragment of a page patch"),
 "delete-frame": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_removed="frame-text"))])),
   note="`frame_removed` carries the bare frame id; the layer-membership cascade is apply-side behaviour, not something the diff spells out",
   claim="delete-frame must emit a nested `frame_removed` fragment carrying the bare frame id"),
 "move-frame": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_patched={"frame_id": "frame-rect", "patch": frame_patch(x=55.0, y=65.0)}))])),
   note="the nested `FramePatch` populates only `x`/`y`; `width`/`height`/`rotation`-bearing fields stay null",
   claim="move-frame must emit a nested frame patch populating only x and y"),
 "resize-frame": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_patched={"frame_id": "frame-rect", "patch": frame_patch(width=90.0, height=70.0)}))])),
   note="the nested `FramePatch` populates only `width`/`height`; `x`/`y` stay null, so the origin cannot drift",
   claim="resize-frame must emit a nested frame patch populating only width and height"),
 "change-frame-fill": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_patched={"frame_id": "frame-rect", "patch": frame_patch(fill=[0.5, 0.25, 0.75, 1.0])}))])),
   note="the doubly-optional `fill` serializes to the bare RGBA array (outer Some, inner Some); `stroke` stays null",
   claim="change-frame-fill must emit a nested frame patch populating only fill"),
 "change-frame-stroke": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_patched={"frame_id": "frame-rect", "patch": frame_patch(stroke=[0.0, 0.0, 0.0, 1.0])}))])),
   note="the doubly-optional `stroke` serializes to the bare RGBA array; `fill` stays null",
   claim="change-frame-stroke must emit a nested frame patch populating only stroke"),
 "change-frame-wrap-mode": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_patched={"frame_id": "frame-text", "patch": frame_patch(wrap_mode="column")}))])),
   note="only `wrap_mode` is populated — the SAME `FramePatch` type also carries `columns`, and the diff proves this verb does not touch it",
   claim="change-frame-wrap-mode must emit a nested frame patch populating only wrap_mode"),
 "change-frame-columns": dict(
   diff=ldiff(pages=delta(patched=[page_entry("page-1", page_patch(frame_patched={"frame_id": "frame-text", "patch": frame_patch(columns=2)}))])),
   note="only `columns` is populated — `wrap_mode` stays null in the same shared `FramePatch`",
   claim="change-frame-columns must emit a nested frame patch populating only columns"),
}

SLOT_META = {
 "shape": ("shapeModel", "shape-model-2", "cad-shape-2", "shape"),
 "building": ("buildingModel", "building-model-2", "cad-building-2", "building"),
 "energy": ("energyModel", "energy-model-2", "cad-energy-2", "energy"),
 "structure-classic": ("structureClassicModel", "structure-classic-model-2", "cad-structure-2", "structure-classic"),
}

CAD = {}
for pane, (field, new_child, new_artifact, _label) in SLOT_META.items():
    CAD[f"create-{pane}-model"] = dict(
        diff=cdiff(**{field: child(new_child, new_artifact, "model")}),
        note=f"the ONLY populated field is `{field}`, carrying the occupied arm — the other three fixed slots stay null even though this create overwrites one of them",
        claim=f"create-{pane}-model must emit a diff whose sole populated field is `{field}`",
        vacates=False)
    CAD[f"delete-{pane}-model"] = dict(
        diff=cdiff(**{field: None}),
        note=f"`{field}` is set to the VACATED arm (`Some(None)` in memory)",
        claim=f"delete-{pane}-model must emit a diff whose sole populated field is `{field}`",
        vacates=True, field=field, pane=pane)

CAD.update({
 "create-drawing": dict(
   diff=cdiff(drawings={"values": [child("drawing-1", "cad-drawing-1", "drawing"), child("drawing-2", "cad-drawing-2", "drawing")]}),
   note="`drawings` carries the WHOLE post-state handle list (the existing handle plus the new one) — unlike `nodes` there is no added/removed delta for this composition slot",
   claim="create-drawing must emit the whole post-state drawings list, not an added-only delta", vacates=False),
 "delete-drawing": dict(
   diff=cdiff(drawings={"values": []}),
   note="`drawings` carries the WHOLE post-state list, empty here — the removed id never appears in the diff",
   claim="delete-drawing must emit the whole post-state drawings list rather than a removed-id delta", vacates=False),
 "create-node": dict(
   diff=cdiff(nodes=delta(added=[NODE_3])),
   note="only `nodes.added` is populated — the node tree IS an added/removed/patched delta, unlike `drawings`",
   claim="create-node must emit a nodes delta whose only populated arm is `added`", vacates=False),
 "delete-node": dict(
   diff=cdiff(nodes=delta(removed=["node-2"])),
   note="only `nodes.removed` is populated, carrying the bare id — the removed record lives in the INVERSE",
   claim="delete-node must emit a nodes delta whose only populated arm is `removed`", vacates=False),
 "rename-node": dict(
   diff=cdiff(nodes=delta(patched=[{"id": "node-1", "patch": {"label": "Assembly Root"}}])),
   note="`CadNodePatch` has exactly one field, so the diff structurally CANNOT retype the node",
   claim="rename-node must emit a node patch carrying the new label and nothing else", vacates=False),
 "change-reference-hidden": dict(
   diff=cdiff(referencesByModelDefinitionId={"spatial.shape": [ref1(hidden=True)]}),
   note="the diff carries the WHOLE post-patch row for ONE model-definition bucket — apply merges per key, so the other buckets are untouched by omission",
   claim="change-reference-hidden must emit exactly one bucket, holding the whole post-patch reference row", vacates=False),
 "change-reference-locked": dict(
   diff=cdiff(referencesByModelDefinitionId={"spatial.shape": [ref1(locked=False)]}),
   note="the emitted row differs from BASE in `locked` alone — every other field of the reference is reproduced verbatim",
   claim="change-reference-locked must emit one bucket whose row differs from BASE in `locked` alone", vacates=False),
 "change-reference-width": dict(
   diff=cdiff(referencesByModelDefinitionId={"spatial.shape": [ref1(widthWorld=12.0)]}),
   note="the emitted row differs from BASE in `widthWorld` alone — the uniform `scale` factor is reproduced verbatim",
   claim="change-reference-width must emit one bucket whose row differs from BASE in `widthWorld` alone", vacates=False),
 "move-reference": dict(
   diff=cdiff(referencesByModelDefinitionId={"spatial.shape": [ref1(origin=[1.0, 2.0, 3.0])]}),
   note="the emitted row differs from BASE in `origin` alone",
   claim="move-reference must emit one bucket whose row differs from BASE in `origin` alone", vacates=False),
 "replace-reference-media": dict(
   diff=cdiff(referencesByModelDefinitionId={"spatial.shape": [ref1(sourceUrl="https://example.test/plan-v2.png", mediaKind="drawing", scale=2.0, opacity=0.25)]}),
   note="the emitted row shows `orientation` still null — the patch's `orientation: None` means \"unchanged\", so the media bundle can never CLEAR an orientation",
   claim="replace-reference-media must emit one bucket whose row keeps the placement fields and the untouched orientation", vacates=False),
 "replace-references": dict(
   diff=cdiff(referencesByModelDefinitionId={"spatial.shape": [REF_2]}),
   note="the emitted bucket IS the payload list verbatim — `ref-1` appears nowhere, which is exactly how a wholesale substitution differs from a merge",
   claim="replace-references must emit the payload list verbatim as the bucket's new value", vacates=False),
 "change-active-model-definition": dict(
   diff=cdiff(activeModelDefinitionId="aec.building"),
   note="the ONLY populated field is the root selector string — no bucket, slot or node rides along",
   claim="change-active-model-definition must emit a diff whose sole populated field is `activeModelDefinitionId`", vacates=False),
})

# ── the three appended assertions ────────────────────────────────────────────────────────────────
TAIL = '''
/// \U0001f53a️ The sparse delta `{kind}` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here {note}.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{kind}/{case}: {claim}");
}}

/// \U0001f523️ The committed diff decodes into `{difftype}` and re-encodes byte-for-byte: `{difftype}` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    let decoded: {diffpath} = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{kind}/{case}: committed diff JSON is not canonical");
}}

{third}'''

THIRD_NORMAL = '''/// \U0001fa79 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `{kind}` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: {diffpath} = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{kind}/{case}: committed diff did not carry before to after");
}}
'''

THIRD_VACATE = '''/// \U0001fa79 Applying the committed diff to `before`. ⚠️ `CadDiff::{field}` is
/// `Option<Option<CadModelChild>>` with a plain serde derive: `Some(None)` ("vacate the slot") and
/// `None` ("leave the slot alone") BOTH serialize to `null`, and JSON `null` decodes back to the
/// OUTER `None`. So `delete-{pane}-model`'s vacate intent does not survive a JSON round trip, and the
/// decoded diff is inert. This test pins that hole rather than papering over it: the in-memory diff
/// DOES carry `before` to `after`; the JSON-decoded one is indistinguishable from an empty diff.
/// See `📓️census/📓️fixtures-layout-cad.md`. Fixing the wire shape (a `double_option` helper, or
/// `skip_serializing_if` so an untouched slot is OMITTED rather than `null`) must flip this test to
/// the plain `assert_eq!(produced, expected_after())` every other case uses.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let base = before();
    let in_memory = mutation().diff(&base).diff().clone();
    assert_eq!(in_memory.apply(&base).expect("the in-memory diff applies"), expected_after(), "{kind}/{case}: the in-memory diff must carry before to after");

    let decoded: {diffpath} = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    assert_eq!(decoded, {diffpath}::default(), "{kind}/{case}: a `null` {field} is indistinguishable from an untouched one, so the decoded diff is empty");
    assert_eq!(decoded.apply(&base).expect("the decoded diff applies"), base, "{kind}/{case}: the JSON-decoded diff is inert — the vacate intent is lost on the wire");
}}
'''


def dump(path, obj):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def process(mut_root, table, difftype, diffpath):
    leaves = {}
    for entry in sorted(os.listdir(mut_root)):
        if (mut_root / entry).is_dir():
            leaves[re.sub(r"^[^a-z]*", "", entry)] = entry
    done = []
    for slug, spec in table.items():
        leaf = mut_root / leaves[slug]
        tests = leaf / "\U0001f9ea️tests"
        cases = [c for c in sorted(os.listdir(tests)) if (tests / c).is_dir()]
        assert len(cases) == 1, (slug, cases)
        case = cases[0]
        case_dir = tests / case
        dump(case_dir / "\U0001f53a️diff/\U0001f523️component.json", spec["diff"])

        rs_path = case_dir / "\U0001f980️component.rs"
        src = rs_path.read_text(encoding="utf-8")
        # 1. DIFF const, right after the MUTATION const (matching puzzle5d's ordering)
        if "const DIFF:" not in src:
            src = src.replace(
                'const MUTATION: &str = include_str!("\U0001f9a0️mutation/\U0001f523️component.json");\n',
                'const MUTATION: &str = include_str!("\U0001f9a0️mutation/\U0001f523️component.json");\n'
                'const DIFF: &str = include_str!("\U0001f53a️diff/\U0001f523️component.json");\n', 1)
        # 3. the three assertions
        if "async fn produces_committed_diff()" not in src:
            third = (THIRD_VACATE if spec.get("vacates") else THIRD_NORMAL).format(
                kind=slug, case=case, diffpath=diffpath,
                field=spec.get("field", ""), pane=spec.get("pane", ""))
            src = src.rstrip("\n") + "\n" + TAIL.format(
                kind=slug, case=case, note=spec["note"], claim=spec["claim"],
                difftype=difftype, diffpath=diffpath, third=third)
        rs_path.write_text(src, encoding="utf-8")
        done.append((leaves[slug], case))
    return done


l = process(LMUT, LAYOUT, "LayoutDiff", "crate::artifacts::layout::LayoutDiff")
c = process(CMUT, CAD, "CadDiff", "crate::artifacts::cad::diff::CadDiff")
print(f"layout cases updated: {len(l)}")
print(f"cad cases updated:    {len(c)}")
