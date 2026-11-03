/** ➖️ `remove-pier` wire twin: the leaf payload `RemovePier`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePier {
  mutation: "removePier";
  id: string;
}

export const parseRemovePier: NormWireReader<RemovePier> = normWireObject<RemovePier>({ mutation: normWireRequired(normWireLiteral("removePier")), id: normWireRequired(normWireString) });
