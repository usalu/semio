/** 🔩 `change-stud-row-diameter` wire twin: the leaf payload `ChangeStudRowDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudRowDiameter {
  memberId: string;
  studRowId: string;
  newDiameter: number;
}

export const parseChangeStudRowDiameter: NormWireReader<ChangeStudRowDiameter> = normWireObject<ChangeStudRowDiameter>({ memberId: normWireRequired(normWireString), studRowId: normWireRequired(normWireString), newDiameter: normWireRequired(normWireNumber) });
