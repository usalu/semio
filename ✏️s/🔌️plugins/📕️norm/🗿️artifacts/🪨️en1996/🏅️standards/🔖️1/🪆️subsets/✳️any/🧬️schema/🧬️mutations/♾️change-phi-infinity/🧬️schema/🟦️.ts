/** ♾️ `change-phi-infinity` wire twin: the leaf payload `ChangePhiInfinity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePhiInfinity {
  id: string;
  newPhiInfinity: number;
}

export const parseChangePhiInfinity: NormWireReader<ChangePhiInfinity> = normWireObject<ChangePhiInfinity>({ id: normWireRequired(normWireString), newPhiInfinity: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":20})) });
