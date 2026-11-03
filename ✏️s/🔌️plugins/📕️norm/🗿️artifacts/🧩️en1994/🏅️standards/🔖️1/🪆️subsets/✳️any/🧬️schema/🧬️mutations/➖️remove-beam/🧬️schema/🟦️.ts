/** ➖️ `remove-beam` wire twin: the leaf payload `RemoveBeam`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBeam {
  id: string;
}

export const parseRemoveBeam: NormWireReader<RemoveBeam> = normWireObject<RemoveBeam>({ id: normWireRequired(normWireString) });
