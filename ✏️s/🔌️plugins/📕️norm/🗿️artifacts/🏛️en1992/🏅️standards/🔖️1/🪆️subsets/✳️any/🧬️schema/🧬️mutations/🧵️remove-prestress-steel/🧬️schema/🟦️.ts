/** 🧵 `remove-prestress-steel` wire twin: the leaf payload `RemovePrestressSteel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePrestressSteel {
  index: number;
}

export const parseRemovePrestressSteel: NormWireReader<RemovePrestressSteel> = normWireObject<RemovePrestressSteel>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
