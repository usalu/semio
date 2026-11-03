/** 🏛️ `change-member-punch-column-depth` wire twin: the leaf payload `ChangeMemberPunchColumnDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPunchColumnDepth {
  memberId: string;
  newPunchingColumnDepth: number;
}

export const parseChangeMemberPunchColumnDepth: NormWireReader<ChangeMemberPunchColumnDepth> = normWireObject<ChangeMemberPunchColumnDepth>({ memberId: normWireRequired(normWireString), newPunchingColumnDepth: normWireRequired(normWireNumber) });
