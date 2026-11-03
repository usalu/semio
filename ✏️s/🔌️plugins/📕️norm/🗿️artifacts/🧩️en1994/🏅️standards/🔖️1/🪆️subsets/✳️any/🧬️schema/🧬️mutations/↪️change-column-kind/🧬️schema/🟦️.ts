/** ↪️ `change-column-kind` wire twin: the leaf payload `ChangeColumnKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ColumnKind, parseColumnKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeColumnKind {
  id: string;
  newKind: ColumnKind;
}

export const parseChangeColumnKind: NormWireReader<ChangeColumnKind> = normWireObject<ChangeColumnKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseColumnKind) });
