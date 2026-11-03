/** 🏛️ `change-load-case-design-situation` wire twin: the leaf payload `ChangeLoadCaseDesignSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SituationChoice, parseSituationChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeLoadCaseDesignSituation {
  mutation: "changeLoadCaseDesignSituation";
  footingId: string;
  id: string;
  newDesignSituation: SituationChoice;
}

export const parseChangeLoadCaseDesignSituation: NormWireReader<ChangeLoadCaseDesignSituation> = normWireObject<ChangeLoadCaseDesignSituation>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseDesignSituation")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDesignSituation: normWireRequired(parseSituationChoice) });
