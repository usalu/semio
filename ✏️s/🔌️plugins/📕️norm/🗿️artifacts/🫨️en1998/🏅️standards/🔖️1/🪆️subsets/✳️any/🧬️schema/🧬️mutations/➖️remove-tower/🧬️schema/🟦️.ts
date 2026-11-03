/** ➖️ `remove-tower` wire twin: the leaf payload `RemoveTower`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTower {
  mutation: "removeTower";
  id: string;
}

export const parseRemoveTower: NormWireReader<RemoveTower> = normWireObject<RemoveTower>({ mutation: normWireRequired(normWireLiteral("removeTower")), id: normWireRequired(normWireString) });
