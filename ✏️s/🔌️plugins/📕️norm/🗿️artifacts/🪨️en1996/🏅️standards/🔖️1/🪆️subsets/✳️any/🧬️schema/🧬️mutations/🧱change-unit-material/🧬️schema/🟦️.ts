/** 🧱 `change-unit-material` wire twin: the leaf payload `ChangeUnitMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996UnitMaterial, parseEn1996UnitMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeUnitMaterial {
  id: string;
  newUnitMaterial: En1996UnitMaterial;
}

export const parseChangeUnitMaterial: NormWireReader<ChangeUnitMaterial> = normWireObject<ChangeUnitMaterial>({ id: normWireRequired(normWireString), newUnitMaterial: normWireRequired(parseEn1996UnitMaterial) });
