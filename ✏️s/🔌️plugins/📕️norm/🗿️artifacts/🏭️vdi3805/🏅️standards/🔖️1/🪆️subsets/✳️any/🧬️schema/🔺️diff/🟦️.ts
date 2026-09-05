/** 🧬️ Vdi3805 diff schema — sparse field delta. */

export interface Vdi3805Diff {
  /** @state artifact */
  artifact?: Vdi3805Artifact;
  /** @state artifact */
  manufacturerFile?: string;
  /** @state artifact */
  catalog?: string;
  /** @state artifact */
  editionProfile?: Record<string, string>;
  /** @state artifact */
  correctionAsOf?: string;
  /** @state artifact */
  strictMode?: boolean;
  /** @state artifact */
  index?: string;
  /** @state artifact */
  geometry?: Record<string, string>;
  /** @state artifact */
  curves?: Record<string, string>;
  /** @state artifact */
  limits?: string;
  /** @state presence */
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
