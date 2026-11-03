/** 🎻 `change-prestress-loss` wire twin: the leaf payload `ChangePrestressLoss`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressLoss {
  wallId: string;
  id: string;
  newLossRatio: number;
}

export const parseChangePrestressLoss: NormWireReader<ChangePrestressLoss> = normWireObject<ChangePrestressLoss>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newLossRatio: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
