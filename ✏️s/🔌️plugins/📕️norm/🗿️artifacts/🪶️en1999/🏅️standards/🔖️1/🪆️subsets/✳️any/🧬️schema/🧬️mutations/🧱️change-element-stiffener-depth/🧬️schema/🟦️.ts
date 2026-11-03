/** 🧱️ `change-element-stiffener-depth` wire twin: the leaf payload `ChangeElementStiffenerDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementStiffenerDepth {
  mutation: "changeElementStiffenerDepth";
  sectionId: string;
  elementId: string;
  newEdgeStiffenerDepth: number;
}

export const parseChangeElementStiffenerDepth: NormWireReader<ChangeElementStiffenerDepth> = normWireObject<ChangeElementStiffenerDepth>({ mutation: normWireRequired(normWireLiteral("changeElementStiffenerDepth")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newEdgeStiffenerDepth: normWireRequired(normWireNumber) });
