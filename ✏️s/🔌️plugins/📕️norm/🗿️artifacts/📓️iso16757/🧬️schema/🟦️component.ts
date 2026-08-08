/** 🧬️ Iso16757 artifact schema — every field with its state class. */

export interface Iso16757Artifact {
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
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}
