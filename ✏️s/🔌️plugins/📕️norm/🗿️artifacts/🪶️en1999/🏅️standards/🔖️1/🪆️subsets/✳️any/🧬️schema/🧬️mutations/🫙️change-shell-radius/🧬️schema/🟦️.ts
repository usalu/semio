/** 🫙 `change-shell-radius` wire twin: the leaf payload `ChangeShellRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellRadius {
  mutation: "changeShellRadius";
  shellId: string;
  newRadius: number;
}

export const parseChangeShellRadius: NormWireReader<ChangeShellRadius> = normWireObject<ChangeShellRadius>({ mutation: normWireRequired(normWireLiteral("changeShellRadius")), shellId: normWireRequired(normWireString), newRadius: normWireRequired(normWireNumber) });
