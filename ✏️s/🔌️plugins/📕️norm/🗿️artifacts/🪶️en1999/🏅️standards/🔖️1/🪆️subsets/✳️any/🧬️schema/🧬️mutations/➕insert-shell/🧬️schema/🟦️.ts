/** 🗂️ `insert-shell` wire twin: the leaf payload `InsertShell`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AluminiumShell, parseAluminiumShell } from "../../../📸️snapshot/🟦️.ts";

export interface InsertShell {
  mutation: "insertShell";
  index: number;
  shell: AluminiumShell;
}

export const parseInsertShell: NormWireReader<InsertShell> = normWireObject<InsertShell>({ mutation: normWireRequired(normWireLiteral("insertShell")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), shell: normWireRequired(parseAluminiumShell) });
