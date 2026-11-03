/** ⚙ `change-rail-load-group-assumed-longitudinal` wire twin: the leaf payload `ChangeRailLoadGroupAssumedLongitudinal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupAssumedLongitudinal {
  id: string;
  newAssumedLongitudinal: number;
}

export const parseChangeRailLoadGroupAssumedLongitudinal: NormWireReader<ChangeRailLoadGroupAssumedLongitudinal> = normWireObject<ChangeRailLoadGroupAssumedLongitudinal>({ id: normWireRequired(normWireString), newAssumedLongitudinal: normWireRequired(normWireNumber) });
