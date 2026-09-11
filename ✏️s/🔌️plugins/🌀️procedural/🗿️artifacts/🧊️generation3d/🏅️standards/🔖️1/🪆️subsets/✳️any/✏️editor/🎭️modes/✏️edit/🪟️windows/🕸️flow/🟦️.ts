/** 🕸️ Generation3d editor — Flow window (edit mode): typed twin of `🦀️.rs`'s view-model.
 * Mirrors the pane's `render(document: &Generation3dSnapshot, config: &Generation3dConfig, session:
 * &FlowEvalSession)` boundary — the editable node-graph scene (operators, catalogue, live eval) a
 * mutation-capable surface carries. There is no viewer twin of this window: the read-only surface
 * ships a single mesh-preview window instead (see `👁️viewer/…/🟦️.ts`). */

/** ✏️ The Flow window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface Generation3dFlowViewModel {
  windowKindId: "procedural-main";
  bodyKey: "procedural.play.main";
  surfaceId: "procedural.play";
  /** 📷️ The flow-graph node canvas camera (`x`/`y`/`zoom`). */
  camera: { x: number; y: number; zoom: number };
  /** 🎚️ Level-of-detail tessellation mode driving the LOD chrome measure. */
  lodMode: string;
  /** 🧬️ Whether the graph is editable (always true for the editor's own Flow window). */
  editable: true;
}

/** 🔌️ One port of one node, keyed by the `graph` domain's own `{nodeId}@{portId}` handle id. A port id
 * is unique per node and NOT per direction, so a port reached from both sides is one entry carrying
 * both directions. */
export interface Generation3dFlowOutlinePort {
  id: string;
  label: string;
  directions: readonly ("input" | "output")[];
}

/** 🧩️ One node of the open graph: the widget id verbatim (the `node` granularity target), its ports,
 * and the localized `NodeEvalStatus` the flow session reported for it. */
export interface Generation3dFlowOutlineNode {
  id: string;
  label: string;
  status?: "ok" | "stale" | "queued" | "computing" | "error" | "blocked";
  ports: readonly Generation3dFlowOutlinePort[];
}

/** 🔗️ One wire, keyed by the synapse id the `edge` granularity carries. */
export interface Generation3dFlowOutlineWire {
  id: string;
  sourceNodeId: string;
  sourcePortId: string;
  targetNodeId: string;
  targetPortId: string;
}

/** 🕸️ The graph as semantic rows — the Flow window's renderer-neutral body, beside the GPU canvas that
 * paints the same records. Bound to the `graph` interaction domain, so a row's hover and a row's click
 * are the very same targets the canvas picks into. */
export interface Generation3dFlowOutline {
  interactionDomain: "graph";
  nodes: readonly Generation3dFlowOutlineNode[];
  wires: readonly Generation3dFlowOutlineWire[];
}

export const GENERATION3D_PLAY_FLOW_WINDOW_KIND_ID = "procedural-main" as const;
export const GENERATION3D_PLAY_FLOW_BODY_KEY = "procedural.play.main" as const;
export const GENERATION3D_PLAY_FLOW_SURFACE_ID = "procedural.play" as const;
export const GENERATION3D_PLAY_FLOW_OUTLINE_ID = "procedural-play-graph" as const;
export const GENERATION3D_PLAY_FLOW_OUTLINE_NODES_SECTION_ID = "procedural-play-graph.nodes" as const;
export const GENERATION3D_PLAY_FLOW_OUTLINE_WIRES_SECTION_ID = "procedural-play-graph.wires" as const;
