/** 🧬️ Fem3d diff schema — sparse field delta. */

export interface Fem3dDiff {
  /** @state persistent */
  artifact?: Fem3dArtifact;
  /** @state persistent */
  nodes?: Fem3dNodesDelta;
  /** @state persistent */
  elements?: Fem3dElementsDelta;
  /** @state persistent */
  materials?: Fem3dMaterialsDelta;
  /** @state persistent */
  sections?: Fem3dSectionsDelta;
  /** @state persistent */
  solids?: Fem3dSolidsDelta;
  /** @state persistent */
  supports?: Fem3dSupportsDelta;
  /** @state persistent */
  loadCases?: Fem3dLoadCasesDelta;
  /** @state persistent */
  combinations?: Fem3dCombinationsDelta;
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
  /** @state preview */
  solverResultsJson?: string;
  /** @state preview */
  meshPreviewJson?: string;
}

export interface Fem3dNodesDelta {
  added: FemNode[];
  removed: string[];
  patched: Fem3dNodesPatchEntry[];
  reordered?: string[];
}

export interface Fem3dNodesPatchEntry {
  id: string;
  item: FemNode;
}

export interface Fem3dElementsDelta {
  added: FemElement[];
  removed: string[];
  patched: Fem3dElementsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dElementsPatchEntry {
  id: string;
  item: FemElement;
}

export interface Fem3dMaterialsDelta {
  added: FemMaterial[];
  removed: string[];
  patched: Fem3dMaterialsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dMaterialsPatchEntry {
  id: string;
  item: FemMaterial;
}

export interface Fem3dSectionsDelta {
  added: FemSection[];
  removed: string[];
  patched: Fem3dSectionsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSectionsPatchEntry {
  id: string;
  item: FemSection;
}

export interface Fem3dSolidsDelta {
  added: FemSolid[];
  removed: string[];
  patched: Fem3dSolidsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSolidsPatchEntry {
  id: string;
  item: FemSolid;
}

export interface Fem3dSupportsDelta {
  added: FemSupport[];
  removed: string[];
  patched: Fem3dSupportsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dSupportsPatchEntry {
  id: string;
  item: FemSupport;
}

export interface Fem3dLoadCasesDelta {
  added: FemLoadCase[];
  removed: string[];
  patched: Fem3dLoadCasesPatchEntry[];
  reordered?: string[];
}

export interface Fem3dLoadCasesPatchEntry {
  id: string;
  item: FemLoadCase;
}

export interface Fem3dCombinationsDelta {
  added: FemCombination[];
  removed: string[];
  patched: Fem3dCombinationsPatchEntry[];
  reordered?: string[];
}

export interface Fem3dCombinationsPatchEntry {
  id: string;
  item: FemCombination;
}

