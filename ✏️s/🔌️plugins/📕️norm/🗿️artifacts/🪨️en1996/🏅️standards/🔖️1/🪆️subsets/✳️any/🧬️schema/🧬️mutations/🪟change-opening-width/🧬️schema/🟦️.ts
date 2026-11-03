/** 🪟 `change-opening-width` wire twin: the leaf payload `ChangeOpeningWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningWidth {
  wallId: string;
  id: string;
  newWidthM: number;
}

export const parseChangeOpeningWidth: NormWireReader<ChangeOpeningWidth> = normWireObject<ChangeOpeningWidth>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
