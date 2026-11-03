/** ⚙ `change-dust-explosion-kst` wire twin: the leaf payload `ChangeDustExplosionKst`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDustExplosionKst {
  id: string;
  newKst: number;
}

export const parseChangeDustExplosionKst: NormWireReader<ChangeDustExplosionKst> = normWireObject<ChangeDustExplosionKst>({ id: normWireRequired(normWireString), newKst: normWireRequired(normWireNumber) });
