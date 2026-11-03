/** ⚠️ `change-consequence-class` wire twin: the leaf payload `ChangeConsequenceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConsequenceClass {
  mutation: "changeConsequenceClass";
  newConsequenceClass: number;
}

export const parseChangeConsequenceClass: NormWireReader<ChangeConsequenceClass> = normWireObject<ChangeConsequenceClass>({ mutation: normWireRequired(normWireLiteral("changeConsequenceClass")), newConsequenceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
