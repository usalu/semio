/** 🌬 `change-wind-face-cd` wire twin: the leaf payload `ChangeWindFaceCD`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceCD {
  id: string;
  newCD: number;
}

export const parseChangeWindFaceCD: NormWireReader<ChangeWindFaceCD> = normWireObject<ChangeWindFaceCD>({ id: normWireRequired(normWireString), newCD: normWireRequired(normWireNumber) });
