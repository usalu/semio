/** 🔩 `change-f-yd` wire twin: the leaf payload `ChangeFYd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFYd {
  id: string;
  newFYdPa: number;
}

export const parseChangeFYd: NormWireReader<ChangeFYd> = normWireObject<ChangeFYd>({ id: normWireRequired(normWireString), newFYdPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
