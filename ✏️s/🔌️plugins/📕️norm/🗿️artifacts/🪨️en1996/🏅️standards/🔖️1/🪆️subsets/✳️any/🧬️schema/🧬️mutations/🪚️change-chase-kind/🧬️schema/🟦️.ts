/** 🪚️ `change-chase-kind` wire twin: the leaf payload `ChangeChaseKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseKind {
  id: string;
  newKind: "Vertical" | "Horizontal" | "Recess";
}

export const parseChangeChaseKind: NormWireReader<ChangeChaseKind> = normWireObject<ChangeChaseKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(normWireLiteral("Vertical", "Horizontal", "Recess")) });
