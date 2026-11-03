/** 🔗️ `change-tie-kind` wire twin: the leaf payload `ChangeTieKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996TieKind, parseEn1996TieKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTieKind {
  wallId: string;
  id: string;
  newKind: En1996TieKind;
}

export const parseChangeTieKind: NormWireReader<ChangeTieKind> = normWireObject<ChangeTieKind>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newKind: normWireRequired(parseEn1996TieKind) });
