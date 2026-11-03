/** ➖️ `remove-frame-action` wire twin: the leaf payload `RemoveFrameAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFrameAction {
  index: number;
}

export const parseRemoveFrameAction: NormWireReader<RemoveFrameAction> = normWireObject<RemoveFrameAction>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
