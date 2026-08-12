/** 💡️ GIS map inference schema — per-collection feature counts + geographic bounding box. */

export interface GisMapBounds {
  lonMin: number;
  lonMax: number;
  latMin: number;
  latMax: number;
}

export interface GisMapInference {
  /** @state inferred */
  positionCount: number;
  /** @state inferred */
  routeCount: number;
  /** @state inferred */
  regionCount: number;
  /** @state inferred */
  bounds: GisMapBounds | null;
}
