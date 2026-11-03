/** 🕳️ `change-cutout-depth` wire twin: the leaf payload `ChangeCutoutDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutDepth {
  cutoutId: string;
  newValue: number;
}

export const parseChangeCutoutDepth: NormWireReader<ChangeCutoutDepth> = normWireObject<ChangeCutoutDepth>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
