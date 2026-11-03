/** 🌬 `change-wind-face-zone` wire twin: the leaf payload `ChangeWindFaceZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceZone {
  id: string;
  newZone: string;
}

export const parseChangeWindFaceZone: NormWireReader<ChangeWindFaceZone> = normWireObject<ChangeWindFaceZone>({ id: normWireRequired(normWireString), newZone: normWireRequired(normWireString) });
