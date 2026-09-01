/** ↩️ procedural3d connect-synapse/↩️inverse — mirror of the id-only disconnect-synapse inverse builder. */
import type { ConnectSynapse } from "../🦠️mutation/🟦️component.ts";
import type { DisconnectSynapse } from "../../✂️disconnect-synapse/🦠️mutation/🟦️component.ts";

export function inverse(payload: ConnectSynapse): DisconnectSynapse[] {
  return [{ id: payload.synapse.id }];
}
