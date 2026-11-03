/** ➖️ `remove-flange` wire twin: the leaf payload `RemoveFlange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFlange {
  wallId: string;
  id: string;
}

export const parseRemoveFlange: NormWireReader<RemoveFlange> = normWireObject<RemoveFlange>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
