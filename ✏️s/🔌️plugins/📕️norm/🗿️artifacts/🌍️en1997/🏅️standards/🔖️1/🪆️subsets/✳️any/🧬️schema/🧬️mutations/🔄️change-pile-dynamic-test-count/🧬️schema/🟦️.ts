/** 🔄️ `change-pile-dynamic-test-count` wire twin: the leaf payload `ChangePileDynamicTestCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDynamicTestCount {
  mutation: "changePileDynamicTestCount";
  id: string;
  newDynamicTestCount: number;
}

export const parseChangePileDynamicTestCount: NormWireReader<ChangePileDynamicTestCount> = normWireObject<ChangePileDynamicTestCount>({ mutation: normWireRequired(normWireLiteral("changePileDynamicTestCount")), id: normWireRequired(normWireString), newDynamicTestCount: normWireRequired(normWireNumber) });
