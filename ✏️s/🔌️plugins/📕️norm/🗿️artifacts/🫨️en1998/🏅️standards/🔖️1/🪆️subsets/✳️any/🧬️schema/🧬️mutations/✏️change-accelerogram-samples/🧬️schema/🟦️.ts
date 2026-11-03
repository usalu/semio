/** ✏️ `change-accelerogram-samples` wire twin: the leaf payload `ChangeAccelerogramSamples`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccelerogramSamples {
  mutation: "changeAccelerogramSamples";
  id: string;
  newSamples: string;
}

export const parseChangeAccelerogramSamples: NormWireReader<ChangeAccelerogramSamples> = normWireObject<ChangeAccelerogramSamples>({ mutation: normWireRequired(normWireLiteral("changeAccelerogramSamples")), id: normWireRequired(normWireString), newSamples: normWireRequired(normWireString) });
