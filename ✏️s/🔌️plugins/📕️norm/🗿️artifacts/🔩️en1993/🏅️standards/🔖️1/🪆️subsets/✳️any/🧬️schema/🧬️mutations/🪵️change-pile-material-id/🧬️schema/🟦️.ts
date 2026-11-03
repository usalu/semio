/** 🪵️ `change-pile-material-id` wire twin: the leaf payload `ChangePileMaterialId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileMaterialId {
  id: string;
  newMaterialId: string;
}

export const parseChangePileMaterialId: NormWireReader<ChangePileMaterialId> = normWireObject<ChangePileMaterialId>({ id: normWireRequired(normWireString), newMaterialId: normWireRequired(normWireString) });
