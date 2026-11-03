/** ✏️ `change-accelerogram-dt-s` wire twin: the leaf payload `ChangeAccelerogramDtS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccelerogramDtS {
  mutation: "changeAccelerogramDtS";
  id: string;
  newDtS: number;
}

export const parseChangeAccelerogramDtS: NormWireReader<ChangeAccelerogramDtS> = normWireObject<ChangeAccelerogramDtS>({ mutation: normWireRequired(normWireLiteral("changeAccelerogramDtS")), id: normWireRequired(normWireString), newDtS: normWireRequired(normWireNumber) });
