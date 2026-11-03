/** 🔗️ `change-conn-action-qk-line` wire twin: the leaf payload `ChangeConnActionQKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionQKLine {
  mutation: "changeConnActionQKLine";
  connectionId: string;
  actionId: string;
  newQKLine: number;
}

export const parseChangeConnActionQKLine: NormWireReader<ChangeConnActionQKLine> = normWireObject<ChangeConnActionQKLine>({ mutation: normWireRequired(normWireLiteral("changeConnActionQKLine")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newQKLine: normWireRequired(normWireNumber) });
