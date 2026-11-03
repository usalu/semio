/** 🌋 `change-seismic-importance-class` wire twin: the leaf payload `ChangeSeismicImportanceClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicImportanceClass {
  mutation: "changeSeismicImportanceClass";
  id: string;
  newImportanceClass: "I" | "II" | "III" | "IV";
}

export const parseChangeSeismicImportanceClass: NormWireReader<ChangeSeismicImportanceClass> = normWireObject<ChangeSeismicImportanceClass>({ mutation: normWireRequired(normWireLiteral("changeSeismicImportanceClass")), id: normWireRequired(normWireString), newImportanceClass: normWireRequired(normWireLiteral("I", "II", "III", "IV")) });
