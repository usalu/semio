/** ↩️ generation3d update-synapse/↩️inverse — mirror of the BASE-lookup whole-body restore inverse. */
import type { UpdateSynapse } from "../🦠️mutation/🟦️.ts";
import type { SynapseSpec } from "../../🔗connect-synapse/🦠️mutation/🟦️.ts";

export function inverse(_payload: UpdateSynapse, baseSynapse: SynapseSpec | undefined): UpdateSynapse[] {
  return baseSynapse === undefined ? [] : [{ synapse: baseSynapse }];
}
