/** 🧬️ SequencePresence */
export interface SequencePresence {
  /** @state presence */
  orientation: string;
  /** @state presence */
  camera: SequenceCamera;
}
export interface SequenceCamera { x: number; y: number; zoom: number; }
