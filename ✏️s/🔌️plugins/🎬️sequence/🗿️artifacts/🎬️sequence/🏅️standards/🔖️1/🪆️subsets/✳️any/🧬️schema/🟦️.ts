/** 🧬️ Sequence artifact schema — every field with its state class. */
export interface SequenceArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ steps: SequenceStep[];
  /** @state artifact */ edges: SequenceEdge[];
  /** @state config */ lastRunJson: string;
  /** @state config */ orientation: string;
  /** @state config */ camera: SequenceCamera;
  /** @state config */ locale: string;
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }
