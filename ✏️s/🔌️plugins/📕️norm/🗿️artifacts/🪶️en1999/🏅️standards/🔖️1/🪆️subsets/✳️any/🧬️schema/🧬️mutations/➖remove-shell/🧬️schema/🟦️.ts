/** 🗂️ `remove-shell` wire twin: the leaf payload `RemoveShell`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveShell {
  mutation: "removeShell";
  id: string;
}

export const parseRemoveShell: NormWireReader<RemoveShell> = normWireObject<RemoveShell>({ mutation: normWireRequired(normWireLiteral("removeShell")), id: normWireRequired(normWireString) });
