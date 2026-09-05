/** 🧬️ Iso16757 artifact schema — every field with its state class. */

export interface Iso16757Artifact {
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
  /** @state presence */
  selectedCheckIndex?: number | null;
}
