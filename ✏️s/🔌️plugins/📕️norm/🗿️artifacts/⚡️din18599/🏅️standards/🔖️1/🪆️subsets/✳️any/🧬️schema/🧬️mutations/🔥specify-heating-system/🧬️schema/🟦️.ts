/** 🔥 `specify-heating-system` wire twin: the leaf payload `SpecifyHeatingSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type HeatingSystem, parseHeatingSystem } from "../../../📸️snapshot/🟦️.ts";

export interface SpecifyHeatingSystem {
  mutation: "specifyHeatingSystem";
  newHeating: HeatingSystem;
}

export const parseSpecifyHeatingSystem: NormWireReader<SpecifyHeatingSystem> = normWireObject<SpecifyHeatingSystem>({ mutation: normWireRequired(normWireLiteral("specifyHeatingSystem")), newHeating: normWireRequired(parseHeatingSystem) });
