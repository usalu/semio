/** 🏋️ `insert-action` wire twin: the leaf payload `InsertAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LoadCaseActions, parseLoadCaseActions } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAction {
  memberId: string;
  index: number;
  action: LoadCaseActions;
}

export const parseInsertAction: NormWireReader<InsertAction> = normWireObject<InsertAction>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), action: normWireRequired(parseLoadCaseActions) });
