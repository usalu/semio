/** 🌬️ `update-ventilation` wire twin: the leaf payload `UpdateVentilation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseVentilationSystem, type VentilationSystem } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateVentilation {
  mutation: "updateVentilation";
  newVentilation: VentilationSystem;
}

export const parseUpdateVentilation: NormWireReader<UpdateVentilation> = normWireObject<UpdateVentilation>({ mutation: normWireRequired(normWireLiteral("updateVentilation")), newVentilation: normWireRequired(parseVentilationSystem) });
