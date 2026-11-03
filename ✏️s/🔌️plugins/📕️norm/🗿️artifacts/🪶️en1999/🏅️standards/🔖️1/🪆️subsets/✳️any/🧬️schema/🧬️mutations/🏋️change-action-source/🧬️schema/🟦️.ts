/** 🏋️ `change-action-source` wire twin: the leaf payload `ChangeActionSource`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionSource, parseActionSource } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeActionSource {
  mutation: "changeActionSource";
  memberId: string;
  actionId: string;
  newSource: ActionSource;
}

export const parseChangeActionSource: NormWireReader<ChangeActionSource> = normWireObject<ChangeActionSource>({ mutation: normWireRequired(normWireLiteral("changeActionSource")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newSource: normWireRequired(parseActionSource) });
