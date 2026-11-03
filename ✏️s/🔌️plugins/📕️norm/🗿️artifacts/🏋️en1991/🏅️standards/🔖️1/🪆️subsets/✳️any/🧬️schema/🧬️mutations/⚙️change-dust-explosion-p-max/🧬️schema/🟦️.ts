/** ⚙ `change-dust-explosion-p-max` wire twin: the leaf payload `ChangeDustExplosionPMax`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDustExplosionPMax {
  id: string;
  newPMax: number;
}

export const parseChangeDustExplosionPMax: NormWireReader<ChangeDustExplosionPMax> = normWireObject<ChangeDustExplosionPMax>({ id: normWireRequired(normWireString), newPMax: normWireRequired(normWireNumber) });
