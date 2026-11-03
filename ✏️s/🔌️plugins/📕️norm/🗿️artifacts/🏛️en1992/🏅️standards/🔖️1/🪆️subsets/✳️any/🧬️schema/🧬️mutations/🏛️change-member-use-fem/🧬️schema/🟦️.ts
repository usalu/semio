/** 🏛️ `change-member-use-fem` wire twin: the leaf payload `ChangeMemberUseFem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberUseFem {
  memberId: string;
  newUseFem: boolean;
}

export const parseChangeMemberUseFem: NormWireReader<ChangeMemberUseFem> = normWireObject<ChangeMemberUseFem>({ memberId: normWireRequired(normWireString), newUseFem: normWireRequired(normWireBoolean) });
