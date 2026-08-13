/** 🧬️ Procedural3d diff schema — sparse field delta. */

export interface Procedural3dDiff {
  /** @state artifact */
  artifact?: Procedural3dArtifact;
  /** @state artifact */
  fixture?: FlowFixture;
  /** @state artifact */
  generation?: GenerationPlayState;
  /** @state presence */
  selectedNodeIds?: Procedural3dStringList;
  /** @state config */
  lodMode?: string;
  /** @state config */
  showMode?: string;
  /** @state config */
  selectionMethod?: string;
  /** @state artifact */
  hoveredNodeId?: string | null;
  /** @state config */
  graphCamera?: CameraJson;
  /** @state config */
  previewCamera?: Procedural3dPreviewCamera;
  /** @state config */
  sunJson?: string;
  /** @state presence */
  selectedGenerationId?: string | null;
  /** @state artifact */
  generationPreviewText?: string | null;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  locale?: string;
  /** @state config */
  contributionsJson?: string;
}

export type Procedural3dStringList = { values: string[] };
export interface Procedural3dArtifact { /* see artifact facet */ }

export type CameraJson = { x: number; y: number; zoom: number };
export type WidgetLayout = { x: number; y: number };
export type SynapseSpec = { id: string; from: string; to: string; fromPort: string; toPort: string };
/** @description Polymorphic flow widget — JSON blob. */
export type Widget = string;
export type FlowFixture = {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
};
export type FormGeneration = { id: string; name: string; valuesJson: string };
export type GenerationPlayState = {
  generations: FormGeneration[];
  selectedGenerationId?: string;
  previewText?: string;
};
export type Procedural3dPreviewCamera = {
  positionX: number;
  positionY: number;
  positionZ: number;
  targetX: number;
  targetY: number;
  targetZ: number;
  fov: number;
};
