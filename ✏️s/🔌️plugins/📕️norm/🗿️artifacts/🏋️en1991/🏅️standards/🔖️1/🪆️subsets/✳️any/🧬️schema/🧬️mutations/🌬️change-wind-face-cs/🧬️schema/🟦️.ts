/** 🌬 `change-wind-face-cs` wire twin: the leaf payload `ChangeWindFaceCS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceCS {
  id: string;
  newCS: number;
}

export const parseChangeWindFaceCS: NormWireReader<ChangeWindFaceCS> = normWireObject<ChangeWindFaceCS>({ id: normWireRequired(normWireString), newCS: normWireRequired(normWireNumber) });
