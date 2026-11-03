/** 📉 `change-deflection-limit` wire twin: the leaf payload `ChangeDeflectionLimit`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeflectionLimit {
  id: string;
  newDeflectionLimit: number;
}

export const parseChangeDeflectionLimit: NormWireReader<ChangeDeflectionLimit> = normWireObject<ChangeDeflectionLimit>({ id: normWireRequired(normWireString), newDeflectionLimit: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000})) });
