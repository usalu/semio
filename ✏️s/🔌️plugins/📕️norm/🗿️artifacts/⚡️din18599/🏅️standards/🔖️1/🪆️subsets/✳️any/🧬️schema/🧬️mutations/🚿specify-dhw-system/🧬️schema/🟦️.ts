/** 🚿 `specify-dhw-system` wire twin: the leaf payload `SpecifyDhwSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type DhwSystem, parseDhwSystem } from "../../../📸️snapshot/🟦️.ts";

export interface SpecifyDhwSystem {
  mutation: "specifyDhwSystem";
  newDhw: DhwSystem;
}

export const parseSpecifyDhwSystem: NormWireReader<SpecifyDhwSystem> = normWireObject<SpecifyDhwSystem>({ mutation: normWireRequired(normWireLiteral("specifyDhwSystem")), newDhw: normWireRequired(parseDhwSystem) });
