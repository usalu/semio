/** 🧬️ Din18599 artifact schema — every field with its state class. */

export interface Din18599Artifact {
  /** @state artifact */
  useClass: string;
  /** @state artifact */
  heatedAreaM2: number;
  /** @state artifact */
  occupants: number;
  /** @state artifact */
  hT: number;
  /** @state artifact */
  hV: number;
  /** @state artifact */
  climate: string;
  /** @state artifact */
  internalGainsWM2: number;
  /** @state artifact */
  solarGainsKwh: number;
  /** @state artifact */
  systemLossesKwh: number;
  /** @state artifact */
  renewableKwh: number;
  /** @state artifact */
  annualLimitKwh: number;
  /** @state artifact */
  energyCarrier: string;
  /** @state artifact */
  referenceQPKwh: number;
  /** @state presence */
  selectedCheckIndex?: number | null;
}
