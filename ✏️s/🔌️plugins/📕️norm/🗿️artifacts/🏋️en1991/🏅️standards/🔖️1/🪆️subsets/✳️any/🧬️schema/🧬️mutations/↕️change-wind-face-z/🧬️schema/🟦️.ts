/** ↕️ `change-wind-face-z` wire twin: the leaf payload `ChangeWindFaceZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceZ {
  id: string;
  newZ: number;
}

export const parseChangeWindFaceZ: NormWireReader<ChangeWindFaceZ> = normWireObject<ChangeWindFaceZ>({ id: normWireRequired(normWireString), newZ: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
