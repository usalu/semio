/** 🕳️ `change-cutout-centre-height` wire twin: the leaf payload `ChangeCutoutCentreHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutCentreHeight {
  cutoutId: string;
  newValue: number;
}

export const parseChangeCutoutCentreHeight: NormWireReader<ChangeCutoutCentreHeight> = normWireObject<ChangeCutoutCentreHeight>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
