/** 💡️ GIS map inference schema — per-collection feature counts + geographic bounding box. */

export interface GisMapBounds {
  lonMin: number;
  lonMax: number;
  latMin: number;
  latMax: number;
}

export interface GisMapInference {
  /** @derived */
  positionCount: number;
  /** @derived */
  routeCount: number;
  /** @derived */
  regionCount: number;
  /** @derived */
  bounds: GisMapBounds | null;
}
