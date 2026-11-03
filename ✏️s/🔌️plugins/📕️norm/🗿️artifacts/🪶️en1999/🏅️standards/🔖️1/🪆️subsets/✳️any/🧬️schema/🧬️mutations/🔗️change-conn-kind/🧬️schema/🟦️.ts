/** 🔗️ `change-conn-kind` wire twin: the leaf payload `ChangeConnKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConnectionKind, parseConnectionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnKind {
  mutation: "changeConnKind";
  connectionId: string;
  newKind: ConnectionKind;
}

export const parseChangeConnKind: NormWireReader<ChangeConnKind> = normWireObject<ChangeConnKind>({ mutation: normWireRequired(normWireLiteral("changeConnKind")), connectionId: normWireRequired(normWireString), newKind: normWireRequired(parseConnectionKind) });
