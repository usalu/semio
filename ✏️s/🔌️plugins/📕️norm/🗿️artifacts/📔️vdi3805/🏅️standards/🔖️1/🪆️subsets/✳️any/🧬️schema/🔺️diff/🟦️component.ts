/** 🧬️ Vdi3805 diff schema — sparse field delta. */

export interface Vdi3805Diff {
  /** @state persistent */
  artifact?: Vdi3805Artifact;
  /** @state persistent */
  manufacturerFile?: string;
  /** @state persistent */
  catalog?: string;
  /** @state persistent */
  editionProfile?: Record<string, string>;
  /** @state persistent */
  correctionAsOf?: string;
  /** @state persistent */
  strictMode?: boolean;
  /** @state persistent */
  index?: string;
  /** @state persistent */
  geometry?: Record<string, string>;
  /** @state persistent */
  curves?: Record<string, string>;
  /** @state persistent */
  limits?: string;
  /** @state shared-ui */
  selectedCheckIndex?: number | null;
}

export interface Vdi3805Artifact {
  manufacturerFile: string;
  catalog: string;
  editionProfile: Record<string, string>;
  correctionAsOf: string;
  strictMode: boolean;
  index: string;
  geometry: Record<string, string>;
  curves: Record<string, string>;
  limits: string;
  selectedCheckIndex?: number | null;
}
