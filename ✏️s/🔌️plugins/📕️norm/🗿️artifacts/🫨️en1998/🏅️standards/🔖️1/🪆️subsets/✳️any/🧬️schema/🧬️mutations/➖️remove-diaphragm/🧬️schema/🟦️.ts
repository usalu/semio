/** ➖️ `remove-diaphragm` wire twin: the leaf payload `RemoveDiaphragm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveDiaphragm {
  mutation: "removeDiaphragm";
  id: string;
}

export const parseRemoveDiaphragm: NormWireReader<RemoveDiaphragm> = normWireObject<RemoveDiaphragm>({ mutation: normWireRequired(normWireLiteral("removeDiaphragm")), id: normWireRequired(normWireString) });
