/** ➖️ `remove-pile` wire twin: the leaf payload `RemovePile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePile {
  index: number;
}

export const parseRemovePile: NormWireReader<RemovePile> = normWireObject<RemovePile>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
