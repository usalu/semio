/** 🫙 `change-shell-welded` wire twin: the leaf payload `ChangeShellWelded`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellWelded {
  mutation: "changeShellWelded";
  shellId: string;
  newWelded: boolean;
}

export const parseChangeShellWelded: NormWireReader<ChangeShellWelded> = normWireObject<ChangeShellWelded>({ mutation: normWireRequired(normWireLiteral("changeShellWelded")), shellId: normWireRequired(normWireString), newWelded: normWireRequired(normWireBoolean) });
