/** ⚓️ `change-permanent-kind` wire twin: the leaf payload `ChangePermanentKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990PermanentKind, parseEn1990PermanentKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePermanentKind {
  mutation: "changePermanentKind";
  id: string;
  newKind: En1990PermanentKind;
}

export const parseChangePermanentKind: NormWireReader<ChangePermanentKind> = normWireObject<ChangePermanentKind>({ mutation: normWireRequired(normWireLiteral("changePermanentKind")), id: normWireRequired(normWireString), newKind: normWireRequired(parseEn1990PermanentKind) });
