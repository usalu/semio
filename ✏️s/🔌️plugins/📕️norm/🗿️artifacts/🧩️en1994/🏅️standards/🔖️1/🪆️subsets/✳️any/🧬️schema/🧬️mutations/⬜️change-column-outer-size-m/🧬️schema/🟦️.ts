/** ⬜️ `change-column-outer-size-m` wire twin: the leaf payload `ChangeColumnOuterSizeM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnOuterSizeM {
  id: string;
  newOuterSizeM: number;
}

export const parseChangeColumnOuterSizeM: NormWireReader<ChangeColumnOuterSizeM> = normWireObject<ChangeColumnOuterSizeM>({ id: normWireRequired(normWireString), newOuterSizeM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
