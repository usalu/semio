/** 📐️ `change-column-second-moment-m4` wire twin: the leaf payload `ChangeColumnSecondMomentM4`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSecondMomentM4 {
  id: string;
  newIM4: number;
}

export const parseChangeColumnSecondMomentM4: NormWireReader<ChangeColumnSecondMomentM4> = normWireObject<ChangeColumnSecondMomentM4>({ id: normWireRequired(normWireString), newIM4: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
