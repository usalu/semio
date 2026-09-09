/** 👁️ Generation3d viewer — Preview window: typed twin of `🦀️.rs`'s view-model. A read-only
 * `World3dScene` over the evaluated, tessellated flow geometry, bound to the framework's `graph`
 * interaction domain so hover and selection paint here. No gumball and no utility row: a transform
 * handle is a mutation affordance and a viewer emits no mutations. */

import type { Generation3dViewCamera } from "../../../../🎚️config/🧬️schema/🟦️";

export const GENERATION3D_VIEW_PREVIEW_WINDOW_KIND_ID = "procedural-view-preview" as const;
export const GENERATION3D_VIEW_PREVIEW_BODY_KEY = "procedural.view.preview" as const;

/** 🕹️ The framework interaction domain this window is bound to, and the granularity a world-3d
 * instance pick reports — a channel-qualified `{widgetId}@{channel}` port, i.e. a `handle`. */
export const GENERATION3D_VIEW_INTERACTION_DOMAIN = "graph" as const;
export const GENERATION3D_VIEW_INTERACTION_CHANNEL = "pointer" as const;
export const GENERATION3D_VIEW_INTERACTION_GRANULARITY = "handle" as const;

/** 👁️ The Preview window's typed view-model — mirrors the `World3dScene` payload this window builds. */
export interface Generation3dViewPreviewViewModel {
  windowKindId: typeof GENERATION3D_VIEW_PREVIEW_WINDOW_KIND_ID;
  bodyKey: typeof GENERATION3D_VIEW_PREVIEW_BODY_KEY;
  /** 📷️ The viewport camera, from the viewer's own persisted view state. */
  camera: Generation3dViewCamera;
  /** 🧊️ Evaluated + tessellated preview meshes, JSON-encoded (`MeshData[]` keyed by id). */
  meshesJson: string;
  /** 🧊️ World-placed instances referencing `meshesJson` entries, JSON-encoded. Each carries an
   * `interactionId` plus live `hovered`/`selected` flags. */
  instancesJson: string;
  /** 🧭️ Selection payload: marked ids, hovered id and the shading flags the show mode implies. */
  selectionJson: string;
  /** 🕹️ Always `"graph"` / `"handle"` — the declared domain a pick on this surface resolves into. */
  domainId: typeof GENERATION3D_VIEW_INTERACTION_DOMAIN;
  domainGranularityId: typeof GENERATION3D_VIEW_INTERACTION_GRANULARITY;
}

/** 🕹️ One render's resolved `graph` marks — the typed twin of `Generation3dViewMarks`. An id counts
 * as marked when the domain names the instance, its channel, or its widget. */
export interface Generation3dViewMarks {
  hovered: readonly string[];
  selected: readonly string[];
}

/** 🕸️ The widget id behind any interaction id — `w`, `w@c` and `w@c#i` all resolve to `w`. */
export const generation3dViewWidgetOf = (id: string): string => (id.split("#")[0] ?? id).split("@")[0] ?? id;

/** 🕹️ The same three-level match `Generation3dViewMarks::marked` implements in Rust. */
export const generation3dViewIsMarked = (marks: readonly string[], widgetId: string, channel: string, index: number): boolean =>
  marks.includes(widgetId) || marks.includes(`${widgetId}@${channel}`) || marks.includes(`${widgetId}@${channel}#${index}`);
