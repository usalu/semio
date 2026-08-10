/** 🧬️ Fem3d artifact schema — every field with its state class. */

export interface Fem3dArtifact {
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
  /** @state shared-ui */
  resultSourceId?: string;
  /** @state shared-ui */
  resultMode: string;
  /** @state shared-ui */
  resultModeIndex: number;
  /** @state local-ui */
  camera: FemCamera;
  /** @state preview */
  solverResultsJson: string;
  /** @state preview */
  meshPreviewJson: string;
}

