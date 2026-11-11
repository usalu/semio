/** 🔹 `change-element-fc` wire twin: the leaf payload `ChangeElementFc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementFc {
  mutation: "changeelementfc";
  elementId: string;
  newFc: number;
}

export const parseChangeElementFc: NormWireReader<ChangeElementFc> = normWireObject<ChangeElementFc>({ mutation: normWireRequired(normWireLiteral("changeelementfc")), elementId: normWireRequired(normWireString), newFc: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
