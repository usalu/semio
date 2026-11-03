/** 🕳️ `change-cutout-face` wire twin: the leaf payload `ChangeCutoutFace`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CutoutFace, parseCutoutFace } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeCutoutFace {
  cutoutId: string;
  newValue: CutoutFace;
}

export const parseChangeCutoutFace: NormWireReader<ChangeCutoutFace> = normWireObject<ChangeCutoutFace>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(parseCutoutFace) });
