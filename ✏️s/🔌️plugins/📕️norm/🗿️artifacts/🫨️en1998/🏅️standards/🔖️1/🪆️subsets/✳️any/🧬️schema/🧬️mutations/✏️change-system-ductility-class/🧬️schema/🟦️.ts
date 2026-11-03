/** ✏️ `change-system-ductility-class` wire twin: the leaf payload `ChangeSystemDuctilityClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DuctilityClass, parseDuctilityClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSystemDuctilityClass {
  mutation: "changeSystemDuctilityClass";
  buildingId: string;
  id: string;
  newDuctilityClass: DuctilityClass;
}

export const parseChangeSystemDuctilityClass: NormWireReader<ChangeSystemDuctilityClass> = normWireObject<ChangeSystemDuctilityClass>({ mutation: normWireRequired(normWireLiteral("changeSystemDuctilityClass")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDuctilityClass: normWireRequired(parseDuctilityClass) });
