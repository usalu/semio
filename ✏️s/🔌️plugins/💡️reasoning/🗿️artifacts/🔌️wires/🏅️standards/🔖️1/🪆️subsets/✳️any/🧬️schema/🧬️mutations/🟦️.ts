/** 🧩️ Wires mutation union references its leaf-owned payload contracts. */

import type { DisconnectNodes } from "./✂️disconnect-nodes/🧬️schema/🟦️.ts";
import type { EditNodeText } from "./✏️edit-node-text/🧬️schema/🟦️.ts";
import type { CreateNode } from "./🌱create-node/🧬️schema/🟦️.ts";
import type { ChangeNodeKind } from "./🏷️change-node-kind/🧬️schema/🟦️.ts";
import type { ResizeNode } from "./📐resize-node/🧬️schema/🟦️.ts";
import type { ChangeNodeShape } from "./🔷change-node-shape/🧬️schema/🟦️.ts";
import type { DeleteNode } from "./🗑️delete-node/🧬️schema/🟦️.ts";
import type { SetNodeRoot } from "./🚩set-node-root/🧬️schema/🟦️.ts";
import type { ConnectNodes } from "./🤝️connect-nodes/🧬️schema/🟦️.ts";
import type { MoveNode } from "./🧭move-node/🧬️schema/🟦️.ts";

export type { DisconnectNodes } from "./✂️disconnect-nodes/🧬️schema/🟦️.ts";
export type { EditNodeText } from "./✏️edit-node-text/🧬️schema/🟦️.ts";
export type { CreateNode } from "./🌱create-node/🧬️schema/🟦️.ts";
export type { ChangeNodeKind } from "./🏷️change-node-kind/🧬️schema/🟦️.ts";
export type { ResizeNode } from "./📐resize-node/🧬️schema/🟦️.ts";
export type { ChangeNodeShape } from "./🔷change-node-shape/🧬️schema/🟦️.ts";
export type { DeleteNode } from "./🗑️delete-node/🧬️schema/🟦️.ts";
export type { SetNodeRoot } from "./🚩set-node-root/🧬️schema/🟦️.ts";
export type { ConnectNodes } from "./🤝️connect-nodes/🧬️schema/🟦️.ts";
export type { MoveNode } from "./🧭move-node/🧬️schema/🟦️.ts";

export type WiresMutation =
  | ({ mutation: "disconnectNodes" } & DisconnectNodes)
  | ({ mutation: "editNodeText" } & EditNodeText)
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "changeNodeKind" } & ChangeNodeKind)
  | ({ mutation: "resizeNode" } & ResizeNode)
  | ({ mutation: "changeNodeShape" } & ChangeNodeShape)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "setNodeRoot" } & SetNodeRoot)
  | ({ mutation: "connectNodes" } & ConnectNodes)
  | ({ mutation: "moveNode" } & MoveNode);
