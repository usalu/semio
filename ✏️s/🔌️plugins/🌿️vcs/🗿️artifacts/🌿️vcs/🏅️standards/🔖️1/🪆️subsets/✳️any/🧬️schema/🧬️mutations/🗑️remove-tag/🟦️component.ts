/** 🗑️ Direct remove-tag payload and tagged wire identity. */
export interface RemoveTag {
  readonly tag: string;
}

export type RemoveTagMutation = RemoveTag & { readonly mutation: "removeTag" };
