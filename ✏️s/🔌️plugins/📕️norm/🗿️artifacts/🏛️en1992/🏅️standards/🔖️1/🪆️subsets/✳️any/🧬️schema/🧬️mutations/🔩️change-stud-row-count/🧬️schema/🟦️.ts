/** 🔩 `change-stud-row-count` wire twin: the leaf payload `ChangeStudRowCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudRowCount {
  memberId: string;
  studRowId: string;
  newCount: number;
}

export const parseChangeStudRowCount: NormWireReader<ChangeStudRowCount> = normWireObject<ChangeStudRowCount>({ memberId: normWireRequired(normWireString), studRowId: normWireRequired(normWireString), newCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
