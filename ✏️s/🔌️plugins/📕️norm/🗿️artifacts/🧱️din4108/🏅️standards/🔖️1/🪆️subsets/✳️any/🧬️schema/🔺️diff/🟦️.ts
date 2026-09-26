/** 🧬️ Din4108 diff schema — sparse field delta over the envelope subject. */

export interface Din4108Diff {
  /** @state artifact */
  climateZone?: string;
  /** @state artifact */
  usage?: string;
  /** @state artifact */
  tIntC?: number;
  /** @state artifact */
  rhInt?: number;
  /** @state artifact */
  hasMechanicalVentilation?: boolean;
  /** @state artifact */
  airtightnessN50?: number;
  /** @state artifact */
  bb2DetailsConform?: boolean;
  /** @state artifact */
  zones?: { values: unknown[] };
  /** @state artifact */
  elements?: { values: unknown[] };
  /** @state artifact */
  thermalBridges?: { values: unknown[] };
}
