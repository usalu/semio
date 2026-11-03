/** 📏️ `change-member-buckling-length-y` wire twin: the leaf payload `ChangeMemberBucklingLengthY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBucklingLengthY {
  id: string;
  newBucklingLengthY: number;
}

export const parseChangeMemberBucklingLengthY: NormWireReader<ChangeMemberBucklingLengthY> = normWireObject<ChangeMemberBucklingLengthY>({ id: normWireRequired(normWireString), newBucklingLengthY: normWireRequired(normWireNumber) });
