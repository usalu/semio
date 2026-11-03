/** ❄️ `change-sheet-action-source` wire twin: the leaf payload `ChangeSheetActionSource`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionSource, parseActionSource } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSheetActionSource {
  mutation: "changeSheetActionSource";
  coldFormedId: string;
  actionId: string;
  newSource: ActionSource;
}

export const parseChangeSheetActionSource: NormWireReader<ChangeSheetActionSource> = normWireObject<ChangeSheetActionSource>({ mutation: normWireRequired(normWireLiteral("changeSheetActionSource")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newSource: normWireRequired(parseActionSource) });
