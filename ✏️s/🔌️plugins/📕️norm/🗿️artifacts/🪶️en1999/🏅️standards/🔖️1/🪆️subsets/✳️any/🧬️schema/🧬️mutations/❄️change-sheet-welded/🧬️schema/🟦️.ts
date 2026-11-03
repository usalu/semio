/** ❄️ `change-sheet-welded` wire twin: the leaf payload `ChangeSheetWelded`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetWelded {
  mutation: "changeSheetWelded";
  coldFormedId: string;
  newWelded: boolean;
}

export const parseChangeSheetWelded: NormWireReader<ChangeSheetWelded> = normWireObject<ChangeSheetWelded>({ mutation: normWireRequired(normWireLiteral("changeSheetWelded")), coldFormedId: normWireRequired(normWireString), newWelded: normWireRequired(normWireBoolean) });
