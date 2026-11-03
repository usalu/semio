/** 🪜 `change-member-udl` wire twin: the leaf payload `ChangeMemberUdl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberUdl {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberUdl: NormWireReader<ChangeMemberUdl> = normWireObject<ChangeMemberUdl>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
