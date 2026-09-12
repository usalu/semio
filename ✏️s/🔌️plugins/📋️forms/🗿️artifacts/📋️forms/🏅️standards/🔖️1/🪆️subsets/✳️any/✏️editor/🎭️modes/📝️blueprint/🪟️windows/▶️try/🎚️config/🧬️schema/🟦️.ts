export interface FormsTryWindowConfig {
  currentStepIndex: number;
}

export interface FormsTryWindowConfigMutation {
  kind: "snapshot";
  config: FormsTryWindowConfig;
}

export const applyFormsTryWindowConfigMutation = (_base: FormsTryWindowConfig, mutation: FormsTryWindowConfigMutation): FormsTryWindowConfig => structuredClone(mutation.config);
