/** ⚙ `change-dust-explosion-static-opening-pressure` wire twin: the leaf payload `ChangeDustExplosionStaticOpeningPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDustExplosionStaticOpeningPressure {
  id: string;
  newStaticOpeningPressure: number;
}

export const parseChangeDustExplosionStaticOpeningPressure: NormWireReader<ChangeDustExplosionStaticOpeningPressure> = normWireObject<ChangeDustExplosionStaticOpeningPressure>({ id: normWireRequired(normWireString), newStaticOpeningPressure: normWireRequired(normWireNumber) });
