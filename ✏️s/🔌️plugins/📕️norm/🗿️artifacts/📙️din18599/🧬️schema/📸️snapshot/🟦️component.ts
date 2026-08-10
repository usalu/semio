/** 🧬️ Din18599 snapshot schema — persistent fields only. */

export interface Din18599Snapshot {
  /** @state persistent */
  useClass: string;
  /** @state persistent */
  heatedAreaM2: number;
  /** @state persistent */
  occupants: number;
  /** @state persistent */
  hT: number;
  /** @state persistent */
  hV: number;
  /** @state persistent */
  climate: string;
  /** @state persistent */
  internalGainsWM2: number;
  /** @state persistent */
  solarGainsKwh: number;
  /** @state persistent */
  systemLossesKwh: number;
  /** @state persistent */
  renewableKwh: number;
  /** @state persistent */
  annualLimitKwh: number;
  /** @state persistent */
  energyCarrier: string;
  /** @state persistent */
  referenceQPKwh: number;
}
