/** 🧬️ Fem2d artifact schema — every field with its state class. */

export interface Fem2dArtifact {
  /** @state artifact */
  nodes: FemNode[];
  /** @state artifact */
  elements: FemElement[];
  /** @state artifact */
  regions: FemRegion[];
  /** @state artifact */
  materials: FemMaterial[];
  /** @state artifact */
  sections: FemSection[];
  /** @state artifact */
  supports: FemSupport[];
  /** @state artifact */
  loadCases: FemLoadCase[];
  /** @state artifact */
  combinations: FemCombination[];
  /** @state artifact */
  analysis: FemAnalysisSettings;
  /** @state presence */
  resultSourceId?: string;
  /** @state presence */
  resultMode: string;
  /** @state presence */
  resultModeIndex: number;
  /** @state config */
  camera: FemCamera;
  /** @state config */
  locale: string;
  /** @state artifact */
  solverResultsJson: string;
  /** @state artifact */
  meshPreviewJson: string;
}

