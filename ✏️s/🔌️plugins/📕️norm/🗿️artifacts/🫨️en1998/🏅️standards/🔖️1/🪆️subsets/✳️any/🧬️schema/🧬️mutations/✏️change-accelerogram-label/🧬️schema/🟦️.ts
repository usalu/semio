/** ✏️ `change-accelerogram-label` wire twin: the leaf payload `ChangeAccelerogramLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccelerogramLabel {
  mutation: "changeAccelerogramLabel";
  id: string;
  newLabel: string;
}

export const parseChangeAccelerogramLabel: NormWireReader<ChangeAccelerogramLabel> = normWireObject<ChangeAccelerogramLabel>({ mutation: normWireRequired(normWireLiteral("changeAccelerogramLabel")), id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
