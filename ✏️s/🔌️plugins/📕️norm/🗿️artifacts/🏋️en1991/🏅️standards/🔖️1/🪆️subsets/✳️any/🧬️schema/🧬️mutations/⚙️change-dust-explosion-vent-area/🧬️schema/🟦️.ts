/** ⚙ `change-dust-explosion-vent-area` wire twin: the leaf payload `ChangeDustExplosionVentArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDustExplosionVentArea {
  id: string;
  newVentArea: number;
}

export const parseChangeDustExplosionVentArea: NormWireReader<ChangeDustExplosionVentArea> = normWireObject<ChangeDustExplosionVentArea>({ id: normWireRequired(normWireString), newVentArea: normWireRequired(normWireNumber) });
