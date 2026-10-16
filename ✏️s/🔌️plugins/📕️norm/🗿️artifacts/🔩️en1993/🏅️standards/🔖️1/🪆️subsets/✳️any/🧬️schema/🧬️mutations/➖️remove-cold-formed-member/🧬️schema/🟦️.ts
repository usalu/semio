/** ➖️ `remove-cold-formed-member` wire twin: the leaf payload `RemoveColdFormedMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";

export interface RemoveColdFormedMember {
  index: number;
}

export const parseRemoveColdFormedMember: NormWireReader<RemoveColdFormedMember> = normWireObject<RemoveColdFormedMember>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
