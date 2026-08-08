/** 🧬️ Fem3d snapshot schema — persistent fields only. */

export interface Fem3dSnapshot {
  /** @state persistent */
  nodes: FemNode[];
  /** @state persistent */
  elements: FemElement[];
  /** @state persistent */
  materials: FemMaterial[];
  /** @state persistent */
  sections: FemSection[];
  /** @state persistent */
  solids: FemSolid[];
  /** @state persistent */
  supports: FemSupport[];
  /** @state persistent */
  loadCases: FemLoadCase[];
  /** @state persistent */
  combinations: FemCombination[];
  /** @state persistent */
  analysis: FemAnalysisSettings;
}

