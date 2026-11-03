/** 🫙 `change-shell-length` wire twin: the leaf payload `ChangeShellLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellLength {
  mutation: "changeShellLength";
  shellId: string;
  newLength: number;
}

export const parseChangeShellLength: NormWireReader<ChangeShellLength> = normWireObject<ChangeShellLength>({ mutation: normWireRequired(normWireLiteral("changeShellLength")), shellId: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
