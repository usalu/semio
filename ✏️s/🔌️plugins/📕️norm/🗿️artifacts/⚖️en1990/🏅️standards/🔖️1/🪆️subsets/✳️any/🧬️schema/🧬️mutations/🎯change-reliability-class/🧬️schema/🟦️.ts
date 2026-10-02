/** 🎯 `change-reliability-class` wire twin: the leaf payload `ChangeReliabilityClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeReliabilityClass {
  mutation: "changeReliabilityClass";
  newReliabilityClass: number;
}

export const parseChangeReliabilityClass: NormWireReader<ChangeReliabilityClass> = normWireObject<ChangeReliabilityClass>({ mutation: normWireRequired(normWireLiteral("changeReliabilityClass")), newReliabilityClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
