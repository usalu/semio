/** 🧬️ Din18599 diff schema — sparse field delta. */

export interface Din18599Diff {
  /** @state persistent */
  artifact?: Din18599Artifact;
  /** @state persistent */
  useClass?: string;
  /** @state persistent */
  heatedAreaM2?: number;
  /** @state persistent */
  occupants?: number;
  /** @state persistent */
  hT?: number;
  /** @state persistent */
  hV?: number;
  /** @state persistent */
  climate?: string;
  /** @state persistent */
  internalGainsWM2?: number;
  /** @state persistent */
  solarGainsKwh?: number;
  /** @state persistent */
  systemLossesKwh?: number;
  /** @state persistent */
  renewableKwh?: number;
  /** @state persistent */
  annualLimitKwh?: number;
  /** @state persistent */
  energyCarrier?: string;
  /** @state persistent */
  referenceQPKwh?: number;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}

export interface Din18599Artifact {
  useClass: string;
  heatedAreaM2: number;
  occupants: number;
  hT: number;
  hV: number;
  climate: string;
  internalGainsWM2: number;
  solarGainsKwh: number;
  systemLossesKwh: number;
  renewableKwh: number;
  annualLimitKwh: number;
  energyCarrier: string;
  referenceQPKwh: number;
  selectedCheckIndex?: number | null;
}