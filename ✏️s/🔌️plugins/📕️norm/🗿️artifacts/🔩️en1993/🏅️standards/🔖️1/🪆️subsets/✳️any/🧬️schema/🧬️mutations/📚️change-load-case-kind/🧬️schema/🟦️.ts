/** 📚️ `change-load-case-kind` wire twin: the leaf payload `ChangeLoadCaseKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeLoadCaseKind {
  id: string;
  newKind: ActionKind;
}

export const parseChangeLoadCaseKind: NormWireReader<ChangeLoadCaseKind> = normWireObject<ChangeLoadCaseKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
