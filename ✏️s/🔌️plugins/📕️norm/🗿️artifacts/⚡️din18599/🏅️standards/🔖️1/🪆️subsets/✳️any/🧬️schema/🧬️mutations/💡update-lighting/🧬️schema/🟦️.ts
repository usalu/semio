/** 💡 `update-lighting` wire twin: the leaf payload `UpdateLighting`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type LightingSystem, parseLightingSystem } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateLighting {
  mutation: "updateLighting";
  newLighting: LightingSystem;
}

export const parseUpdateLighting: NormWireReader<UpdateLighting> = normWireObject<UpdateLighting>({ mutation: normWireRequired(normWireLiteral("updateLighting")), newLighting: normWireRequired(parseLightingSystem) });
