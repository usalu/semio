/** 🧬️ Sequence diff schema — sparse field delta. */
export interface SequenceDiff {
  /** @state persistent */ artifact?: SequenceArtifact;
  /** @state persistent */ schema?: string;
  /** @state persistent */ steps?: SequenceStepsDelta;
  /** @state persistent */ edges?: SequenceEdgesDelta;
  /** @state shared-ui */ selectedStepIds?: SequenceStringList;
  /** @state local-ui */ lastRunJson?: string;
  /** @state local-ui */ orientation?: string;
  /** @state local-ui */ camera?: SequenceCamera;
  /** @state local-ui */ locale?: string;
}
export interface SequenceStringList { values: string[]; }
export interface SequenceStepsDelta { added: SequenceStep[]; removed: string[]; patched: SequenceStepPatchEntry[]; reordered?: string[]; }
export interface SequenceEdgesDelta { added: SequenceEdge[]; removed: string[]; patched: SequenceEdgePatchEntry[]; reordered?: string[]; }
export interface SequenceStepPatchEntry { id: string; patch: SequenceStepPatch; }
export interface SequenceEdgePatchEntry { id: string; patch: SequenceEdgePatch; }
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceStepPatch { params?: Record<string, unknown>; x?: number; y?: number; collapsed?: boolean; }
export interface SequenceEdgePatch { from?: string; to?: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }
export interface SequenceArtifact {
  schema: string; steps: SequenceStep[]; edges: SequenceEdge[];
  selectedStepIds: string[]; lastRunJson: string; orientation: string; camera: SequenceCamera; locale: string;
}
