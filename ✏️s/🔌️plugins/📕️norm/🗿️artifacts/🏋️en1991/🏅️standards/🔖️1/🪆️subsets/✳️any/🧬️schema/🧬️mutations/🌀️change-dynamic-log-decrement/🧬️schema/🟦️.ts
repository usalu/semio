/** 🌀 `change-dynamic-log-decrement` wire twin: the leaf payload `ChangeDynamicLogDecrement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDynamicLogDecrement {
  id: string;
  newLogDecrement: number;
}

export const parseChangeDynamicLogDecrement: NormWireReader<ChangeDynamicLogDecrement> = normWireObject<ChangeDynamicLogDecrement>({ id: normWireRequired(normWireString), newLogDecrement: normWireRequired(normWireNumber) });
