/** 🔀️ `change-member-order` wire twin: the leaf payload `ChangeMemberOrder`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberOrder {
  fromIndex: number;
  toIndex: number;
}

export const parseChangeMemberOrder: NormWireReader<ChangeMemberOrder> = normWireObject<ChangeMemberOrder>({ fromIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), toIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
