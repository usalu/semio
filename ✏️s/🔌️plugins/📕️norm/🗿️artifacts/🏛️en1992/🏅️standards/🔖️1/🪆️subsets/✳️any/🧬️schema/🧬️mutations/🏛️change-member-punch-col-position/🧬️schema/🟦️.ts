/** 🏛️ `change-member-punch-col-position` wire twin: the leaf payload `ChangeMemberPunchColPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ColumnPosition, parseColumnPosition } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberPunchColPosition {
  memberId: string;
  newPunchingColumnPosition: ColumnPosition;
}

export const parseChangeMemberPunchColPosition: NormWireReader<ChangeMemberPunchColPosition> = normWireObject<ChangeMemberPunchColPosition>({ memberId: normWireRequired(normWireString), newPunchingColumnPosition: normWireRequired(parseColumnPosition) });
