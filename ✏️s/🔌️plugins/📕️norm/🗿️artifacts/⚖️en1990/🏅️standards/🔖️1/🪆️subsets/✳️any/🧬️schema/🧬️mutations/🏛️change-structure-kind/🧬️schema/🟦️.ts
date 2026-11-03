/** 🏛️ `change-structure-kind` wire twin: the leaf payload `ChangeStructureKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990StructureKind, parseEn1990StructureKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeStructureKind {
  mutation: "changeStructureKind";
  newStructureKind: En1990StructureKind;
}

export const parseChangeStructureKind: NormWireReader<ChangeStructureKind> = normWireObject<ChangeStructureKind>({ mutation: normWireRequired(normWireLiteral("changeStructureKind")), newStructureKind: normWireRequired(parseEn1990StructureKind) });
