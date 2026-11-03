/** ➖️ `remove-opening` wire twin: the leaf payload `RemoveOpening`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveOpening {
  wallId: string;
  id: string;
}

export const parseRemoveOpening: NormWireReader<RemoveOpening> = normWireObject<RemoveOpening>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
