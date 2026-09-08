/** 🧬️ SequenceConfig */
export interface SequenceConfig {
  /** @state config */
  lastRunJson: string;
  /** @state config */
  orientation: string;
  /** @state config */
  camera: SequenceCamera;
  /** @state config */
}
export interface SequenceCamera { x: number; y: number; zoom: number; }
