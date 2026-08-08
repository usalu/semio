/** 🧬️ Iso16757 snapshot schema — persistent fields only. */

export interface Iso16757Snapshot {
  /** @state persistent */
  catalogue: string;
  /** @state persistent */
  dictionary: string;
  /** @state persistent */
  geometry: string;
  /** @state persistent */
  selection: string;
  /** @state persistent */
  partNumberRule: string;
  /** @state persistent */
  partNumberInputs: Record<string, string>;
  /** @state persistent */
  scriptLimits: string;
  /** @state persistent */
  exchangeProcess: string;
}
