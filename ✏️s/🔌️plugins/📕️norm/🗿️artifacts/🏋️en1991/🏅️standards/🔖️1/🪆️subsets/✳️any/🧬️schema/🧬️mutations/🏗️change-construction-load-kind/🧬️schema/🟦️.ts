/** 🏗 `change-construction-load-kind` wire twin: the leaf payload `ChangeConstructionLoadKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConstructionLoadKind {
  id: string;
  newKind: number;
}

export const parseChangeConstructionLoadKind: NormWireReader<ChangeConstructionLoadKind> = normWireObject<ChangeConstructionLoadKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
