/** 🚦 Direct change-status payload and tagged wire identity. */
export interface ChangeStatus {
  readonly newStatus: string;
}

export type ChangeStatusMutation = ChangeStatus & { readonly mutation: "changeStatus" };
