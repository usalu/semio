/** 🧬️ Procedural2d artifact schema — every field with its state class. */

export interface Procedural2dArtifact {
  /** @state persistent */
  fixture: FlowFixture;
  /** @state persistent */
  generation: GenerationPlayState;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state local-ui */
  graphCamera: CameraJson;
  /** @state local-ui */
  showMode: string;
  /** @state shared-ui */
  selectedGenerationId?: string;
  /** @state preview */
  generationPreviewText?: string;
  /** @state local-ui */
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
export type FormGeneration = { id: string; name: string; valuesJson: string };
export type GenerationPlayState = {
  generations: FormGeneration[];
  selectedGenerationId?: string;
  previewText?: string;
};
