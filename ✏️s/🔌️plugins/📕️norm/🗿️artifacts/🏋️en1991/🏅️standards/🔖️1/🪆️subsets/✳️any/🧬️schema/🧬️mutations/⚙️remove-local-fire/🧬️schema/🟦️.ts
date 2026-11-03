/** ⚙ `remove-local-fire` wire twin: the leaf payload `RemoveLocalFire`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLocalFire {
  id: string;
}

export const parseRemoveLocalFire: NormWireReader<RemoveLocalFire> = normWireObject<RemoveLocalFire>({ id: normWireRequired(normWireString) });
