/** ⚙ `change-dust-explosion-design-pressure` wire twin: the leaf payload `ChangeDustExplosionDesignPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDustExplosionDesignPressure {
  id: string;
  newDesignPressure: number;
}

export const parseChangeDustExplosionDesignPressure: NormWireReader<ChangeDustExplosionDesignPressure> = normWireObject<ChangeDustExplosionDesignPressure>({ id: normWireRequired(normWireString), newDesignPressure: normWireRequired(normWireNumber) });
