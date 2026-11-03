/** ✏️ `change-site-importance-class` wire twin: the leaf payload `ChangeSiteImportanceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImportanceClass, parseImportanceClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSiteImportanceClass {
  mutation: "changeSiteImportanceClass";
  newImportanceClass: ImportanceClass;
}

export const parseChangeSiteImportanceClass: NormWireReader<ChangeSiteImportanceClass> = normWireObject<ChangeSiteImportanceClass>({ mutation: normWireRequired(normWireLiteral("changeSiteImportanceClass")), newImportanceClass: normWireRequired(parseImportanceClass) });
