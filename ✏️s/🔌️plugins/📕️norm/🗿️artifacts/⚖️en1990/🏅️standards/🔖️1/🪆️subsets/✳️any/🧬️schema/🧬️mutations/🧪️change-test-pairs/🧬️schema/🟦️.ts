/** 🧪 `change-test-pairs` wire twin: the leaf payload `ChangeTestPairs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestPairs {
  mutation: "changeTestPairs";
  id: string;
  newPairs: string;
}

export const parseChangeTestPairs: NormWireReader<ChangeTestPairs> = normWireObject<ChangeTestPairs>({ mutation: normWireRequired(normWireLiteral("changeTestPairs")), id: normWireRequired(normWireString), newPairs: normWireRequired(normWireString) });
