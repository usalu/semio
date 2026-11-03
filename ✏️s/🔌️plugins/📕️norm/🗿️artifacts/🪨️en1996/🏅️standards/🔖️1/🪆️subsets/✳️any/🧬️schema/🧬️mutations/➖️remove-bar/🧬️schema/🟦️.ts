/** ➖️ `remove-bar` wire twin: the leaf payload `RemoveBar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBar {
  wallId: string;
  id: string;
}

export const parseRemoveBar: NormWireReader<RemoveBar> = normWireObject<RemoveBar>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
