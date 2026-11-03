/** 🏛️ `change-member-prestress-force` wire twin: the leaf payload `ChangeMemberPrestressForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPrestressForce {
  memberId: string;
  newPrestressForce: number;
}

export const parseChangeMemberPrestressForce: NormWireReader<ChangeMemberPrestressForce> = normWireObject<ChangeMemberPrestressForce>({ memberId: normWireRequired(normWireString), newPrestressForce: normWireRequired(normWireNumber) });
