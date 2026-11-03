/** 🧱 `change-unit-group` wire twin: the leaf payload `ChangeUnitGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996UnitGroup, parseEn1996UnitGroup } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeUnitGroup {
  id: string;
  newUnitGroup: En1996UnitGroup;
}

export const parseChangeUnitGroup: NormWireReader<ChangeUnitGroup> = normWireObject<ChangeUnitGroup>({ id: normWireRequired(normWireString), newUnitGroup: normWireRequired(parseEn1996UnitGroup) });
