/** 🔩 `change-stud-row-angle` wire twin: the leaf payload `ChangeStudRowAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudRowAngle {
  memberId: string;
  studRowId: string;
  newAngle: number;
}

export const parseChangeStudRowAngle: NormWireReader<ChangeStudRowAngle> = normWireObject<ChangeStudRowAngle>({ memberId: normWireRequired(normWireString), studRowId: normWireRequired(normWireString), newAngle: normWireRequired(normWireNumber) });
