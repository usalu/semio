/** 🧱️ `change-element-weld-position` wire twin: the leaf payload `ChangeElementWeldPosition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementWeldPosition {
  mutation: "changeElementWeldPosition";
  sectionId: string;
  elementId: string;
  newWeldPosition: number;
}

export const parseChangeElementWeldPosition: NormWireReader<ChangeElementWeldPosition> = normWireObject<ChangeElementWeldPosition>({ mutation: normWireRequired(normWireLiteral("changeElementWeldPosition")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newWeldPosition: normWireRequired(normWireNumber) });
