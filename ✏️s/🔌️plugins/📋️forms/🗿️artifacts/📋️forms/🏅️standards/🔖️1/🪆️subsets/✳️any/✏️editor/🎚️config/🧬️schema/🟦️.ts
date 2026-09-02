/** 🧬️ FormsConfig */
export interface FormsConfig {
  /** @state config */
  currentStepIndex: number;
  /** @state config */
  tryValues: Record<string, string[]>;
  /** @state config */
  locale: string;
  /** @state config */
  contributionsJson: string;
}
