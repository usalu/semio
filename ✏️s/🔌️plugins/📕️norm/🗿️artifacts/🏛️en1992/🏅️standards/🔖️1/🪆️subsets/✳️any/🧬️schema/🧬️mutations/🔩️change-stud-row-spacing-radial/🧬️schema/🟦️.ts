/** 🔩 `change-stud-row-spacing-radial` wire twin: the leaf payload `ChangeStudRowSpacingRadial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudRowSpacingRadial {
  memberId: string;
  studRowId: string;
  newSpacingRadial: number;
}

export const parseChangeStudRowSpacingRadial: NormWireReader<ChangeStudRowSpacingRadial> = normWireObject<ChangeStudRowSpacingRadial>({ memberId: normWireRequired(normWireString), studRowId: normWireRequired(normWireString), newSpacingRadial: normWireRequired(normWireNumber) });
