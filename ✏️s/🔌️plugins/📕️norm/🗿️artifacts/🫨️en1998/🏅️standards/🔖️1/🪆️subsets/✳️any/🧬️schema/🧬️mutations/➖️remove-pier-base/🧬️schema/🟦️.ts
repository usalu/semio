/** ➖️ `remove-pier-base` wire twin: the leaf payload `RemovePierBase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePierBase {
  mutation: "removePierBase";
  id: string;
}

export const parseRemovePierBase: NormWireReader<RemovePierBase> = normWireObject<RemovePierBase>({ mutation: normWireRequired(normWireLiteral("removePierBase")), id: normWireRequired(normWireString) });
