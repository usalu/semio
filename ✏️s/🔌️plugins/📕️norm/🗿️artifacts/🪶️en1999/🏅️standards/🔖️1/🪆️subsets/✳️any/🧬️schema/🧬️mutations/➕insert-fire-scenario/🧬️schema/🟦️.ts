/** 🗂️ `insert-fire-scenario` wire twin: the leaf payload `InsertFireScenario`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FireScenario, parseFireScenario } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFireScenario {
  mutation: "insertFireScenario";
  index: number;
  fireScenario: FireScenario;
}

export const parseInsertFireScenario: NormWireReader<InsertFireScenario> = normWireObject<InsertFireScenario>({ mutation: normWireRequired(normWireLiteral("insertFireScenario")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fireScenario: normWireRequired(parseFireScenario) });
