/** 🫙 `change-shell-action-source` wire twin: the leaf payload `ChangeShellActionSource`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionSource, parseActionSource } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeShellActionSource {
  mutation: "changeShellActionSource";
  shellId: string;
  actionId: string;
  newSource: ActionSource;
}

export const parseChangeShellActionSource: NormWireReader<ChangeShellActionSource> = normWireObject<ChangeShellActionSource>({ mutation: normWireRequired(normWireLiteral("changeShellActionSource")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newSource: normWireRequired(parseActionSource) });
