/** 🧱 `change-plate-thickness` wire twin: the leaf payload `ChangePlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlateThickness {
  mutation: "changePlateThickness";
  sectionId: string;
  elementId: string;
  newThickness: number;
}

export const parseChangePlateThickness: NormWireReader<ChangePlateThickness> = normWireObject<ChangePlateThickness>({ mutation: normWireRequired(normWireLiteral("changePlateThickness")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newThickness: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
