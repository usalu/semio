/** 🔗️ `change-conn-action-source` wire twin: the leaf payload `ChangeConnActionSource`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionSource, parseActionSource } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnActionSource {
  mutation: "changeConnActionSource";
  connectionId: string;
  actionId: string;
  newSource: ActionSource;
}

export const parseChangeConnActionSource: NormWireReader<ChangeConnActionSource> = normWireObject<ChangeConnActionSource>({ mutation: normWireRequired(normWireLiteral("changeConnActionSource")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newSource: normWireRequired(parseActionSource) });
