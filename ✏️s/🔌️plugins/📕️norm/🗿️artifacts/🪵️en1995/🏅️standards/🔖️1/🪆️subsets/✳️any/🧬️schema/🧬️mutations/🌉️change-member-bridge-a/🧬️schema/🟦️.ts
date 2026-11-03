/** 🌉️ `change-member-bridge-a` wire twin: the leaf payload `ChangeMemberBridgeA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBridgeA {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeA: NormWireReader<ChangeMemberBridgeA> = normWireObject<ChangeMemberBridgeA>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
