/** ⚗️ `change-material-subgrade` wire twin: the leaf payload `ChangeMaterialSubgrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialSubgrade {
  id: string;
  newSubgrade: string;
}

export const parseChangeMaterialSubgrade: NormWireReader<ChangeMaterialSubgrade> = normWireObject<ChangeMaterialSubgrade>({ id: normWireRequired(normWireString), newSubgrade: normWireRequired(normWireString) });
