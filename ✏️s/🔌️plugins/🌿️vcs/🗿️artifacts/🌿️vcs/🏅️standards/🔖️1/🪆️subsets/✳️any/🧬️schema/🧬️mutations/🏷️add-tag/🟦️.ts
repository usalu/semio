/** 🏷️ Direct add-tag payload and tagged wire identity. */
export interface AddTag {
  readonly tag: string;
  readonly index?: number;
}

export type AddTagMutation = AddTag & { readonly mutation: "addTag" };
