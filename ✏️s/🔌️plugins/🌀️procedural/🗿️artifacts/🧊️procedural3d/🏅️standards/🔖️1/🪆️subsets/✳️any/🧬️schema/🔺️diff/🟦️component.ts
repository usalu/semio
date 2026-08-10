/** 🧬️ Procedural3d diff schema — sparse field delta. */

export interface Procedural3dDiff {
  /** @state persistent */
  artifact?: Procedural3dArtifact;
  /** @state persistent */
  fixture?: FlowFixture;
  /** @state persistent */
  generation?: GenerationPlayState;
  /** @state shared-ui */
  selectedNodeIds?: Procedural3dStringList;
  /** @state local-ui */
  lodMode?: string;
  /** @state local-ui */
  showMode?: string;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state preview */
  hoveredNodeId?: string | null;
  /** @state local-ui */
  graphCamera?: CameraJson;
  /** @state local-ui */
  previewCamera?: Procedural3dPreviewCamera;
  /** @state local-ui */
  sunJson?: string;
  /** @state shared-ui */
  selectedGenerationId?: string | null;
  /** @state preview */
  generationPreviewText?: string | null;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
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
