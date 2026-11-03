/** 🏛️ `change-member-deflection-sensitive` wire twin: the leaf payload `ChangeMemberDeflectionSensitive`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDeflectionSensitive {
  memberId: string;
  newDeflectionSensitive: boolean;
}

export const parseChangeMemberDeflectionSensitive: NormWireReader<ChangeMemberDeflectionSensitive> = normWireObject<ChangeMemberDeflectionSensitive>({ memberId: normWireRequired(normWireString), newDeflectionSensitive: normWireRequired(normWireBoolean) });
