/** 🔩 `insert-stud-row` wire twin: the leaf payload `InsertStudRow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StudRow, parseStudRow } from "../../../📸️snapshot/🟦️.ts";

export interface InsertStudRow {
  memberId: string;
  index: number;
  studRow: StudRow;
}

export const parseInsertStudRow: NormWireReader<InsertStudRow> = normWireObject<InsertStudRow>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), studRow: normWireRequired(parseStudRow) });
