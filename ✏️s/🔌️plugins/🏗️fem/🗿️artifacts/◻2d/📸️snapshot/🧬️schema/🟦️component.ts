/** 🧬️ Fem2d snapshot schema — persistent fields only. */

export interface Fem2dSnapshot {
  /** @state persistent */
  nodes: FemNode[];
  /** @state persistent */
  elements: FemElement[];
  /** @state persistent */
  regions: FemRegion[];
  /** @state persistent */
  materials: FemMaterial[];
  /** @state persistent */
  sections: FemSection[];
  /** @state persistent */
  supports: FemSupport[];
  /** @state persistent */
  loadCases: FemLoadCase[];
  /** @state persistent */
  combinations: FemCombination[];
  /** @state persistent */
  analysis: FemAnalysisSettings;
}

