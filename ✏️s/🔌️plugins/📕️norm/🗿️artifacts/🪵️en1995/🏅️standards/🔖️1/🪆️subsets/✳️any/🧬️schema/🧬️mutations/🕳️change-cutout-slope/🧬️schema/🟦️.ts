/** 🕳️ `change-cutout-slope` wire twin: the leaf payload `ChangeCutoutSlope`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutSlope {
  cutoutId: string;
  newValue: number;
}

export const parseChangeCutoutSlope: NormWireReader<ChangeCutoutSlope> = normWireObject<ChangeCutoutSlope>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
