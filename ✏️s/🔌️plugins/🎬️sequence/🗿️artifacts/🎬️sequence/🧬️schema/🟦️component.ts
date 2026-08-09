/** 🧬️ Sequence artifact schema — every field with its state class. */
export interface SequenceArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ steps: SequenceStep[];
  /** @state persistent */ edges: SequenceEdge[];
  /** @state shared-ui */ selectedStepIds: string[];
  /** @state local-ui */ lastRunJson: string;
  /** @state local-ui */ orientation: string;
  /** @state local-ui */ camera: SequenceCamera;
  /** @state local-ui */ locale: string;
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }
