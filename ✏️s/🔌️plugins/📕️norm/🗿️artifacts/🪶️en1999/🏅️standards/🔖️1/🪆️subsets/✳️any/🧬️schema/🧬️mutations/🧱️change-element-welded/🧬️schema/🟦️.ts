/** 🧱️ `change-element-welded` wire twin: the leaf payload `ChangeElementWelded`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementWelded {
  mutation: "changeElementWelded";
  sectionId: string;
  elementId: string;
  newWelded: boolean;
}

export const parseChangeElementWelded: NormWireReader<ChangeElementWelded> = normWireObject<ChangeElementWelded>({ mutation: normWireRequired(normWireLiteral("changeElementWelded")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newWelded: normWireRequired(normWireBoolean) });
