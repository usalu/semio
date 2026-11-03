/** ➖️ `remove-silo` wire twin: the leaf payload `RemoveSilo`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSilo {
  mutation: "removeSilo";
  id: string;
}

export const parseRemoveSilo: NormWireReader<RemoveSilo> = normWireObject<RemoveSilo>({ mutation: normWireRequired(normWireLiteral("removeSilo")), id: normWireRequired(normWireString) });
