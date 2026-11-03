/** ➖️ `remove-frame-bar` wire twin: the leaf payload `RemoveFrameBar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFrameBar {
  index: number;
}

export const parseRemoveFrameBar: NormWireReader<RemoveFrameBar> = normWireObject<RemoveFrameBar>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
