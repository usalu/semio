"""📋 C4P28 spec writer: emits the modsplit/fragsplit specs of the seven files of package 28 into c4p28-tools/specs/<n>.json (line numbers refer to the pre-edit snapshots 🗑️generated/c4p28-orig/<n>.rs)."""
import json, os
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
B = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
S = "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any"
STAGE = F + "/🗑️generated/c4p28-stage"
OUT = os.path.dirname(os.path.abspath(__file__)) + "/specs"
V = "️"


def folder(emoji, name):
    return emoji + V + name


def seg(a, b, before=None, after=None):
    s = dict(lines=[a, b])
    if before:
        s["before"] = before
    if after:
        s["after"] = after
    return s


def chunk(emoji, name, doc, segments, **kw):
    snake = name.replace("-", "_")
    return dict(name=snake, folder=folder(emoji, name), doc=f"{emoji}{V} {doc}", segments=segments, **kw)


def spec(n, file, descendants, chunks):
    return dict(file=file, original=f"{F}/🗑️generated/c4p28-orig/{n}.rs", stage=STAGE, descendants=descendants, chunks=chunks)


def frag(n, file, chunks):
    return dict(file=file, original=f"{F}/🗑️generated/c4p28-orig/{n}.rs", stage=STAGE, chunks=chunks)


def fchunk(emoji, name, doc, ranges, **kw):
    f = folder(emoji, name)
    return dict(name=f, folder=f, doc=f"{emoji}{V} {doc}", ranges=ranges, **kw)


SPECS = {}

SPECS[1] = frag(1, f"{P}/✏️editor/🧪️tests/🔬️unit/🦀️.rs", [
    fchunk("🕹", "roster-and-read-laws", "Roster and read laws: the dispatchable action roster, the paged local interaction reads under render pressure and the interactive step budget of every maintenance unit.", [[119, 333]]),
    fchunk("🧵", "cursorized-route-laws", "Cursorized route laws: the exact retained publication contracts and the static laws that keep the suggestion, transform, focus, inspector, attraction and active-example routes cursorized.", [[334, 572]]),
    fchunk("🧮", "extent-and-step-laws", "Extent and step laws: the Nakagin-scale extents and step loops of world relocation, attraction creation, suggestion acceptance, inspector patches and the active example switch.", [[574, 863]]),
    fchunk("🎛", "dispatch-and-operation-laws", "Dispatch and operation laws: gumball gestures, the reserved refresh sections, the remaining cursorized routes, the operations on the concrete forest and the command envelope round trip.", [[865, 1272]]),
    fchunk("🧾", "inspector-and-manifest-laws", "Inspector and manifest laws: the selected object and vortex inspector, the patch field actions, the declared windows, dialogs, panel tabs, bridged actions and the German labels.", [[1274, 1658]]),
    fchunk("💡", "suggestion-and-scene-laws", "Suggestion and scene laws: the context menu, the vortex suggestion popup, the paged world scene carrier, the per-window utilities and the read-only brush search.", [[1660, 2241]]),
])

SPECS[2] = frag(2, f"{P}/✏️editor/🧪️tests/🧰️context/🦀️.rs", [
    fchunk("🏗", "raw-app-fixture", "Raw app fixture: the raw app, the live-cleanup grant, the maintenance stage grants, `Puzzle3dApp` and its close witness, the manifest for tests and the host turn drivers.", [[6, 363]]),
    fchunk("🎛", "settle-and-dispatch-drivers", "Settle and dispatch drivers: the settled outcome, the segmented download drain, the history readers and every dispatch, selection and hover driver.", [[365, 661]]),
    fchunk("🖼", "render-and-scene-probes", "Render and scene probes: the body, panel and window render drivers, the world scene carrier census and the scene field probes.", [[663, 924]]),
    fchunk("📏", "measure-and-tool-run-probes", "Measure and tool run probes: the window measure finders, the fill run and brush suggestion pumps, the context menu drivers, the heap witness and the editor source reader.", [[926, 1190]]),
])

