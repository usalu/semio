/** ➖️ `remove-cutout` wire twin: the leaf payload `RemoveCutout`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCutout {
  index: number;
}

export const parseRemoveCutout: NormWireReader<RemoveCutout> = normWireObject<RemoveCutout>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
