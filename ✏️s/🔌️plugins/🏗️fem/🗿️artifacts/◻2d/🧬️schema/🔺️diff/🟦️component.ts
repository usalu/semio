/** 🧬️ Fem2d diff schema — sparse field delta. */

export interface Fem2dDiff {
  /** @state persistent */
  artifact?: Fem2dArtifact;
  /** @state persistent */
  nodes?: Fem2dNodesDelta;
  /** @state persistent */
  elements?: Fem2dElementsDelta;
  /** @state persistent */
  regions?: Fem2dRegionsDelta;
  /** @state persistent */
  materials?: Fem2dMaterialsDelta;
  /** @state persistent */
  sections?: Fem2dSectionsDelta;
  /** @state persistent */
  supports?: Fem2dSupportsDelta;
  /** @state persistent */
  loadCases?: Fem2dLoadCasesDelta;
  /** @state persistent */
  combinations?: Fem2dCombinationsDelta;
  /** @state persistent */
  analysis?: FemAnalysisSettings;
  /** @state shared-ui */
  resultSourceId?: string | null;
  /** @state shared-ui */
  resultMode?: string;
  /** @state shared-ui */
  resultModeIndex?: number;
  /** @state local-ui */
  camera?: FemCamera;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  solverResultsJson?: string;
  /** @state preview */
  meshPreviewJson?: string;
}

export interface Fem2dNodesDelta {
  added: FemNode[];
  removed: string[];
  patched: Fem2dNodesPatchEntry[];
  reordered?: string[];
}

export interface Fem2dNodesPatchEntry {
  id: string;
  item: FemNode;
}

export interface Fem2dElementsDelta {
  added: FemElement[];
  removed: string[];
  patched: Fem2dElementsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dElementsPatchEntry {
  id: string;
  item: FemElement;
}

export interface Fem2dRegionsDelta {
  added: FemRegion[];
  removed: string[];
  patched: Fem2dRegionsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dRegionsPatchEntry {
  id: string;
  item: FemRegion;
}

export interface Fem2dMaterialsDelta {
  added: FemMaterial[];
  removed: string[];
  patched: Fem2dMaterialsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dMaterialsPatchEntry {
  id: string;
  item: FemMaterial;
}

export interface Fem2dSectionsDelta {
  added: FemSection[];
  removed: string[];
  patched: Fem2dSectionsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dSectionsPatchEntry {
  id: string;
  item: FemSection;
}

export interface Fem2dSupportsDelta {
  added: FemSupport[];
  removed: string[];
  patched: Fem2dSupportsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dSupportsPatchEntry {
  id: string;
  item: FemSupport;
}

export interface Fem2dLoadCasesDelta {
  added: FemLoadCase[];
  removed: string[];
  patched: Fem2dLoadCasesPatchEntry[];
  reordered?: string[];
}

export interface Fem2dLoadCasesPatchEntry {
  id: string;
  item: FemLoadCase;
}

export interface Fem2dCombinationsDelta {
  added: FemCombination[];
  removed: string[];
  patched: Fem2dCombinationsPatchEntry[];
  reordered?: string[];
}

export interface Fem2dCombinationsPatchEntry {
  id: string;
  item: FemCombination;
}

