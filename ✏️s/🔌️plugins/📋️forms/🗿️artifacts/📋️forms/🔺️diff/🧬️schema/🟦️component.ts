/** @emoji 🔺️ Sparse field delta over the forms artifact. */
export interface FormsDiff {
  artifact?: FormsArtifact;
  schema?: string;
  id?: string;
  version?: string;
  title?: string | null;
  steps?: FormsStepsDelta;
  selectedIds?: FormsStringList;
  currentStepIndex?: number;
  tryValuesJson?: string;
  locale?: string;
  contributionsJson?: string;
}

export interface FormsStringList {
  values: string[];
}

export interface FormsStepsDelta {
  added?: FormStep[];
  removed?: string[];
  patched?: FormsStepPatchEntry[];
  reordered?: string[];
}

export interface FormsStepPatchEntry {
  id: string;
  patch: FormsStepPatch;
}

export interface FormsStepPatch {
  title?: string;
  description?: string;
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
