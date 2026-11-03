/** 🏗️ `change-is-basement` wire twin: the leaf payload `ChangeIsBasement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsBasement {
  id: string;
  newIsBasement: boolean;
}

export const parseChangeIsBasement: NormWireReader<ChangeIsBasement> = normWireObject<ChangeIsBasement>({ id: normWireRequired(normWireString), newIsBasement: normWireRequired(normWireBoolean) });
