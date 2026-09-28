export interface FormsTryWindowConfig {
  currentStepIndex: number; submittedResponseId?: string | null;
}

export interface FormsTryWindowConfigMutation {
  kind: "snapshot";
  config: FormsTryWindowConfig;
}

export const applyFormsTryWindowConfigMutation = (_base: FormsTryWindowConfig, mutation: FormsTryWindowConfigMutation): FormsTryWindowConfig => structuredClone(mutation.config);
