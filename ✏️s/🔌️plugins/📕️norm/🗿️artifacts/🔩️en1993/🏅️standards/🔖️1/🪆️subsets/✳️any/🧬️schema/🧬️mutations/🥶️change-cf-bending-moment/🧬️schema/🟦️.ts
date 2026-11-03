/** 🥶️ `change-cf-bending-moment` wire twin: the leaf payload `ChangeCfBendingMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfBendingMoment {
  id: string;
  newBendingMoment: number;
}

export const parseChangeCfBendingMoment: NormWireReader<ChangeCfBendingMoment> = normWireObject<ChangeCfBendingMoment>({ id: normWireRequired(normWireString), newBendingMoment: normWireRequired(normWireNumber) });
