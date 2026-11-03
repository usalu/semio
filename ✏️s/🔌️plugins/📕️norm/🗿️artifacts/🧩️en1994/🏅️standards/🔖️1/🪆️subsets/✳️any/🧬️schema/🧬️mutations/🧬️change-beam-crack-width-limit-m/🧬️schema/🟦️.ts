/** 🧬️ `change-beam-crack-width-limit-m` wire twin: the leaf payload `ChangeBeamCrackWidthLimitM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamCrackWidthLimitM {
  id: string;
  newWkLimitM: number;
}

export const parseChangeBeamCrackWidthLimitM: NormWireReader<ChangeBeamCrackWidthLimitM> = normWireObject<ChangeBeamCrackWidthLimitM>({ id: normWireRequired(normWireString), newWkLimitM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
