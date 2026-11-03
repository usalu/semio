/** ➖️ `remove-member-action` wire twin: the leaf payload `RemoveMemberAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMemberAction {
  index: number;
}

export const parseRemoveMemberAction: NormWireReader<RemoveMemberAction> = normWireObject<RemoveMemberAction>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
