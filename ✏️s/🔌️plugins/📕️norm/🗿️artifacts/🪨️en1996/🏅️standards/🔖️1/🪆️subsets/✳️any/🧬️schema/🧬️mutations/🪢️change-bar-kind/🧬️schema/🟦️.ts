/** 🪢️ `change-bar-kind` wire twin: the leaf payload `ChangeBarKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996BarKind, parseEn1996BarKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBarKind {
  wallId: string;
  id: string;
  newKind: En1996BarKind;
}

export const parseChangeBarKind: NormWireReader<ChangeBarKind> = normWireObject<ChangeBarKind>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newKind: normWireRequired(parseEn1996BarKind) });
