/** 🔩 `change-stud-row-spacing-first` wire twin: the leaf payload `ChangeStudRowSpacingFirst`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStudRowSpacingFirst {
  memberId: string;
  studRowId: string;
  newSpacingFirst: number;
}

export const parseChangeStudRowSpacingFirst: NormWireReader<ChangeStudRowSpacingFirst> = normWireObject<ChangeStudRowSpacingFirst>({ memberId: normWireRequired(normWireString), studRowId: normWireRequired(normWireString), newSpacingFirst: normWireRequired(normWireNumber) });
