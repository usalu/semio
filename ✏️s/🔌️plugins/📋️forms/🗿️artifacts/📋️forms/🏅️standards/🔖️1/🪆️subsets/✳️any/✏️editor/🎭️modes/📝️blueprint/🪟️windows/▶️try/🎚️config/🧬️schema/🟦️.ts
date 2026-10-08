export interface FormsTryWindowConfig {
  currentStepIndex: number; submittedResponseId?: string | null;
}

export type FormsTryWindowConfigMutation =
  | { kind: "set-current-step-index"; value: number }
  | { kind: "set-submitted-response-id"; value: string | null };

export const applyFormsTryWindowConfigMutation = (base: FormsTryWindowConfig, mutation: FormsTryWindowConfigMutation): FormsTryWindowConfig => {
  const next = structuredClone(base);
  if (mutation.kind === "set-current-step-index") next.currentStepIndex = mutation.value;
  else if (mutation.value === null) delete next.submittedResponseId;
  else next.submittedResponseId = mutation.value;
  return next;
};
