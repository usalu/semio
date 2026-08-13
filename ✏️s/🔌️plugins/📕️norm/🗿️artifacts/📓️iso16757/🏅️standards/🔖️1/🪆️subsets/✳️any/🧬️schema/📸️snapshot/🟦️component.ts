/** 🧬️ Iso16757 snapshot schema — persistent fields only. */

export interface Iso16757Snapshot {
  /** @state artifact */
  catalogue: string;
  /** @state artifact */
  dictionary: string;
  /** @state artifact */
  geometry: string;
  /** @state artifact */
  selection: string;
  /** @state artifact */
  partNumberRule: string;
  /** @state artifact */
  partNumberInputs: Record<string, string>;
  /** @state artifact */
  scriptLimits: string;
  /** @state artifact */
  exchangeProcess: string;
}
