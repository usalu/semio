/** ➖️ `remove-taper` wire twin: the leaf payload `RemoveTaper`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTaper {
  index: number;
}

export const parseRemoveTaper: NormWireReader<RemoveTaper> = normWireObject<RemoveTaper>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
