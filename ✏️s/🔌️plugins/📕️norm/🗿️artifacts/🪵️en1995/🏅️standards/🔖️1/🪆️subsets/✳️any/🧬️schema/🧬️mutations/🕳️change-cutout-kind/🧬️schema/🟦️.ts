/** 🕳️ `change-cutout-kind` wire twin: the leaf payload `ChangeCutoutKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CutoutKind, parseCutoutKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeCutoutKind {
  cutoutId: string;
  newValue: CutoutKind;
}

export const parseChangeCutoutKind: NormWireReader<ChangeCutoutKind> = normWireObject<ChangeCutoutKind>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(parseCutoutKind) });
