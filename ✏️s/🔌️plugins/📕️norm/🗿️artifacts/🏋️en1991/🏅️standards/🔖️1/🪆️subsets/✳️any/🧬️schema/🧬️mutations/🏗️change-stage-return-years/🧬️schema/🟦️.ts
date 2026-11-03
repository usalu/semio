/** 🏗️ `change-stage-return-years` wire twin: the leaf payload `ChangeStageReturnYears`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStageReturnYears {
  id: string;
  newReturnYears: number;
}

export const parseChangeStageReturnYears: NormWireReader<ChangeStageReturnYears> = normWireObject<ChangeStageReturnYears>({ id: normWireRequired(normWireString), newReturnYears: normWireRequired(normWireNumber) });
