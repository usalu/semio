/** 🧱️ `change-plated-patch-type` wire twin: the leaf payload `ChangePlatedPatchType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PatchLoadType, parsePatchLoadType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePlatedPatchType {
  id: string;
  newPatchType: PatchLoadType;
}

export const parseChangePlatedPatchType: NormWireReader<ChangePlatedPatchType> = normWireObject<ChangePlatedPatchType>({ id: normWireRequired(normWireString), newPatchType: normWireRequired(parsePatchLoadType) });
