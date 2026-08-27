/** 📝 Direct change-notes payload and tagged wire identity. */
export interface ChangeNotes {
  readonly newNotes: string;
}

export type ChangeNotesMutation = ChangeNotes & { readonly mutation: "changeNotes" };
