/** ➖️ `remove-material` wire twin: the leaf payload `RemoveMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMaterial {
  index: number;
}

export const parseRemoveMaterial: NormWireReader<RemoveMaterial> = normWireObject<RemoveMaterial>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
