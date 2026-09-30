import type { DslValue } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

/** 🧬️ ConnectNodes payload owned by the connect-nodes mutation. */
export interface ConnectNodes {
  mutation: "connectNodes";
  edge: DslValue;
  relationship: DslValue;
}
