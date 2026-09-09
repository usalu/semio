/** 👁️ Generation3d viewer — subset-level typed twin. Re-exports the viewer's view-state config and
 * its single window's typed view-model binding, so a host-side TS consumer has one import surface
 * for the whole viewer manifest, mirroring `🦀️.rs`'s `create_generation3d_viewer()` stitching the
 * mode, window, actions and interaction domain together. MUST NOT import anything from the sibling
 * `✏️editor` surface. */

export const GENERATION3D_VIEWER_DIALECT = { artifactKind: "s.procedural.generation3d", standard: "1", subset: "*" } as const;

export const GENERATION3D_VIEW_MODE_VIEW = "view" as const;

/** 🎯️ The controller every viewer chrome measure addresses its `on_change` action to. */
export const GENERATION3D_VIEW_APP_ID = "procedural3d-view" as const;

/** 👁️ Every action the read-only surface declares, in `Generation3dViewCommand` declaration order —
 * the wire ordinal is the row order, so appending is safe and reordering is a format break. Each is
 * an `InteractiveJobClassification::Migrated` job publishing on the config/presence/transient lanes
 * only; none of them can reach the document. */
export const GENERATION3D_VIEW_ACTIONS = ["setShowMode", "setLodMode", "setCamera", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"] as const;
export type Generation3dViewAction = (typeof GENERATION3D_VIEW_ACTIONS)[number];

export * as viewConfig from "./🎚️config/🧬️schema/🟦️";
export * as previewWindow from "./🎭️modes/👁️view/🪟️windows/👁️preview/🟦️";
