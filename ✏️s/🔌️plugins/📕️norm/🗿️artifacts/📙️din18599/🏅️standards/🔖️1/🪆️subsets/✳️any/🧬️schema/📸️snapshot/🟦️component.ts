/** 🧬️ Din18599 snapshot schema — persistent fields only. */

export interface Din18599Snapshot {
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
}
