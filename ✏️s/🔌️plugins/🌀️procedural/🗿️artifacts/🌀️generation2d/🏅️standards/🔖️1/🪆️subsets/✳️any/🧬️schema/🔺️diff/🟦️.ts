/** 🧬️ Generation2d diff schema — sparse field delta. */

export interface Generation2dDiff {
  /** @state artifact */
  artifact?: Generation2dArtifact;
  /** @state artifact */
  fixture?: FlowFixture;
  /** @state artifact */
  generation?: GenerationPlayState;
  /** @state presence */
  selectedIds?: Generation2dStringList;
  /** @state config */
  graphCamera?: CameraJson;
  /** @state config */
  showMode?: string;
  /** @state presence */
  selectedGenerationId?: string | null;
  /** @state artifact */
  generationPreviewText?: string | null;
  /** @state config */
  locale?: string;
}

export type Generation2dStringList = { values: string[] };
export interface Generation2dArtifact { /* see artifact facet */ }

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
