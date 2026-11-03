/** 🏛️ `change-member-label-de` wire twin: the leaf payload `ChangeMemberLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLabelDe {
  memberId: string;
  newLabelDe: string;
}

export const parseChangeMemberLabelDe: NormWireReader<ChangeMemberLabelDe> = normWireObject<ChangeMemberLabelDe>({ memberId: normWireRequired(normWireString), newLabelDe: normWireRequired(normWireString) });
