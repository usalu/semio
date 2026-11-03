/** ⚙ `change-rail-load-group-factor-longitudinal` wire twin: the leaf payload `ChangeRailLoadGroupFactorLongitudinal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupFactorLongitudinal {
  id: string;
  newFactorLongitudinal: number;
}

export const parseChangeRailLoadGroupFactorLongitudinal: NormWireReader<ChangeRailLoadGroupFactorLongitudinal> = normWireObject<ChangeRailLoadGroupFactorLongitudinal>({ id: normWireRequired(normWireString), newFactorLongitudinal: normWireRequired(normWireNumber) });
