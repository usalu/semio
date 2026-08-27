/** 🔢 Direct change-counter payload and tagged wire identity. */
export interface ChangeCounter {
  readonly newCounter: number;
}

export type ChangeCounterMutation = ChangeCounter & { readonly mutation: "changeCounter" };
