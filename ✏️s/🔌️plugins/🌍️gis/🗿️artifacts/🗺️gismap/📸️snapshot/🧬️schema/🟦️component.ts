/** 🧬️ GIS map snapshot schema — persistent fields only. */

export interface GisMapSnapshot {
  /** @state persistent */
  positions: GisMapFeature[];
  /** @state persistent */
  routes: GisMapFeature[];
  /** @state persistent */
  regions: GisMapFeature[];
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}
