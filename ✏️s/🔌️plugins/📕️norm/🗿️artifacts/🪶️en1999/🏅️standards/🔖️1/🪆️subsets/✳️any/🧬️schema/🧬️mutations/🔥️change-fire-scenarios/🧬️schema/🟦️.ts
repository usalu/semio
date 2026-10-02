/** 🔥️ `change-fire-scenarios` wire twin: the leaf payload `ChangeFireScenarios`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FireScenario, parseFireScenario } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireScenarios {
  mutation: "changeFireScenarios";
  fireScenarios: FireScenario[];
}

export const parseChangeFireScenarios: NormWireReader<ChangeFireScenarios> = normWireObject<ChangeFireScenarios>({ mutation: normWireRequired(normWireLiteral("changeFireScenarios")), fireScenarios: normWireRequired(normWireArray(parseFireScenario)) });
