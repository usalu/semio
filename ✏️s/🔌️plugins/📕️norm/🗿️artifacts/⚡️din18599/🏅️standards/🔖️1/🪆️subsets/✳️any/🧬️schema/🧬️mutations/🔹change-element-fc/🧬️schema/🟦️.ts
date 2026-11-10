/** 🔹 `change-element-fc` wire twin: the leaf payload `ChangeElementFc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementFc {
  mutation: "changeelementfc";
  elementId: string;
  newFc: number;
}

export const parseChangeElementFc: NormWireReader<ChangeElementFc> = normWireObject<ChangeElementFc>({ mutation: normWireRequired(normWireLiteral("changeelementfc")), elementId: normWireRequired(normWireString), newFc: normWireRequired(normWireRange(normWireNumber, {"minimum":0.0,"maximum":1.0})) });
