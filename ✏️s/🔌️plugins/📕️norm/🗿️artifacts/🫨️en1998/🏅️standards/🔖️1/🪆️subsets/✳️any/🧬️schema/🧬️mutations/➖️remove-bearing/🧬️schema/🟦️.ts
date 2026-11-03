/** ➖️ `remove-bearing` wire twin: the leaf payload `RemoveBearing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBearing {
  mutation: "removeBearing";
  id: string;
}

export const parseRemoveBearing: NormWireReader<RemoveBearing> = normWireObject<RemoveBearing>({ mutation: normWireRequired(normWireLiteral("removeBearing")), id: normWireRequired(normWireString) });
