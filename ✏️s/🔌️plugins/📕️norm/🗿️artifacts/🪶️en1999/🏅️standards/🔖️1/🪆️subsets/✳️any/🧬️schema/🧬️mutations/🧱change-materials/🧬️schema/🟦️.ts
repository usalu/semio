/** 🧱 `change-materials` wire twin: the leaf payload `ChangeMaterials`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumMaterial, parseAluminiumMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMaterials {
  mutation: "changeMaterials";
  materials: AluminiumMaterial[];
}

export const parseChangeMaterials: NormWireReader<ChangeMaterials> = normWireObject<ChangeMaterials>({ mutation: normWireRequired(normWireLiteral("changeMaterials")), materials: normWireRequired(normWireArray(parseAluminiumMaterial)) });
