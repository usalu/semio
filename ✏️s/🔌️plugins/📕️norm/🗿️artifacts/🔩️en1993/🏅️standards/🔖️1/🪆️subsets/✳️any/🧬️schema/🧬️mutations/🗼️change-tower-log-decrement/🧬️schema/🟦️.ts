/** 🗼️ `change-tower-log-decrement` wire twin: the leaf payload `ChangeTowerLogDecrement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerLogDecrement {
  id: string;
  newLogDecrement: number;
}

export const parseChangeTowerLogDecrement: NormWireReader<ChangeTowerLogDecrement> = normWireObject<ChangeTowerLogDecrement>({ id: normWireRequired(normWireString), newLogDecrement: normWireRequired(normWireNumber) });
