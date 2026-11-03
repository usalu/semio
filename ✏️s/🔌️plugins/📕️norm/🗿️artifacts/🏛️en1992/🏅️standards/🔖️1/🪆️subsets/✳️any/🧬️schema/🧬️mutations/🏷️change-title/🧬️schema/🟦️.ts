/** 🏷️ `change-title` wire twin: the leaf payload `ChangeTitle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTitle {
  newTitle: string;
}

export const parseChangeTitle: NormWireReader<ChangeTitle> = normWireObject<ChangeTitle>({ newTitle: normWireRequired(normWireString) });
