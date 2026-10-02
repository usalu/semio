/** 🧱 `change-concrete-f-ck` wire twin: the leaf payload `ChangeConcreteFCk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConcreteFCk {
  gradeId: string;
  newFCk: number;
}

export const parseChangeConcreteFCk: NormWireReader<ChangeConcreteFCk> = normWireObject<ChangeConcreteFCk>({ gradeId: normWireRequired(normWireString), newFCk: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
