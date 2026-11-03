/** ⚙ `remove-snow-guard` wire twin: the leaf payload `RemoveSnowGuard`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSnowGuard {
  id: string;
}

export const parseRemoveSnowGuard: NormWireReader<RemoveSnowGuard> = normWireObject<RemoveSnowGuard>({ id: normWireRequired(normWireString) });
