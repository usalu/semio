/** 💡️ Generation3d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph) and geometry (the evaluation of every widget, summarised). */

export interface Generation3dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export type Generation3dGeometryQuality = "exact-analytic" | "exact-numerical" | "approximate" | "mesh-derived-brep" | "polygon-mesh" | "tessellated-mesh";

export interface Generation3dFaultRecord {
  code: string;
  en: string;
  de: string;
  port: string | null;
}

export interface Generation3dOutputRecord {
  port: string;
  kind: string;
  detail: string;
}

export interface Generation3dWidgetRecord {
  quality: Generation3dGeometryQuality;
  fault: Generation3dFaultRecord | null;
  outputs: Generation3dOutputRecord[];
}

export interface Generation3dGeometryRecord {
  widgets: Record<string, Generation3dWidgetRecord>;
  faulted: number;
}

export interface Generation3dInference {
  /** @derived */
  topology: Generation3dTopology;
  /** @derived */
  geometry: Generation3dGeometryRecord;
}

export const generation3dGeometryQualities: readonly Generation3dGeometryQuality[] = ["exact-analytic", "exact-numerical", "approximate", "mesh-derived-brep", "polygon-mesh", "tessellated-mesh"];
