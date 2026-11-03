/** 🪟 `change-opening-height` wire twin: the leaf payload `ChangeOpeningHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningHeight {
  wallId: string;
  id: string;
  newHeightM: number;
}

export const parseChangeOpeningHeight: NormWireReader<ChangeOpeningHeight> = normWireObject<ChangeOpeningHeight>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
