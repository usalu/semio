/** 🧬️ Sequence snapshot schema — persistent fields only. */
export interface SequenceSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ steps: SequenceStep[];
  /** @state persistent */ edges: SequenceEdge[];
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
