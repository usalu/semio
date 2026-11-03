/** 🔗️ `change-conn-action-gk-line` wire twin: the leaf payload `ChangeConnActionGKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionGKLine {
  mutation: "changeConnActionGKLine";
  connectionId: string;
  actionId: string;
  newGKLine: number;
}

export const parseChangeConnActionGKLine: NormWireReader<ChangeConnActionGKLine> = normWireObject<ChangeConnActionGKLine>({ mutation: normWireRequired(normWireLiteral("changeConnActionGKLine")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newGKLine: normWireRequired(normWireNumber) });
