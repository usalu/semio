"""🗺️ c4p1 split plans for Package 1 (os/♾️infinite world crate roots and the two host unit-test roots).
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `pub use <mod>::*;`). `mode` "include": children are `include!` fragments (the test paths stay identical).
`ranges` are 1-based inclusive line ranges of the pre-edit file; each starts at an item start (docs and attributes included) and ends at an item end (or its trailing region marker)."""
from c4plib import INF

W = INF
H = f"{W}/🎛️host"
PLANS = {
    "bridge": dict(
        rel=f"{W}/🌉️bridge/🦀️.rs", mode="mod", lib=f"{W}/🌉️bridge/🦀️.rs", pkg="semio-framework-os-infinite-world-bridge",
        children=[
            dict(folder="🎞️scene-bridge", mod="scene_bridge", ranges=[(78, 1016)], doc="🎞️ World3d guest scene bridge: scene records, camera fit, the bridge cursor, snapshot publication and selection sync."),
            dict(folder="🖌️overlays-and-gumball", mod="overlays_and_gumball", ranges=[(1018, 1727)], doc="🖌️ World3d overlays: vertex markers, reference outlines, component overlays, gumball geometry, picking and commit."),
            dict(folder="🎥️camera-and-retirement", mod="camera_and_retirement", ranges=[(1729, 2525)], doc="🎥️ World3d projection, native mesh admission, snapshot apply, navigation classification, dynamic retirement and draw rebuild."),
        ]),
    "host": dict(
        rel=f"{H}/🦀️.rs", mode="mod", lib=f"{H}/🦀️.rs", pkg="semio-framework-os-infinite-world-host",
        children=[
            dict(folder="🔄️sync-and-overlay", mod="sync_and_overlay", ranges=[(256, 941)], doc="🔄️ World3d host sync: camera fit and scene bridge steps, state sync, tool run trace draws and the modelling overlay."),
            dict(folder="🎬️render", mod="render", ranges=[(943, 1437)], doc="🎬️ World3d host render: `render_world_3d` and the component face overlay build."),
            dict(folder="📥️snapshot-and-input", mod="snapshot_and_input", ranges=[(1439, 1999)], doc="📥️ World3d host snapshot apply, pointer button entry, interaction steps and dynamic retirement."),
        ]),
    "model": dict(
        rel=f"{W}/🗃️model/🦀️.rs", mode="mod", lib=f"{W}/🗃️model/🦀️.rs", pkg="semio-framework-os-infinite-world-model",
        children=[
            dict(folder="🎒️prepared-resources", mod="prepared_resources", ranges=[(25, 442)], doc="🎒️ World3d prepared frame resources, build context and the cursor wake authority."),
            dict(folder="🧬️scene-records", mod="scene_records", ranges=[(444, 1012)], doc="🧬️ World3d scene records: lod, chunking, targets, references, pick targets, environment and terrain style."),
            dict(folder="🗂️registries", mod="registries", ranges=[(1014, 1890)], doc="🗂️ World3d dynamic, mesh and draw registries, opaque quarantine, session and interaction state."),
            dict(folder="🖍️tool-run-trace-layer", mod="tool_run_trace_layer", ranges=[(1892, 2301)], doc="🖍️ World3d tool run trace layer: limits, store, paint and the refcount pool."),
            dict(folder="🔖️interaction-transaction", mod="interaction_transaction", ranges=[(2316, 2967)], doc="🔖️ World3d interaction transaction: capacities, ids, mesh and object registries, intents, plans and steps."),
        ]),
    "components": dict(
        rel=f"{W}/🧩️components/🦀️.rs", mode="mod", lib=f"{W}/🧩️components/🦀️.rs", pkg="semio-framework-os-infinite-world-components",
        children=[
            dict(folder="🫧️face-overlay-and-fit", mod="face_overlay_and_fit", ranges=[(544, 1203)], doc="🫧️ World3d camera fit, gumball config, brush runs, face overlay cursors, prepared status and snapshot apply cursor."),
            dict(folder="🪄️placeholder-mesh-and-gumball", mod="placeholder_mesh_and_gumball", ranges=[(1205, 1961)], doc="🪄️ World3d relocate session, gumball handles, the placeholder mesh authority and mesh buffer helpers."),
        ]),
    "cursors": dict(
        rel=f"{W}/🧲️cursors/🦀️.rs", mode="mod", lib=f"{W}/🧲️cursors/🦀️.rs", pkg="semio-framework-os-infinite-world-cursors",
        children=[
            dict(folder="🔲️marquee", mod="marquee", ranges=[(254, 1186)], doc="🔲️ World3d marquee gesture, pick cursor and publish jobs for objects and components."),
            dict(folder="🕹️gumball-and-brush", mod="gumball_and_brush", ranges=[(1188, 2146)], doc="🕹️ World3d component and gumball targets, the gumball pick cursor, gestures and commit jobs, and the brush commit job."),
            dict(folder="🎯️ray-and-component-pick", mod="ray_and_component_pick", ranges=[(2148, 2921)], doc="🎯️ World3d authority step and the ray, object and component pick cursors."),
        ]),
    "scene": dict(
        rel=f"{W}/🧱️scene/🦀️.rs", mode="mod", lib=f"{W}/🧱️scene/🦀️.rs", pkg="semio-framework-os-infinite-world-scene",
        children=[
            dict(folder="📡️asset-io", mod="asset_io", ranges=[(129, 819)], doc="📡️ World3d asset io authority: request ladder, fetch owner, responses and asset publication."),
            dict(folder="🥽️brush-lod-and-style", mod="brush_lod_and_style", ranges=[(821, 1678)], doc="🥽️ World3d brush mesh announce, reference images, mesh lease publication, chunking, lod grid, mesh style and the mesh pool."),
            dict(folder="🏔️environment-and-terrain", mod="environment_and_terrain", ranges=[(1680, 2407)], doc="🏔️ World3d environment lighting and shadows, terrain tiles and meshing, and the placeholder meshes."),
        ]),
    "interaction": dict(
        rel=f"{W}/🧵️interaction/🦀️.rs", mode="mod", lib=f"{W}/🧵️interaction/🦀️.rs", pkg="semio-framework-os-infinite-world-interaction",
        children=[
            dict(folder="🖱️pointer-and-picking", mod="pointer_and_picking", ranges=[(245, 1019)], doc="🖱️ World3d pointer handlers, context menu, action previews, gumball drag and the pick entry points."),
            dict(folder="🌪️vortex-relocate-and-terrain", mod="vortex_relocate_and_terrain", ranges=[(1021, 1619)], doc="🌪️ World3d vortex arrows, relocate drag, catalogue drop preview, environment helpers, terrain sync and placeholder mesh steps."),
            dict(folder="⚙️authority-core", mod="authority_core", ranges=[(1795, 2701)], doc="⚙️ World3d interaction authority: the settle point, queue coalescing, intent steps and close."),
            dict(folder="📜️plan-and-publish", mod="plan_and_publish", ranges=[(2703, 3275)], doc="📜️ World3d interaction plans: camera settle, wheel, drag and paint plans, plan publication and the active census."),
        ]),
    "unit": dict(
        rel=f"{H}/🧪️tests/🔬️unit/🦀️.rs", mode="include", lib=f"{H}/🧪️tests/🔬️unit/🦀️.rs", pkg=None,
        children=[
            dict(folder="🔲️cursor-and-marquee-laws", mod="cursor_and_marquee_laws", ranges=[(165, 861)], doc="🔲️ Laws of the object, component and marquee cursors, the face overlay and the right-gesture intents."),
            dict(folder="🕹️gumball-and-authority-laws", mod="gumball_and_authority_laws", ranges=[(863, 1565)], doc="🕹️ Laws of the gumball and brush jobs, the wheel, drag and paint plans, the ray pick cursor and the authority."),
            dict(folder="🖼️reference-and-pick-lane-laws", mod="reference_and_pick_lane_laws", ranges=[(1567, 2265)], doc="🖼️ Laws of the reference underlay, scene lanes, the pick target lane, camera framing and component overlays."),
            dict(folder="🧲️selection-and-retirement-laws", mod="selection_and_retirement_laws", ranges=[(2267, 2950)], doc="🧲️ Laws of marquee previews, selection previews, dynamic retirement, retained draw rebuild and the url mesh bridge."),
        ]),
    "unit2": dict(
        rel=f"{H}/🧪️tests/🔭️unit-2/🦀️.rs", mode="include", lib=f"{H}/🧪️tests/🔭️unit-2/🦀️.rs", pkg=None,
        children=[
            dict(folder="🥽️brush-environment-and-hover-laws", mod="brush_environment_and_hover_laws", ranges=[(168, 1019)], doc="🥽️ Laws of the glb asset and brush mesh announce, environment and shadows, terrain, the context menu and domain-bound previews."),
            dict(folder="🌉️scene-bridge-and-camera-laws", mod="scene_bridge_and_camera_laws", ranges=[(1021, 1759)], doc="🌉️ Laws of the scene bridge, camera framing and fit ownership, transparent draws and projection selection."),
            dict(folder="🖼️reference-and-component-laws", mod="reference_and_component_laws", ranges=[(1761, 2455)], doc="🖼️ Laws of the reference visual, the world grid, native analytic component picks and face selection overlays."),
            dict(folder="🧲️native-gumball-and-primary-laws", mod="native_gumball_and_primary_laws", ranges=[(2457, 3026)], doc="🧲️ Laws of the face overlay invalidation, native component gumballs and native primary geometry."),
        ]),
}
ORDER = ["bridge", "host", "model", "components", "cursors", "scene", "interaction", "unit", "unit2"]
