/** 💡️ GIS terrain inference schema — geographic bounding box + position count of the `map:in`
 * overlay decoded from `importedFeaturesJson`. */

export interface GisTerrainBounds {
  lonMin: number;
  lonMax: number;
  latMin: number;
  latMax: number;
}

export interface GisTerrainInference {
  /** @derived */
  positionCount: number;
  /** @derived */
  bounds: GisTerrainBounds | null;
}
