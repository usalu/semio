/** ↩️ procedural3d connect-synapse/↩️inverse — mirror of the id-only disconnect-synapse inverse builder. */
import type { ConnectSynapse } from "../🦠️mutation/🟦️.ts";
import type { DisconnectSynapse } from "../../✂️disconnect-synapse/🦠️mutation/🟦️.ts";

export function inverse(payload: ConnectSynapse): DisconnectSynapse[] {
  return [{ id: payload.synapse.id }];
}
