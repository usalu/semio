/** 🔄️ `change-column-action-moment-nm` wire twin: the leaf payload `ChangeColumnActionMomentNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnActionMomentNm {
  columnId: string;
  id: string;
  newMKNm: number;
}

export const parseChangeColumnActionMomentNm: NormWireReader<ChangeColumnActionMomentNm> = normWireObject<ChangeColumnActionMomentNm>({ columnId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMKNm: normWireRequired(normWireNumber) });
