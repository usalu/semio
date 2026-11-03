/** ⚙ `change-rail-load-group-assumed-axle-load` wire twin: the leaf payload `ChangeRailLoadGroupAssumedAxleLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupAssumedAxleLoad {
  id: string;
  newAssumedAxleLoad: number;
}

export const parseChangeRailLoadGroupAssumedAxleLoad: NormWireReader<ChangeRailLoadGroupAssumedAxleLoad> = normWireObject<ChangeRailLoadGroupAssumedAxleLoad>({ id: normWireRequired(normWireString), newAssumedAxleLoad: normWireRequired(normWireNumber) });
