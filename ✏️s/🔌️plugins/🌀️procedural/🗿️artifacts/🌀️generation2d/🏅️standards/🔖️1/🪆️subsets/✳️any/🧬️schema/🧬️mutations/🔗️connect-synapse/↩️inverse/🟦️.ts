/** ↩️ generation2d connect-synapse inverse — mirrors `inverse()` (…/🔗️connect-synapse/↩️inverse/🦀️.rs): always one `disconnect-synapse` for the created edge's id (no BASE lookup needed). */
import type { ConnectSynapse } from "../🦠️mutation/🟦️.ts";
import type { DisconnectSynapse } from "../../✂️disconnect-synapse/🦠️mutation/🟦️.ts";

export function inverse(payload: ConnectSynapse): DisconnectSynapse[] {
  return [{ id: payload.synapse.id }];
}
