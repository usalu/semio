/** 🧬️ SequenceConfig */
export interface SequenceConfig {
  /** @state config */
  selectedStepIds: string[];
  /** @state config */
  lastRunJson: string;
  /** @state config */
  orientation: string;
  /** @state config */
  camera: SequenceCamera;
  /** @state config */
  locale: string;
}
export interface SequenceCamera { x: number; y: number; zoom: number; }
