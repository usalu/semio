/** 🧬️ Iso16757 diff schema — sparse field delta. */

export interface Iso16757Diff {
  /** @state artifact */
  artifact?: Iso16757Artifact;
  /** @state artifact */
  catalogue?: string;
  /** @state artifact */
  dictionary?: string;
  /** @state artifact */
  geometry?: string;
  /** @state artifact */
  selection?: string;
  /** @state artifact */
  partNumberRule?: string;
  /** @state artifact */
  partNumberInputs?: Record<string, string>;
  /** @state artifact */
  scriptLimits?: string;
  /** @state artifact */
  exchangeProcess?: string;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface Iso16757Artifact {
  catalogue: string;
  dictionary: string;
  geometry: string;
  selection: string;
  partNumberRule: string;
  partNumberInputs: Record<string, string>;
  scriptLimits: string;
  exchangeProcess: string;
  selectedCheckIndex?: number | null;
}
