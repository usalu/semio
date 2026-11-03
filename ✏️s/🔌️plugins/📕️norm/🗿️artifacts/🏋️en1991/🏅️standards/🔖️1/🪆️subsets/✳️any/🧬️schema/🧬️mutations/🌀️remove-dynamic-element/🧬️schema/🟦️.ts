/** 🌀 `remove-dynamic-element` wire twin: the leaf payload `RemoveDynamicElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveDynamicElement {
  id: string;
}

export const parseRemoveDynamicElement: NormWireReader<RemoveDynamicElement> = normWireObject<RemoveDynamicElement>({ id: normWireRequired(normWireString) });
