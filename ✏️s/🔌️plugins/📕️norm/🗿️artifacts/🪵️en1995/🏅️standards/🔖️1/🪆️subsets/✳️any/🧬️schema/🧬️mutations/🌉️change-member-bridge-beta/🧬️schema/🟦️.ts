/** 🌉️ `change-member-bridge-beta` wire twin: the leaf payload `ChangeMemberBridgeBeta`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBridgeBeta {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeBeta: NormWireReader<ChangeMemberBridgeBeta> = normWireObject<ChangeMemberBridgeBeta>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
