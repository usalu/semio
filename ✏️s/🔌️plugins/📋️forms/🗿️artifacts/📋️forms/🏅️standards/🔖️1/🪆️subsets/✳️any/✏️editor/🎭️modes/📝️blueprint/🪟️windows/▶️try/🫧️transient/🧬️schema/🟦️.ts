export interface FormsTryWindowTransient {
  tryValues: Record<string, string[]>;
}

export interface FormsTryWindowTransientMutation {
  kind: "snapshot";
  transient: FormsTryWindowTransient;
}

export const applyFormsTryWindowTransientMutation = (_base: FormsTryWindowTransient, mutation: FormsTryWindowTransientMutation): FormsTryWindowTransient => structuredClone(mutation.transient);
