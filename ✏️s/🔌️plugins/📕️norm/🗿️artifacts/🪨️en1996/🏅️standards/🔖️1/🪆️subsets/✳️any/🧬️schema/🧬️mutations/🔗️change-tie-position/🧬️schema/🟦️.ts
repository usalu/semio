/** 🔗️ `change-tie-position` wire twin: the leaf payload `ChangeTiePosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTiePosition {
  wallId: string;
  id: string;
  newPositionM: number;
}

export const parseChangeTiePosition: NormWireReader<ChangeTiePosition> = normWireObject<ChangeTiePosition>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newPositionM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
