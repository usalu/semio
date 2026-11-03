/** 🧨 `remove-internal-explosion` wire twin: the leaf payload `RemoveInternalExplosion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveInternalExplosion {
  id: string;
}

export const parseRemoveInternalExplosion: NormWireReader<RemoveInternalExplosion> = normWireObject<RemoveInternalExplosion>({ id: normWireRequired(normWireString) });
