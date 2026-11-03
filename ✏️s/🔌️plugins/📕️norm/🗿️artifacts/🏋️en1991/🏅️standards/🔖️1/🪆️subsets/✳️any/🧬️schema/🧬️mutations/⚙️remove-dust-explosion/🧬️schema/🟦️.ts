/** ⚙ `remove-dust-explosion` wire twin: the leaf payload `RemoveDustExplosion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveDustExplosion {
  id: string;
}

export const parseRemoveDustExplosion: NormWireReader<RemoveDustExplosion> = normWireObject<RemoveDustExplosion>({ id: normWireRequired(normWireString) });
