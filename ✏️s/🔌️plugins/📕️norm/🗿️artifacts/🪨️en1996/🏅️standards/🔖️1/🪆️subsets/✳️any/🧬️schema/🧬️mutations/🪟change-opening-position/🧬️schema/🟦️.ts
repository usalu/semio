/** 🪟 `change-opening-position` wire twin: the leaf payload `ChangeOpeningPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningPosition {
  wallId: string;
  id: string;
  newPositionM: number;
}

export const parseChangeOpeningPosition: NormWireReader<ChangeOpeningPosition> = normWireObject<ChangeOpeningPosition>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newPositionM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
