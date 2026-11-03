/** ➖️ `remove-buckling-length` wire twin: the leaf payload `RemoveBucklingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBucklingLength {
  index: number;
}

export const parseRemoveBucklingLength: NormWireReader<RemoveBucklingLength> = normWireObject<RemoveBucklingLength>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
