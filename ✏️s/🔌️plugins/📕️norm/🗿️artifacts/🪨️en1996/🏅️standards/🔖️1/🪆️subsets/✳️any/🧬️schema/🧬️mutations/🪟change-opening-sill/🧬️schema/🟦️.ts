/** 🪟 `change-opening-sill` wire twin: the leaf payload `ChangeOpeningSill`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeOpeningSill {
  wallId: string;
  id: string;
  newSillHeightM: number;
}

export const parseChangeOpeningSill: NormWireReader<ChangeOpeningSill> = normWireObject<ChangeOpeningSill>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newSillHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
