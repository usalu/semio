/** 🧱️ `change-element-stiffener-thk` wire twin: the leaf payload `ChangeElementStiffenerThk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementStiffenerThk {
  mutation: "changeElementStiffenerThk";
  sectionId: string;
  elementId: string;
  newEdgeStiffenerThickness: number;
}

export const parseChangeElementStiffenerThk: NormWireReader<ChangeElementStiffenerThk> = normWireObject<ChangeElementStiffenerThk>({ mutation: normWireRequired(normWireLiteral("changeElementStiffenerThk")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newEdgeStiffenerThickness: normWireRequired(normWireNumber) });