SPECS[3] = spec(3, f"{P}/✏️editor/🧵️retained/🎬️action/🦀️.rs", [], [
    chunk("🪟", "window-command-work", "Window command work: the one-action-per-window route through the shared action prologue, spending one bounded turn on the scene, one per owed mesh fallback and one on the dispatch.", [seg(99, 273)], replace=[
        ("    pub(super) app_instance_id: u32,", "    pub app_instance_id: u32,"),
        ("    pub(super) parent_document_id: String,", "    pub parent_document_id: String,"),
        ("    pub(super) owner: ArtifactInstanceOperationOwnerHandle,", "    pub owner: ArtifactInstanceOperationOwnerHandle,"),
        ("    pub(super) fn new(tool_id: &'static str) -> Self {", "    pub fn new(tool_id: &'static str) -> Self {"),
        ("    fn with_tool_run(mut self,", "    pub fn with_tool_run(mut self,"),
        ("    pub(super) fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {", "    pub fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {"),
    ]),
    chunk("🔄", "transform-work", "Transform work: the staged selection transform (gumball, target volume relocation, Relocate drop, inspector nudge) that reads the gesture, scans a drop page by page and commits one transform tool transaction.", [seg(275, 464)], replace=[
        ("    pub(super) fn new(tool_id: &'static str, authoring_seed: String) -> Self {", "    pub fn new(tool_id: &'static str, authoring_seed: String) -> Self {"),
    ]),
    chunk("🖌", "brush-object-work", "Brush object work: the staged placement of one brush-previewed object, its kind, representation and vortices, the attractions it joins and the published object and attraction.", [seg(466, 747)]),
    chunk("🎯", "focus-selection-work", "Focus selection work: the staged focus that sums and measures the selected objects page by page before it publishes the result.", [seg(749, 919)]),
    chunk("💡", "accept-suggestion-work", "Accept suggestion work: the staged acceptance of one suggestion, its target, candidate, vortices and existing attractions, then the published object, attraction and result.", [seg(921, 1204)], replace=[
        ("    pub(super) fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {", "    pub fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {"),
    ]),
    chunk("🧠", "precompute-command-work", "Precompute command work: the staged precompute commands that decode and page the scene, its catalogs and the mesh positions and indices into the engine session before they publish.", [seg(1206, 1493)], replace=[
        ("    pub(super) fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {", "    pub fn bound(mut self, instance: &Puzzle3dInstanceBinding) -> Self {"),
        ("    pub(super) fn new(tool_id: &'static str) -> Self {", "    pub fn new(tool_id: &'static str) -> Self {"),
    ]),
])

SPECS[4] = spec(4, f"{P}/✏️editor/🧵️retained/🗂️document/🦀️.rs", [], [
    chunk("⚖", "kind-weight-work", "Kind weight work: the staged edit of the brush kind weights, the catalog, its validation, the sum of the other weights and the rebuilt publication.", [seg(33, 277)], replace=[
        ("    pub(super) fn new(tool_id: &'static str) -> Self {", "    pub fn new(tool_id: &'static str) -> Self {"),
    ]),
    chunk("➕", "add-object-kind-work", "Add object kind work: the staged addition of one object kind, decoded and checked against the catalog, with its representation and vortex, then published.", [seg(279, 540)]),
    chunk("🔍", "patch-inspector-work", "Patch inspector work: the staged inspector patch that pages through the selected objects, vortices, attractions, references and target volumes.", [seg(542, 972)]),
    chunk("💞", "create-attraction-work", "Create attraction work: the staged creation of one attraction, its existing attractions, the two endpoints and their compatibility, with the relocation page size.", [seg(974, 1202)]),
    chunk("📚", "set-active-example-work", "Set active example work: the staged switch of the active example document in fixed steps and bounded chunks.", [seg(1204, 1536)]),
])

