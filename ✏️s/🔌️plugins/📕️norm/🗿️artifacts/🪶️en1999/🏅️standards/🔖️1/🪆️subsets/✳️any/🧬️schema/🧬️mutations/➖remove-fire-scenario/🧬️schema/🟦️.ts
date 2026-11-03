/** 🗂️ `remove-fire-scenario` wire twin: the leaf payload `RemoveFireScenario`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFireScenario {
  mutation: "removeFireScenario";
  id: string;
}

export const parseRemoveFireScenario: NormWireReader<RemoveFireScenario> = normWireObject<RemoveFireScenario>({ mutation: normWireRequired(normWireLiteral("removeFireScenario")), id: normWireRequired(normWireString) });
