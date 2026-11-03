/** 📏️ `change-member-length` wire twin: the leaf payload `ChangeMemberLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLength {
  id: string;
  newLength: number;
}

export const parseChangeMemberLength: NormWireReader<ChangeMemberLength> = normWireObject<ChangeMemberLength>({ id: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
