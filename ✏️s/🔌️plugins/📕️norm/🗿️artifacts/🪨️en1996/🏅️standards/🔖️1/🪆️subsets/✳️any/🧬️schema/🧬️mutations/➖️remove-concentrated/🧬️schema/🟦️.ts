/** ➖️ `remove-concentrated` wire twin: the leaf payload `RemoveConcentrated`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConcentrated {
  wallId: string;
  loadCaseId: string;
  id: string;
}

export const parseRemoveConcentrated: NormWireReader<RemoveConcentrated> = normWireObject<RemoveConcentrated>({ wallId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
