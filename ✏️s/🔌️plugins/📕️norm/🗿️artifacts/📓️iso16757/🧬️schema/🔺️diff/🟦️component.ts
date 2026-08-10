/** 🧬️ Iso16757 diff schema — sparse field delta. */

export interface Iso16757Diff {
  /** @state persistent */
  artifact?: Iso16757Artifact;
  /** @state persistent */
  catalogue?: string;
  /** @state persistent */
  dictionary?: string;
  /** @state persistent */
  geometry?: string;
  /** @state persistent */
  selection?: string;
  /** @state persistent */
  partNumberRule?: string;
  /** @state persistent */
  partNumberInputs?: Record<string, string>;
  /** @state persistent */
  scriptLimits?: string;
  /** @state persistent */
  exchangeProcess?: string;
  /** @state shared-ui */
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