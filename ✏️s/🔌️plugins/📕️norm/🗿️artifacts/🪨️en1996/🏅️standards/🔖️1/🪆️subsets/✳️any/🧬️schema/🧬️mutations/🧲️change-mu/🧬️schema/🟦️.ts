/** 🧲️ `change-mu` wire twin: the leaf payload `ChangeMu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMu {
  id: string;
  newMu: number;
}

export const parseChangeMu: NormWireReader<ChangeMu> = normWireObject<ChangeMu>({ id: normWireRequired(normWireString), newMu: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":5})) });
