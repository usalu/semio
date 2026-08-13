/** 🧬️ Din18599 diff schema — sparse field delta. */

export interface Din18599Diff {
  /** @state artifact */
  artifact?: Din18599Artifact;
  /** @state artifact */
  useClass?: string;
  /** @state artifact */
  heatedAreaM2?: number;
  /** @state artifact */
  occupants?: number;
  /** @state artifact */
  hT?: number;
  /** @state artifact */
  hV?: number;
  /** @state artifact */
  climate?: string;
  /** @state artifact */
  internalGainsWM2?: number;
  /** @state artifact */
  solarGainsKwh?: number;
  /** @state artifact */
  systemLossesKwh?: number;
  /** @state artifact */
  renewableKwh?: number;
  /** @state artifact */
  annualLimitKwh?: number;
  /** @state artifact */
  energyCarrier?: string;
  /** @state artifact */
  referenceQPKwh?: number;
  /** @state presence */
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
