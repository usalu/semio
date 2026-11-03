/** ➖️ `remove-prestress` wire twin: the leaf payload `RemovePrestress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePrestress {
  wallId: string;
  id: string;
}

export const parseRemovePrestress: NormWireReader<RemovePrestress> = normWireObject<RemovePrestress>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
