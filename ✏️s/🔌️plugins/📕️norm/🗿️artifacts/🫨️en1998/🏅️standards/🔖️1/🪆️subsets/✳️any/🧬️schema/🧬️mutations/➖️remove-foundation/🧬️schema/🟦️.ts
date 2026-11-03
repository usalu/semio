/** ➖️ `remove-foundation` wire twin: the leaf payload `RemoveFoundation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFoundation {
  mutation: "removeFoundation";
  id: string;
}

export const parseRemoveFoundation: NormWireReader<RemoveFoundation> = normWireObject<RemoveFoundation>({ mutation: normWireRequired(normWireLiteral("removeFoundation")), id: normWireRequired(normWireString) });
