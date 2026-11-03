/** 📏 `remove-load-dispersal` wire twin: the leaf payload `RemoveLoadDispersal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLoadDispersal {
  id: string;
}

export const parseRemoveLoadDispersal: NormWireReader<RemoveLoadDispersal> = normWireObject<RemoveLoadDispersal>({ id: normWireRequired(normWireString) });
