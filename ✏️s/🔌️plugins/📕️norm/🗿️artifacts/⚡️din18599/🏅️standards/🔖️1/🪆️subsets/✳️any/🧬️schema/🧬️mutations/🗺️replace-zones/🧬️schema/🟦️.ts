/** 🗺️ `replace-zones` wire twin: the leaf payload `ReplaceZones`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseThermalZone, type ThermalZone } from "../../../📸️snapshot/🟦️.ts";

export interface ReplaceZones {
  mutation: "replaceZones";
  newZones: ThermalZone[];
}

export const parseReplaceZones: NormWireReader<ReplaceZones> = normWireObject<ReplaceZones>({ mutation: normWireRequired(normWireLiteral("replaceZones")), newZones: normWireRequired(normWireArray(parseThermalZone)) });
