/** 🕳️ `change-cutout-length` wire twin: the leaf payload `ChangeCutoutLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutLength {
  cutoutId: string;
  newValue: number;
}

export const parseChangeCutoutLength: NormWireReader<ChangeCutoutLength> = normWireObject<ChangeCutoutLength>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
