/** 🌿️ Transparent VCS mutation assembly. */
import type { RenameVcsMutation } from "./✏️rename-vcs/🟦️";
import type { ChangeCounterMutation } from "./🔢change-counter/🟦️";
import type { ChangeNotesMutation } from "./📝change-notes/🟦️";
import type { ChangeStatusMutation } from "./🚦change-status/🟦️";
import type { AddTagMutation } from "./🏷️add-tag/🟦️";
import type { RemoveTagMutation } from "./🗑️remove-tag/🟦️";

export type VcsMutation = RenameVcsMutation | ChangeCounterMutation | ChangeNotesMutation | ChangeStatusMutation | AddTagMutation | RemoveTagMutation;
export const VCS_MUTATION_KINDS = ["rename-vcs","change-counter","change-notes","change-status","add-tag","remove-tag"] as const;
