/** 🧱 `change-concrete-f-ck` wire twin: the leaf payload `ChangeConcreteFCk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConcreteFCk {
  gradeId: string;
  newFCk: number;
}

export const parseChangeConcreteFCk: NormWireReader<ChangeConcreteFCk> = normWireObject<ChangeConcreteFCk>({ gradeId: normWireRequired(normWireString), newFCk: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
