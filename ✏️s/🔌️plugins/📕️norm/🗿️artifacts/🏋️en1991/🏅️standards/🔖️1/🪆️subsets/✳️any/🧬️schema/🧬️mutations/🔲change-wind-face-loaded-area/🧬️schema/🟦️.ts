/** 🔲 `change-wind-face-loaded-area` wire twin: the leaf payload `ChangeWindFaceLoadedArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWindFaceLoadedArea {
  id: string;
  newLoadedArea: number;
}

export const parseChangeWindFaceLoadedArea: NormWireReader<ChangeWindFaceLoadedArea> = normWireObject<ChangeWindFaceLoadedArea>({ id: normWireRequired(normWireString), newLoadedArea: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
