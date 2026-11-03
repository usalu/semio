/** ➖️ `remove-foundation-block` wire twin: the leaf payload `RemoveFoundationBlock`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFoundationBlock {
  index: number;
}

export const parseRemoveFoundationBlock: NormWireReader<RemoveFoundationBlock> = normWireObject<RemoveFoundationBlock>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
