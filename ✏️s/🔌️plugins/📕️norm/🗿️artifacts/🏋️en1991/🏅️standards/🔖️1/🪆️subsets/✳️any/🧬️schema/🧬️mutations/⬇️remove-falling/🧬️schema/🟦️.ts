/** ⬇ `remove-falling` wire twin: the leaf payload `RemoveFalling`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFalling {
  id: string;
}

export const parseRemoveFalling: NormWireReader<RemoveFalling> = normWireObject<RemoveFalling>({ id: normWireRequired(normWireString) });
