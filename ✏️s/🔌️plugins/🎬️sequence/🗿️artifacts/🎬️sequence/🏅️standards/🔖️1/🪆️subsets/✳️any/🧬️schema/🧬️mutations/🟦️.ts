/** 🎬️ Sequence mutation union assembled from direct payload owners. */
import type { CreateStep } from "../../../✳️step/🧬️schema/🧬️mutations/🌱create-step/🟦️";
import type { DeleteStep } from "../../../✳️step/🧬️schema/🧬️mutations/🗑️delete-step/🟦️";
import type { MoveStep } from "../../../✳️step/🧬️schema/🧬️mutations/📍move-step/🟦️";
import type { EditStepParams } from "../../../✳️step/🧬️schema/🧬️mutations/🔧edit-step-params/🟦️";
import type { ChangeStepCollapsed } from "../../../✳️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🟦️";
import type { ConnectSteps } from "../../../✳️dependency/🧬️schema/🧬️mutations/🔗connect-steps/🟦️";
import type { DisconnectSteps } from "../../../✳️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🟦️";
import type { DuplicateStep } from "../../../✳️step/🧬️schema/🧬️mutations/🧬duplicate-step/🟦️";

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
