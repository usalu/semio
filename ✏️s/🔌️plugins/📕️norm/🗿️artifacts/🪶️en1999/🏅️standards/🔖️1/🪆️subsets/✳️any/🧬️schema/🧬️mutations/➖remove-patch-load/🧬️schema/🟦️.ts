/** 🗂️ `remove-patch-load` wire twin: the leaf payload `RemovePatchLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePatchLoad {
  mutation: "removePatchLoad";
  id: string;
}

export const parseRemovePatchLoad: NormWireReader<RemovePatchLoad> = normWireObject<RemovePatchLoad>({ mutation: normWireRequired(normWireLiteral("removePatchLoad")), id: normWireRequired(normWireString) });
