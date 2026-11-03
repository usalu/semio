/** 🌾 `remove-bulk-solid` wire twin: the leaf payload `RemoveBulkSolid`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBulkSolid {
  id: string;
}

export const parseRemoveBulkSolid: NormWireReader<RemoveBulkSolid> = normWireObject<RemoveBulkSolid>({ id: normWireRequired(normWireString) });
