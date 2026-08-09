/** 🧬️ SequenceConfig */
export interface SequenceConfig {
  /** @state local-ui */
  selectedStepIds: string[];
  /** @state local-ui */
  lastRunJson: string;
  /** @state local-ui */
  orientation: string;
  /** @state local-ui */
  camera: SequenceCamera;
  /** @state local-ui */
  locale: string;
}
export interface SequenceCamera { x: number; y: number; zoom: number; }
