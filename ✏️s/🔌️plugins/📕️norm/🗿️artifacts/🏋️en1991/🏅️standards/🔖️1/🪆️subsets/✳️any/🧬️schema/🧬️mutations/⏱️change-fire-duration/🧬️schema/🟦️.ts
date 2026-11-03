/** ⏱️ `change-fire-duration` wire twin: the leaf payload `ChangeFireDuration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireDuration {
  newFireDuration: number;
}

export const parseChangeFireDuration: NormWireReader<ChangeFireDuration> = normWireObject<ChangeFireDuration>({ newFireDuration: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
