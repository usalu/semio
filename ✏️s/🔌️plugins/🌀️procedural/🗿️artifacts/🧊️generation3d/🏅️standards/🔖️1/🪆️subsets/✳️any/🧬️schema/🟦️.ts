/** 🧬️ Generation3d artifact schema — every field with its state class. */

export interface Generation3dArtifact {
  /** @state artifact */
  fixture: FlowFixture;
  /** @state artifact */
  generation: GenerationPlayState;
  /** @state presence */
  selectedNodeIds: string[];
  /** @state config */
  lodMode: string;
  /** @state config */
  showMode: string;
  /** @state config */
  selectionMethod: string;
  /** @state artifact */
  hoveredNodeId?: string;
  /** @state config */
  graphCamera: CameraJson;
  /** @state config */
  previewCamera: Generation3dPreviewCamera;
  /** @state config */
  sunJson: string;
  /** @state presence */
  selectedGenerationId?: string;
  /** @state artifact */
  generationPreviewText?: string;
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}

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
export type FormGeneration = { id: string; name: string; values: Record<string, unknown> };
export type GenerationPlayState = {
  generations: FormGeneration[];
  selectedGenerationId?: string;
  previewText?: string;
};
export type Generation3dPreviewCamera = {
  positionX: number;
  positionY: number;
  positionZ: number;
  targetX: number;
  targetY: number;
  targetZ: number;
  fov: number;
};
