/** 🏛️ `change-pier-position` wire twin: the leaf payload `ChangePierPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierPosition {
  wallId: string;
  id: string;
  newPositionM: number;
}

export const parseChangePierPosition: NormWireReader<ChangePierPosition> = normWireObject<ChangePierPosition>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newPositionM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
