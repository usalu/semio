/** 🚁 `remove-helicopter` wire twin: the leaf payload `RemoveHelicopter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveHelicopter {
  id: string;
}

export const parseRemoveHelicopter: NormWireReader<RemoveHelicopter> = normWireObject<RemoveHelicopter>({ id: normWireRequired(normWireString) });
