/** 🧰️ `change-confined-steel` wire twin: the leaf payload `ChangeConfinedSteel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConfinedSteel {
  wallId: string;
  id: string;
  newAsM2: number;
}

export const parseChangeConfinedSteel: NormWireReader<ChangeConfinedSteel> = normWireObject<ChangeConfinedSteel>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newAsM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });
