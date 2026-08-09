/** 🧬️ SequencePresence */
export interface SequencePresence {
  /** @state shared-ui */
  selectedStepIds: string[];
  /** @state shared-ui */
  orientation: string;
  /** @state shared-ui */
  camera: SequenceCamera;
}
export interface SequenceCamera { x: number; y: number; zoom: number; }
