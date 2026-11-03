/** 🪜️ `change-built-up-kind` wire twin: the leaf payload `ChangeBuiltUpKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BuiltUpKind, parseBuiltUpKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBuiltUpKind {
  id: string;
  newKind: BuiltUpKind;
}

export const parseChangeBuiltUpKind: NormWireReader<ChangeBuiltUpKind> = normWireObject<ChangeBuiltUpKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseBuiltUpKind) });
