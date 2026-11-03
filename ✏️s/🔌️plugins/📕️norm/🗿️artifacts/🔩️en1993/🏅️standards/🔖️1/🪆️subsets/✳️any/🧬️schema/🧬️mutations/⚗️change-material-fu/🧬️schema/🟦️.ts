/** ⚗️ `change-material-fu` wire twin: the leaf payload `ChangeMaterialFu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialFu {
  id: string;
  newFu: number;
}

export const parseChangeMaterialFu: NormWireReader<ChangeMaterialFu> = normWireObject<ChangeMaterialFu>({ id: normWireRequired(normWireString), newFu: normWireRequired(normWireNumber) });
