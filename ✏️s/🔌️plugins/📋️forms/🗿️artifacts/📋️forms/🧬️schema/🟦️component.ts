/** @emoji 🧬️ Forms artifact schema — persistent, shared-ui and local-ui fields. */
export interface FormsArtifact {
  schema: string;
  id: string;
  version: string;
  title?: string | null;
  steps: FormStep[];
  selectedIds: string[];
  currentStepIndex: number;
  tryValuesJson: string;
  locale: string;
  contributionsJson: string;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  [key: string]: unknown;
}
