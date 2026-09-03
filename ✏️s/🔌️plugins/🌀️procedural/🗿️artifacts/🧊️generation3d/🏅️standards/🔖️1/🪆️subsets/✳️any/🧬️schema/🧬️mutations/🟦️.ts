/** 🧬️ Generation3d direct-mutation discriminated union — mirror of `Generation3dMutation`. */
import type { ChangeGenerationValue } from "./🔧change-generation-value/🦠️mutation/🟦️.ts";
import type { ChangeSchema } from "./🔤change-schema/🦠️mutation/🟦️.ts";
import type { ConnectSynapse } from "./🔗connect-synapse/🦠️mutation/🟦️.ts";
import type { CreateGeneration } from "./➕create-generation/🦠️mutation/🟦️.ts";
import type { CreateWidget } from "./🌱create-widget/🦠️mutation/🟦️.ts";
import type { DeleteGeneration } from "./🗑delete-generation/🦠️mutation/🟦️.ts";
import type { DeleteWidget } from "./❌delete-widget/🦠️mutation/🟦️.ts";
import type { DeleteWidgetPosition } from "./🧹delete-widget-position/🦠️mutation/🟦️.ts";
import type { DisconnectSynapse } from "./✂️disconnect-synapse/🦠️mutation/🟦️.ts";
import type { MoveWidget } from "./📍move-widget/🦠️mutation/🟦️.ts";
import type { RenameGeneration } from "./🏷rename-generation/🦠️mutation/🟦️.ts";
import type { UpdateCamera } from "./📷update-camera/🦠️mutation/🟦️.ts";
import type { UpdateSynapse } from "./🔄update-synapse/🦠️mutation/🟦️.ts";
import type { UpdateWidget } from "./🩹update-widget/🦠️mutation/🟦️.ts";

export type Generation3dMutation =
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
