/** 🪚️ `remove-chase` wire twin: the leaf payload `RemoveChase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveChase {
  id: string;
}

export const parseRemoveChase: NormWireReader<RemoveChase> = normWireObject<RemoveChase>({ id: normWireRequired(normWireString) });
