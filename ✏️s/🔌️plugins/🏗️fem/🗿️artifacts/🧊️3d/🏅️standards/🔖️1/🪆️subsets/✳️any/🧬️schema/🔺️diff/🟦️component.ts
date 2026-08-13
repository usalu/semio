/** 🧬️ Fem3d diff schema — sparse field delta. */

export interface Fem3dDiff {
  /** @state artifact */
  artifact?: Fem3dArtifact;
  /** @state artifact */
  nodes?: Fem3dNodesDelta;
  /** @state artifact */
  elements?: Fem3dElementsDelta;
  /** @state artifact */
  materials?: Fem3dMaterialsDelta;
  /** @state artifact */
  sections?: Fem3dSectionsDelta;
  /** @state artifact */
  solids?: Fem3dSolidsDelta;
  /** @state artifact */
  supports?: Fem3dSupportsDelta;
  /** @state artifact */
  loadCases?: Fem3dLoadCasesDelta;
  /** @state artifact */
  combinations?: Fem3dCombinationsDelta;
  /** @state artifact */
  analysis?: FemAnalysisSettings;
  /** @state presence */
  resultSourceId?: string | null;
  /** @state presence */
  resultMode?: string;
  /** @state presence */
  resultModeIndex?: number;
  /** @state config */
  camera?: FemCamera;
  /** @state artifact */
  solverResultsJson?: string;
  /** @state artifact */
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

