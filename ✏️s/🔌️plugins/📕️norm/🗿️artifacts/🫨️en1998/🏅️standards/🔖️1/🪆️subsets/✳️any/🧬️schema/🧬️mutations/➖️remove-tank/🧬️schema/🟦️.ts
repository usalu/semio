/** ➖️ `remove-tank` wire twin: the leaf payload `RemoveTank`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTank {
  mutation: "removeTank";
  id: string;
}

export const parseRemoveTank: NormWireReader<RemoveTank> = normWireObject<RemoveTank>({ mutation: normWireRequired(normWireLiteral("removeTank")), id: normWireRequired(normWireString) });
