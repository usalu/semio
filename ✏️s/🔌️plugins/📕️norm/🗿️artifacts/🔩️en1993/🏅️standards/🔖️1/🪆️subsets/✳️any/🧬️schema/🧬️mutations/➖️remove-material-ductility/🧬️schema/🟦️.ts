/** ➖️ `remove-material-ductility` wire twin: the leaf payload `RemoveMaterialDuctility`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveMaterialDuctility {
  index: number;
}

export const parseRemoveMaterialDuctility: NormWireReader<RemoveMaterialDuctility> = normWireObject<RemoveMaterialDuctility>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
