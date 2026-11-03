/** 🗂️ `insert-patch-load` wire twin: the leaf payload `InsertPatchLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PatchLoad, parsePatchLoad } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPatchLoad {
  mutation: "insertPatchLoad";
  index: number;
  patchLoad: PatchLoad;
}

export const parseInsertPatchLoad: NormWireReader<InsertPatchLoad> = normWireObject<InsertPatchLoad>({ mutation: normWireRequired(normWireLiteral("insertPatchLoad")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), patchLoad: normWireRequired(parsePatchLoad) });
