# GIS Map Window Migration Preparation

The remaining view record is entirely concrete-window state. The six fields are layer_visibility, camera_json, render_mode, vector_style, lod_mode, and layer_stroke_scale. Root has made no GIS Map window-migration source edits. Existing document contract and exact Pack envelope changes must remain.

## Exact Current Owners

- Current runtime record and codec: ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs.
- Duplicate declared schema plus all five facets, diff and six mutation leaves live below that config folder. Move the entire domain record to the Map window config owner and give the record, schema identifiers and facets an explicit window name. The sibling Terrain implementation now uses ⚙️config under its concrete terrain window.
- Artifact root 🦀️.rs mounts current editor::gis2d::config at lines 829–837. The map module is under editor::gis2d::modes::edit::windows::map. Mount the moved config there; do not retain the old app module alias.
- Actual map kind is gis2d-main. Commands must obtain concrete window_id from the dispatch context and confirm its matching window roster entry. A hardcoded kind does not identify an instance.
- App runtime at editor/🦀️.rs uses Gis2dConfig and Gis2dConfigMutation at associated types 621–622. Convert to NoConfig/NoConfigMutation and register exact MapWindowConfigOwner.
- Artifact app assembly is gated by component-app-assembly. Native window tests must explicitly enable that existing feature or demonstrate it is already enabled in the registered route.

## Producers And Consumers

The view command module contains toggleLayerVisibility, fitWorld, setCamera, setRenderMode, setVectorStyle, setLodMode, focusFeature and setLayerStrokeScale. The example command also fits a new document camera and emits config; preserve document changes while directing that camera to the caller's exact window. Features, inference and shell handlers carry the app config generic too. Their signatures must use NoConfig, with any actual camera read coming from the captured window snapshot.

The retained command reducer currently reconstructs ConfigView from the app config. It must carry the request's window_config into ConfigView, as the Terrain retained reducer does. Dispatch and retained dispatch must use the same window addressing mechanism.

Map render consumes all six fields, as do five option measures. Document and inspection panels currently receive the same app config. These panels must read the captured/focused exact map-window context; do not silently select another instance by kind. window_measures currently returns a map keyed by the fixed kind; the SDK's concrete instance contract must be followed.

MapHost projection consumes window state for fitWorld/focusFeature. It remains a derived view of document plus the captured map config. The map record has BTreeMap layer overrides and JSON camera text; preserve sparse deletion/reset semantics and bounded publication admission.

## Verification

Create language-neutral vectors before implementation: two same-kind concrete windows, each of the six fields mutated only on the left, right unchanged, exact left-only undo/redo, reopen restoring each local window record, missing/foreign window rejection, and document pack unchanged by view commands. Validate expected patches with existing Ajv and fast-json-patch. Native laws must drive actual retained/public commands and inspect actual scene fields and measures, including fitWorld and example-triggered camera capture. Preserve prior direct mutation and codec tests after moving their owner.
