/** ➖️ `remove-built-up-member` wire twin: the leaf payload `RemoveBuiltUpMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBuiltUpMember {
  index: number;
}

export const parseRemoveBuiltUpMember: NormWireReader<RemoveBuiltUpMember> = normWireObject<RemoveBuiltUpMember>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
