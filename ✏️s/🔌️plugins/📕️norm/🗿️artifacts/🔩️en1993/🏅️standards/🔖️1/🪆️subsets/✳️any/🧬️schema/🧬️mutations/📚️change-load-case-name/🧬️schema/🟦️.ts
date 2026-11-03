/** 📚️ `change-load-case-name` wire twin: the leaf payload `ChangeLoadCaseName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseName {
  id: string;
  newName: string;
}

export const parseChangeLoadCaseName: NormWireReader<ChangeLoadCaseName> = normWireObject<ChangeLoadCaseName>({ id: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
