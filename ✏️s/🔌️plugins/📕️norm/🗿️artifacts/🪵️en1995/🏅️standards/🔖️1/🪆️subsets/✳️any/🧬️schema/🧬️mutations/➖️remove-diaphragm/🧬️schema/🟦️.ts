/** ➖️ `remove-diaphragm` wire twin: the leaf payload `RemoveDiaphragm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveDiaphragm {
  index: number;
}

export const parseRemoveDiaphragm: NormWireReader<RemoveDiaphragm> = normWireObject<RemoveDiaphragm>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
