/** 🌉 `change-structure-kind` wire twin: the leaf payload `ChangeStructureKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1991StructureKind, parseEn1991StructureKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeStructureKind {
  newStructureKind: En1991StructureKind;
}

export const parseChangeStructureKind: NormWireReader<ChangeStructureKind> = normWireObject<ChangeStructureKind>({ newStructureKind: normWireRequired(parseEn1991StructureKind) });
