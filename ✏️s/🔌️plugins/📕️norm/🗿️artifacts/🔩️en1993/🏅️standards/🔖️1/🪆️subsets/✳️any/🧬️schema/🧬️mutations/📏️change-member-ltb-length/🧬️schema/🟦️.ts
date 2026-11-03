/** 📏️ `change-member-ltb-length` wire twin: the leaf payload `ChangeMemberLtbLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLtbLength {
  id: string;
  newLtbLength: number;
}

export const parseChangeMemberLtbLength: NormWireReader<ChangeMemberLtbLength> = normWireObject<ChangeMemberLtbLength>({ id: normWireRequired(normWireString), newLtbLength: normWireRequired(normWireNumber) });
