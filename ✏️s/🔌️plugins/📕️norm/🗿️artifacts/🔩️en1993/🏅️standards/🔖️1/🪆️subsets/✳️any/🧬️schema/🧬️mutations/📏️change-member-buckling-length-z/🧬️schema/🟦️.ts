/** 📏️ `change-member-buckling-length-z` wire twin: the leaf payload `ChangeMemberBucklingLengthZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBucklingLengthZ {
  id: string;
  newBucklingLengthZ: number;
}

export const parseChangeMemberBucklingLengthZ: NormWireReader<ChangeMemberBucklingLengthZ> = normWireObject<ChangeMemberBucklingLengthZ>({ id: normWireRequired(normWireString), newBucklingLengthZ: normWireRequired(normWireNumber) });
