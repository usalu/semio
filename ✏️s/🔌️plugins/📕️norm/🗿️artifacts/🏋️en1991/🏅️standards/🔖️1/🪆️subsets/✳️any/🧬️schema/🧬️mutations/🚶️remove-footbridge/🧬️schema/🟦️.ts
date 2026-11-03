/** 🚶 `remove-footbridge` wire twin: the leaf payload `RemoveFootbridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFootbridge {
  id: string;
}

export const parseRemoveFootbridge: NormWireReader<RemoveFootbridge> = normWireObject<RemoveFootbridge>({ id: normWireRequired(normWireString) });
