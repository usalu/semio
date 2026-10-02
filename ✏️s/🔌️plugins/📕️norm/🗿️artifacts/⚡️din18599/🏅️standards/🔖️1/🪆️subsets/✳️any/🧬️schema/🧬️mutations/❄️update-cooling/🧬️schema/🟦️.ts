/** ❄️ `update-cooling` wire twin: the leaf payload `UpdateCooling`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CoolingSystem, parseCoolingSystem } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateCooling {
  mutation: "updateCooling";
  newCooling: CoolingSystem;
}

export const parseUpdateCooling: NormWireReader<UpdateCooling> = normWireObject<UpdateCooling>({ mutation: normWireRequired(normWireLiteral("updateCooling")), newCooling: normWireRequired(parseCoolingSystem) });
