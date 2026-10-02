/** 🌡️ `change-element-u` wire twin: the leaf payload `ChangeElementU`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementU {
  mutation: "changeElementU";
  elementId: string;
  newUValueWM2k: number;
}

export const parseChangeElementU: NormWireReader<ChangeElementU> = normWireObject<ChangeElementU>({ mutation: normWireRequired(normWireLiteral("changeElementU")), elementId: normWireRequired(normWireString), newUValueWM2k: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
