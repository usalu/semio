/** 🏛️ `change-member-prestress-area` wire twin: the leaf payload `ChangeMemberPrestressArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPrestressArea {
  memberId: string;
  newPrestressArea: number;
}

export const parseChangeMemberPrestressArea: NormWireReader<ChangeMemberPrestressArea> = normWireObject<ChangeMemberPrestressArea>({ memberId: normWireRequired(normWireString), newPrestressArea: normWireRequired(normWireNumber) });
