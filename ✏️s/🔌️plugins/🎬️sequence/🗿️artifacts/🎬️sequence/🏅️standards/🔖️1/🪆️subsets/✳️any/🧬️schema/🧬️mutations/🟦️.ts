/** 🎬️ Sequence mutation union assembled from direct payload owners. */
import type { CreateStep } from "./🌱create-step/🟦️";
import type { DeleteStep } from "./🗑️delete-step/🟦️";
import type { MoveStep } from "./📍move-step/🟦️";
import type { EditStepParams } from "./🔧edit-step-params/🟦️";
import type { ChangeStepCollapsed } from "./🗂️change-step-collapsed/🟦️";
import type { ConnectSteps } from "./🔗connect-steps/🟦️";
import type { DisconnectSteps } from "./✂️disconnect-steps/🟦️";
import type { DuplicateStep } from "./🧬duplicate-step/🟦️";

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
