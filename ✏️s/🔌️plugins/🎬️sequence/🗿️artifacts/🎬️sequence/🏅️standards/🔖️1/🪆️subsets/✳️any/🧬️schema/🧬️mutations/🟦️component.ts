/** 🎬️ Sequence mutation union assembled from direct payload owners. */
import type { CreateStep } from "./🌱create-step/🟦️component";
import type { DeleteStep } from "./🗑️delete-step/🟦️component";
import type { MoveStep } from "./📍move-step/🟦️component";
import type { EditStepParams } from "./🔧edit-step-params/🟦️component";
import type { ChangeStepCollapsed } from "./🗂️change-step-collapsed/🟦️component";
import type { ConnectSteps } from "./🔗connect-steps/🟦️component";
import type { DisconnectSteps } from "./✂️disconnect-steps/🟦️component";
import type { DuplicateStep } from "./🧬duplicate-step/🟦️component";

//#region 🧬️Aggregate
export type SequenceMutation =
  | ({ mutation: "createStep" } & CreateStep)
  | ({ mutation: "deleteStep" } & DeleteStep)
  | ({ mutation: "moveStep" } & MoveStep)
  | ({ mutation: "editStepParams" } & EditStepParams)
  | ({ mutation: "changeStepCollapsed" } & ChangeStepCollapsed)
  | ({ mutation: "connectSteps" } & ConnectSteps)
  | ({ mutation: "disconnectSteps" } & DisconnectSteps)
  | ({ mutation: "duplicateStep" } & DuplicateStep);
//#endregion 🧬️Aggregate
