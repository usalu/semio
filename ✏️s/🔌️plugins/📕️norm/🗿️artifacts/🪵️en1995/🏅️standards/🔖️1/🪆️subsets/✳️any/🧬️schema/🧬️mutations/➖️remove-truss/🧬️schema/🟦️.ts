/** ➖️ `remove-truss` wire twin: the leaf payload `RemoveTruss`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTruss {
  index: number;
}

export const parseRemoveTruss: NormWireReader<RemoveTruss> = normWireObject<RemoveTruss>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
