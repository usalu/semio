/** 🏛️ `change-member-prestress-steel-id` wire twin: the leaf payload `ChangeMemberPrestressSteelId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPrestressSteelId {
  memberId: string;
  newPrestressSteelId: string;
}

export const parseChangeMemberPrestressSteelId: NormWireReader<ChangeMemberPrestressSteelId> = normWireObject<ChangeMemberPrestressSteelId>({ memberId: normWireRequired(normWireString), newPrestressSteelId: normWireRequired(normWireString) });
