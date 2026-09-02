/** 🧬️ GIS map snapshot schema — artifact-lane fields only. */

export interface GisMapSnapshot {
  /** @state artifact */
  positions: GisMapFeature[];
  /** @state artifact */
  routes: GisMapFeature[];
  /** @state artifact */
  regions: GisMapFeature[];
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}
