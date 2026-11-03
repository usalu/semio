/** 🧱️ `change-element-outstand` wire twin: the leaf payload `ChangeElementOutstand`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementOutstand {
  mutation: "changeElementOutstand";
  sectionId: string;
  elementId: string;
  newOutstand: boolean;
}

export const parseChangeElementOutstand: NormWireReader<ChangeElementOutstand> = normWireObject<ChangeElementOutstand>({ mutation: normWireRequired(normWireLiteral("changeElementOutstand")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newOutstand: normWireRequired(normWireBoolean) });
