/** ➖️ `remove-wind-face` wire twin: the leaf payload `RemoveWindFace`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveWindFace {
  id: string;
}

export const parseRemoveWindFace: NormWireReader<RemoveWindFace> = normWireObject<RemoveWindFace>({ id: normWireRequired(normWireString) });
