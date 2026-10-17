"""📋 C4P5 spec writer: emits the modsplit/fragsplit specs of the seven files of package 5 into c4p5-tools/specs/<n>.json (line numbers refer to the pre-edit snapshots 🗑️generated/c4p5-orig/<n>.rs)."""
import json, os
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
E = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
STAGE = F + "/🗑️generated/c4p5-stage"
OUT = os.path.dirname(os.path.abspath(__file__)) + "/specs"


def reg(name):
    return dict(before=[f"//#region {name}"], after=[f"//#endregion {name}"])


def seg(a, b, region=None):
    s = dict(lines=[a, b])
    if region:
        s.update(reg(region))
    return s


def chunk(name, folder, doc, segments, **kw):
    return dict(name=name, folder=folder, doc=doc, segments=segments, **kw)


def spec(n, file, descendants, chunks):
    return dict(file=file, original=f"{F}/🗑️generated/c4p5-orig/{n}.rs", stage=STAGE, descendants=descendants, chunks=chunks)


SPECS = {}

SPECS[1] = spec(1, f"{E}/⏳️precompute/🦀️.rs", [f"{E}/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs"], [
    chunk("brush_meshes", "🥽️brush-meshes", "🥽️ Brush meshes: the process-wide content-addressed derived-mesh store (the `puzzle3d.mesh-decode` compute engine) and the paged `registerBrushMesh` upload staging.",
          [seg(72, 207, "🥽️SharedBrushMeshes"), seg(211, 406, "🧩️PagedBrushMeshUploads")]),
    chunk("collision_engine", "⚙️collision-engine", "⚙️ Collision engine: the step clock, the scene invalidation plan, the brush broad phase and narrow phase over the registered meshes, and one document instance's collision session.",
          [seg(44, 50), seg(53, 68, "🔖️Clock"), seg(410, 1221, "🔖️Engine")], mountAt=410),
    chunk("precompute_session", "🧵️precompute-session", "🧵️ Precompute session: the scene the host syncs in and `dispatch`, which drives `Puzzle3dEngineCommand` and `Puzzle3dEngineOutcome` through the collision engine.",
          [seg(1225, 1362, "🔖️Session")]),
])

SPECS[2] = spec(2, f"{E}/⏳️precompute/🪣️fill/🦀️.rs", [], [
    chunk("fill_contract", "🛑️fill-contract", "🛑️ Fill contract: why a plan stops, the step context a planner transition runs under, the run events and the placed collision entry.", [seg(30, 115)]),
    chunk("weighted_draw", "🎲️weighted-draw", "🎲️ Weighted draw: the Fenwick tree over vortex weights and the weighted pick and draw that consume the per-run random stream.", [seg(117, 197)]),
    chunk("builder_state", "🏗️builder-state", "🏗️ Builder state: the job stages and phases, the fixed owners, the scene and catalog views, the preparation capacity refusal, `FillBuilder` and its retained brush weights.", [seg(199, 510)]),
    chunk("retirement", "♻️retirement", "♻️ Retirement: the incremental release of every owner a `FillBuilder` still holds when it closes, one bounded unit per call.", [seg(512, 955)]),
    chunk("builder_preparation", "🧱️builder-preparation", "🧱️ Builder preparation: `begin_preparation`, the close accounting and the cooperative preparation transitions that install the scene, catalogs, meshes, entries, spatial index and lookups.", [seg(957, 1573)]),
    chunk("interactive_fill", "🧵️interactive-fill", "🧵️ Interactive fill: target and candidate selection, the broad and narrow phase, acceptance, the resets and the planner transition under any step context.", [seg(1576, 2377, "🧵️InteractiveFillJob")]),
    chunk("fill_run", "⏯️fill-run", "⏯️ Fill run: the tool run job that steps the planner, publishes every verdict as a trace upsert and keeps accepted placements as provisional ops.", [seg(2382, 3061, "⏯️FillRunJob")]),
    chunk("fill_revalidation", "🔍️fill-revalidation", "🔍️ Fill revalidation: the finalize job that re-checks the provisional placements against the committed document and re-appends the survivors.", [seg(3063, 3411, "⏯️FillRunJob")]),
])

SPECS[4] = spec(4, f"{E}/🎟️session/🦀️.rs", [], [
    chunk("engine_bridge", "🌉️engine-bridge", "🌉️ Engine bridge: the typed translation of the transient scene into the precompute `SceneConfig` and the sync and drive calls that feed one precompute session.", [seg(38, 205)]),
    chunk("action_context", "🎬️action-context", "🎬️ Action context: everything one command arm may read or write while the dispatch prologue and epilogue stay in the dispatcher.", [seg(207, 379)]),
    chunk("context_menu", "🖱️context-menu", "🖱️ Context menu: the localized row builder, the surface selection and the object, vortex, volume and reference menu items.", [seg(381, 561)]),
    chunk("session_registry", "🗃️session-registry", "🗃️ Session registry: the process-wide slots that keep one document instance's session across calls, the instance operation owner and the session itself.", [seg(568, 900)]),
    chunk("instance_residency", "🏘️instance-residency", "🏘️ Instance residency: the per-object instance record text, its fingerprints and delta, the scene geometry fingerprint and the world mesh lane.", [seg(905, 1283)]),
])

