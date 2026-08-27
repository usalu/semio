/** ✏️ Direct rename-vcs payload and tagged wire identity. */
export interface RenameVcs {
  readonly newTitle: string;
}

export type RenameVcsMutation = RenameVcs & { readonly mutation: "renameVcs" };
