/** 🧬️ Fem2d artifact schema — every field with its state class. */

export interface Fem2dArtifact {
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
  /** @state shared-ui */
  resultSourceId?: string;
  /** @state shared-ui */
  resultMode: string;
  /** @state shared-ui */
  resultModeIndex: number;
  /** @state local-ui */
  camera: FemCamera;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  solverResultsJson: string;
  /** @state preview */
  meshPreviewJson: string;
}

