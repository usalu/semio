/** 📅️ `change-design-situation` wire twin: the leaf payload `ChangeDesignSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SituationChoice, parseSituationChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDesignSituation {
  mutation: "changeDesignSituation";
  newDesignSituation: SituationChoice;
}

export const parseChangeDesignSituation: NormWireReader<ChangeDesignSituation> = normWireObject<ChangeDesignSituation>({ mutation: normWireRequired(normWireLiteral("changeDesignSituation")), newDesignSituation: normWireRequired(parseSituationChoice) });
