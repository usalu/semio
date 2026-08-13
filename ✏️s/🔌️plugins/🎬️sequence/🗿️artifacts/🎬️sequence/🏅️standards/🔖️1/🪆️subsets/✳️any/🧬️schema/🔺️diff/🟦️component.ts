/** 🧬️ Sequence diff schema — sparse field delta. */
export interface SequenceDiff {
  /** @state artifact */ artifact?: SequenceArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact */ steps?: SequenceStepsDelta;
  /** @state artifact */ edges?: SequenceEdgesDelta;
  /** @state presence */ selectedStepIds?: SequenceStringList;
  /** @state config */ lastRunJson?: string;
  /** @state config */ orientation?: string;
  /** @state config */ camera?: SequenceCamera;
  /** @state config */ locale?: string;
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
