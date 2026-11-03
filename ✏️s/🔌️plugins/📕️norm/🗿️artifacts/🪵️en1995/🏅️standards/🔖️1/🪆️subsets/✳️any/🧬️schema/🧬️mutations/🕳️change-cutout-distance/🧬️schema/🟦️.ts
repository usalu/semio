/** 🕳️ `change-cutout-distance` wire twin: the leaf payload `ChangeCutoutDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutDistance {
  cutoutId: string;
  newValue: number;
}

export const parseChangeCutoutDistance: NormWireReader<ChangeCutoutDistance> = normWireObject<ChangeCutoutDistance>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
