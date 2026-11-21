/** ➖️ `remove-case` wire twin: the leaf payload `RemoveCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCase {
  mutation: "removeCase";
  id: string;
}

export const parseRemoveCase: NormWireReader<RemoveCase> = normWireObject<RemoveCase>({ mutation: normWireRequired(normWireLiteral("removeCase")), id: normWireRequired(normWireString) });
