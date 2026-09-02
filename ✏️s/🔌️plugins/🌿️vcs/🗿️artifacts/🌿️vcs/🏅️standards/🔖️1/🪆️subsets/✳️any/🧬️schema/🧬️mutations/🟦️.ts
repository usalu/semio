/** 🌿️ Transparent VCS mutation assembly. */
import type { RenameVcsMutation } from "./✏️rename-vcs/🟦️component";
import type { ChangeCounterMutation } from "./🔢change-counter/🟦️component";
import type { ChangeNotesMutation } from "./📝change-notes/🟦️component";
import type { ChangeStatusMutation } from "./🚦change-status/🟦️component";
import type { AddTagMutation } from "./🏷️add-tag/🟦️component";
import type { RemoveTagMutation } from "./🗑️remove-tag/🟦️component";

export type VcsMutation = RenameVcsMutation | ChangeCounterMutation | ChangeNotesMutation | ChangeStatusMutation | AddTagMutation | RemoveTagMutation;
export const VCS_MUTATION_KINDS = ["rename-vcs","change-counter","change-notes","change-status","add-tag","remove-tag"] as const;
