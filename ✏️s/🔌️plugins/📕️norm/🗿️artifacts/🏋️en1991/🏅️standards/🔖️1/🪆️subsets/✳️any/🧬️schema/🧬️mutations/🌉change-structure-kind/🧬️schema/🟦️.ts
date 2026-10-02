/** 🌉 `change-structure-kind` wire twin: the leaf payload `ChangeStructureKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1991StructureKind, parseEn1991StructureKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeStructureKind {
  newStructureKind: En1991StructureKind;
}

export const parseChangeStructureKind: NormWireReader<ChangeStructureKind> = normWireObject<ChangeStructureKind>({ newStructureKind: normWireRequired(parseEn1991StructureKind) });
