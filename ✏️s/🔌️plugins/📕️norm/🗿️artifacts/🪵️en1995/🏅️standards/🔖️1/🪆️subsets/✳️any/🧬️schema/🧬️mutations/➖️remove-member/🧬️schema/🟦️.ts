/** ➖️ `remove-member` wire twin: the leaf payload `RemoveMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMember {
  index: number;
}

export const parseRemoveMember: NormWireReader<RemoveMember> = normWireObject<RemoveMember>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
