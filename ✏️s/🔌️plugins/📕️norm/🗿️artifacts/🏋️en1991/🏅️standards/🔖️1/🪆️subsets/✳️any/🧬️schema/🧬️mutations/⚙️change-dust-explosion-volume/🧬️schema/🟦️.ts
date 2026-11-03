/** ⚙ `change-dust-explosion-volume` wire twin: the leaf payload `ChangeDustExplosionVolume`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDustExplosionVolume {
  id: string;
  newVolume: number;
}

export const parseChangeDustExplosionVolume: NormWireReader<ChangeDustExplosionVolume> = normWireObject<ChangeDustExplosionVolume>({ id: normWireRequired(normWireString), newVolume: normWireRequired(normWireNumber) });
