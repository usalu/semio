/** 🧬️ Sequence snapshot schema — artifact-lane fields only. */
export interface SequenceSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ steps: SequenceStep[];
  /** @state artifact */ edges: SequenceEdge[];
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