SD = f"{P}/🧬️schema"
SPECS[5] = spec(5, f"{SD}/🦀️.rs", [f"{SD}/🧪️tests/🔬️precompute-model/🦀️.rs", f"{SD}/📸️snapshot/🦀️.rs", f"{P}/🚪️io/💾️binary/📸️snapshot/🦀️.rs", f"{P}/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs", f"{P}/🚪️io/📝️text/📸️snapshot/🦀️.rs"], [
    chunk("🗿", "artifact-identity", "Artifact identity: the artifact state record and its conversions, the schema id, the dialect coordinate and the two artifact kinds.",
          [seg(13, 82), seg(872, 881), seg(1816, 1864)]),
    chunk("🧱", "precompute-catalogs", "Precompute catalogs: the host rules and weights, the kind catalogs, the scene wire projection and the object, vortex and attraction props the precompute session exchanges with its host.",
          [seg(95, 454, after=["//#endregion 🔖️PrecomputeModel"])]),
    chunk("🎬", "precompute-commands", "Precompute commands: the brush and fill readouts, the run stage, counter and reason vocabularies, the typed command envelope and the session errors.",
          [seg(456, 799, before=["//#region 🔖️PrecomputeModel"]), seg(801, 827), seg(838, 870)]),
    chunk("📚", "document-model", "Document model: the scale, the objects, vortices, attractions, target volumes, references, the kind catalogs and the document meta of the puzzle 3d artifact.",
          [seg(883, 1574, after=["// #endregion 🔖️Document"])]),
    chunk("📐", "pose-math", "Pose math: the vector and quaternion helpers, the vortex full id and the attraction pose derivation.", [seg(1576, 1814)]),
])

SPECS[6] = spec(6, f"{B}/✏️editor/🦀️.rs", [f"{B}/✏️editor/🧪️tests/🔬️unit/🦀️.rs", f"{B}/✏️editor/📌️panels/🗿️artifact/🦀️.rs", f"{B}/✏️editor/📌️panels/🔍️inspection/🦀️.rs"], [
    chunk("🧰", "retained-commands", "Retained commands: the tool id catalog, the publication contracts, the bounded reducer and the retained command job factory of the block 3d editor.", [seg(216, 433)]),
    chunk("📬", "store-preparation", "Store preparation: the one-item publication preparation of the Artifact lane and of the Config lane.", [seg(435, 646)]),
    chunk("🧩", "play-app", "Play app: `Block3dPlayApp`, the `ArtifactEditor` the framework mounts, with its dispatch, render and media bodies.", [seg(648, 980)]),
    chunk("📋", "manifest", "Manifest: `create_block3d_app` and the manifest stitch.", [seg(982, 1108)]),
])

SPECS[7] = spec(7, f"{S}/✏️editor/🦀️.rs", [f"{S}/✏️editor/🧪️tests/🔬️unit/🦀️.rs", f"{S}/✏️editor/📚️examples/🎬️demo-session/🦀️.rs", f"{S}/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"], [
    chunk("🧰", "retained-commands", "Retained commands: the tool id catalog, the bounded contract, the retained reducer, the bounded command job factory and the finish emit.", [seg(292, 462)]),
    chunk("📬", "config-store-preparation", "Config store preparation: the byte bounds, the admission and the one-item publication preparation of the Config lane.", [seg(464, 655)]),
    chunk("🗄", "artifact-store-preparation", "Artifact store preparation: the document bounds, the admission and the one-item publication preparation of the Artifact lane.", [seg(657, 811)]),
    chunk("🧩", "sourcing-app", "Sourcing app: the `ArtifactEditor` implementation of `SourcingCurationApp` with its dispatch, render, media and composition bodies.", [seg(815, 1034)]),
    chunk("📋", "manifest", "Manifest: the hidden operation and view action builders and `create_sourcing_curation_app`.", [seg(1053, 1232)]),
])

if __name__ == "__main__":
    os.makedirs(OUT, exist_ok=True)
    for n, s in SPECS.items():
        json.dump(s, open(f"{OUT}/{n}.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print("wrote", len(SPECS), "specs")
