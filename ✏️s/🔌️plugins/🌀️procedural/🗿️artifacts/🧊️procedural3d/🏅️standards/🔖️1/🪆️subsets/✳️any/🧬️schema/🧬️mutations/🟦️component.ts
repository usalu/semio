/** 🧬️ Procedural3d direct-mutation discriminated union — mirror of `Procedural3dMutation`. */
import type { ChangeGenerationValue } from "./🔧change-generation-value/🦠️mutation/🟦️component.ts";
import type { ChangeSchema } from "./🔤change-schema/🦠️mutation/🟦️component.ts";
import type { ConnectSynapse } from "./🔗connect-synapse/🦠️mutation/🟦️component.ts";
import type { CreateGeneration } from "./➕create-generation/🦠️mutation/🟦️component.ts";
import type { CreateWidget } from "./🌱create-widget/🦠️mutation/🟦️component.ts";
import type { DeleteGeneration } from "./🗑delete-generation/🦠️mutation/🟦️component.ts";
import type { DeleteWidget } from "./❌delete-widget/🦠️mutation/🟦️component.ts";
import type { DeleteWidgetPosition } from "./🧹delete-widget-position/🦠️mutation/🟦️component.ts";
import type { DisconnectSynapse } from "./✂️disconnect-synapse/🦠️mutation/🟦️component.ts";
import type { MoveWidget } from "./📍move-widget/🦠️mutation/🟦️component.ts";
import type { RenameGeneration } from "./🏷rename-generation/🦠️mutation/🟦️component.ts";
import type { UpdateCamera } from "./📷update-camera/🦠️mutation/🟦️component.ts";
import type { UpdateSynapse } from "./🔄update-synapse/🦠️mutation/🟦️component.ts";
import type { UpdateWidget } from "./🩹update-widget/🦠️mutation/🟦️component.ts";

export type Procedural3dMutation =
  | ({ mutation: "createWidget" } & CreateWidget)
  | ({ mutation: "updateWidget" } & UpdateWidget)
  | ({ mutation: "deleteWidget" } & DeleteWidget)
  | ({ mutation: "connectSynapse" } & ConnectSynapse)
  | ({ mutation: "updateSynapse" } & UpdateSynapse)
  | ({ mutation: "disconnectSynapse" } & DisconnectSynapse)
  | ({ mutation: "moveWidget" } & MoveWidget)
  | ({ mutation: "deleteWidgetPosition" } & DeleteWidgetPosition)
  | ({ mutation: "updateCamera" } & UpdateCamera)
  | ({ mutation: "changeSchema" } & ChangeSchema)
  | ({ mutation: "createGeneration" } & CreateGeneration)
  | ({ mutation: "deleteGeneration" } & DeleteGeneration)
  | ({ mutation: "renameGeneration" } & RenameGeneration)
  | ({ mutation: "changeGenerationValue" } & ChangeGenerationValue);
