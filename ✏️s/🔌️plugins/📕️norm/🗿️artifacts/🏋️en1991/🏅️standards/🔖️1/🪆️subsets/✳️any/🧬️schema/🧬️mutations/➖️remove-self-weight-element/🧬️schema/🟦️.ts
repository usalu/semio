/** ➖️ `remove-self-weight-element` wire twin: the leaf payload `RemoveSelfWeightElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSelfWeightElement {
  id: string;
}

export const parseRemoveSelfWeightElement: NormWireReader<RemoveSelfWeightElement> = normWireObject<RemoveSelfWeightElement>({ id: normWireRequired(normWireString) });
