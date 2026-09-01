/** ↩️ procedural2d connect-synapse inverse — mirrors `inverse()` (…/🔗connect-synapse/↩️inverse/🦀️component.rs): always one `disconnect-synapse` for the created edge's id (no BASE lookup needed). */
import type { ConnectSynapse } from "../🦠️mutation/🟦️component.ts";
import type { DisconnectSynapse } from "../../✂️disconnect-synapse/🦠️mutation/🟦️component.ts";

export function inverse(payload: ConnectSynapse): DisconnectSynapse[] {
  return [{ id: payload.synapse.id }];
}