SPECS[5] = spec(5, f"{E}/🦀️.rs", [f"{E}/🌉️wasm/🦀️.rs", f"{E}/📚️examples/🎬️demo-session/🦀️.rs"], [
    chunk("retained_commands", "🧰️retained-commands", "🧰️ Retained commands: the tool id catalog, the retained command job factory with its publication contracts and the bounded first-step proofs.",
          [seg(133, 327, "🧵️RetainedCommands"), seg(666, 706, "📜️ToolProofs")]),
    chunk("store_preparation", "📬️store-preparation", "📬️ Store preparation: the one-item publication preparation of the Config lane and of the Artifact lane.", [seg(331, 662, "📬️StorePreparation")]),
    chunk("clipboard_job", "✂️clipboard-job", "✂️ Clipboard job: the one-step reserved copy, cut and paste job and the cut operations.", [seg(709, 851), seg(1916, 1925)]),
    chunk("play_app", "🧩️play-app", "🧩️ Play app: `Puzzle3dPlayApp`, the `ArtifactEditor` the framework mounts, and its context menu, render and window measure bodies.", [seg(853, 1364), seg(1832, 1911)]),
    chunk("manifest", "📋️manifest", "📋️ Manifest: the typed media I/O, the interaction definition, the shipped object kinds and `create_puzzle3d_app`.", [seg(1369, 1827, "🔖️Manifest")]),
])


def frag(n, file, chunks):
    return dict(file=file, original=f"{F}/🗑️generated/c4p5-orig/{n}.rs", stage=STAGE, chunks=chunks)


def fchunk(folder, doc, ranges, **kw):
    return dict(name=folder, folder=folder, doc=doc, ranges=ranges, **kw)


CD = "🩹️CheckedDefects"
READ = '&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads")'
SPECS[3] = frag(3, f"{E}/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs", [
    fchunk("🧮️planner-laws", "🧮️ Planner laws: the constructor cap and plus-one handback, the fixed collection pages, the cooperative transitions, the work and watchdog ceilings, the capacity law and the Nakagin-scale plan.", [[69, 704]]),
    fchunk("⏯️fill-run-laws", "⏯️ Fill run laws: the language-neutral run fixture, the parry3d oracle, the trace delivery, the interactive ceilings, resume and checkpoint replay, the capacity danger step, revalidation and the own-mesh bodies.", [[706, 1644]]),
    fchunk("🛑️run-end-laws", "🛑️ Run end laws: rule refusals as warnings and the declared visible end of every case and own-mesh variant.", [[1646, 1745]]),
])

SPECS[6] = frag(6, f"{E}/🧪️tests/🔬️budgets/🦀️.rs", [
    fchunk("🎟️session-and-step-budget-laws", "🎟️ Session and step budget laws: the session geometry cache, the worker hop, the stale lease, the process byte ceiling and the interactive step budgets of the retained commands.", [[111, 493]]),
    fchunk("🖱️menu-and-engagement-laws", "🖱️ Menu and engagement laws: the context menu rows, the Add Object dialog, the engagement verbs, the fill and brush arming and the refused placement notice.", [[495, 834]], post=[f"//#endregion {CD}"]),
    fchunk("🗂️outliner-and-relocate-laws", "🗂️ Outliner and relocate laws: the inspection flag rows, the outliner verbs, the target volume and object relocation undo and redo.", [[835, 1135]], pre=[f"//#region {CD}"], post=[f"//#endregion {CD}"]),
    fchunk("📥️snapshot-exchange-laws", "📥️ Snapshot exchange laws: copy, cut, export and import of the document, the segmented download budget and the leftover routes.", [[1136, 1673]], pre=[f"//#region {CD}"], post=[f"//#endregion {CD}"], replace=[[READ, "&editor_source()"]]),
    fchunk("🧲️leftover-selection-laws", "🧲️ Leftover selection laws: the object context menu, the locked volume scale refusal, the clipboard clones and the leftover translate routes.", [[1675, 1848]], pre=[f"//#region {CD}"]),
    fchunk("📷️camera-and-settings-laws", "📷️ Camera and settings laws: the brush mesh page budget, the id-only announcement, the opening camera and the settings panel scope.", [[1850, 2135]]),
    fchunk("🔬️document-laws", "🔬️ Document laws: the rendered versus the mutated document, the shipped kind catalog, the initial snapshot cost and the host twin.", [[2137, 2248]]),
])

SPECS[7] = frag(7, f"{E}/🧪️tests/🔬️panels/🦀️.rs", [
    fchunk("🪟️window-option-laws", "🪟️ Window option laws: grid, level of detail and vortex options per window instance, the settings steppers, the camera actions and the worker session admission.", [[112, 539]]),
    fchunk("🪣️fill-and-distribution-laws", "🪣️ Fill and distribution laws: the fill tool run, the fill count and the kind weight distribution sliders.", [[541, 1017]]),
    fchunk("🎯️scope-and-utility-laws", "🎯️ Scope and utility laws: the UI dirty scopes of commands and interaction verbs and the add, target volume and transform utilities.", [[1019, 1398]]),
    fchunk("🖱️world-selection-laws", "🖱️ World selection laws: pick, vortex and same-kind selection, the scoped command refusals and the selection after add, duplicate and delete.", [[1400, 1782]]),
    fchunk("🎛️gumball-laws", "🎛️ Gumball laws: the gumball arming, the transform utility per window and the gumball gesture transactions.", [[1784, 2053]]),
    fchunk("🧾️history-kit-close-laws", "🧾️ History, kit and close laws: the history edit reference chips, the kit in-port, the backbone convergence and the terminal close witness.", [[2055, 2233]]),
])

os.makedirs(OUT, exist_ok=True)
for n, s in SPECS.items():
    json.dump(s, open(f"{OUT}/{n}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("wrote", sorted(SPECS))
