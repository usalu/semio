/** 🔩️ `change-fastener-member` wire twin: the leaf payload `ChangeFastenerMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerMember {
  fastenerId: string;
  newValue: string;
}

export const parseChangeFastenerMember: NormWireReader<ChangeFastenerMember> = normWireObject<ChangeFastenerMember>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
