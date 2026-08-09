/** @emoji 📸️ Forms snapshot — persistent fields only. */
export interface FormsSnapshot {
  schema: string;
  id: string;
  version: string;
  title?: string | null;
  steps: FormStep[];
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
