/** 🧭️ `change-design-approach` wire twin: the leaf payload `ChangeDesignApproach`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ApproachChoice, parseApproachChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDesignApproach {
  mutation: "changeDesignApproach";
  newDesignApproach: ApproachChoice;
}

export const parseChangeDesignApproach: NormWireReader<ChangeDesignApproach> = normWireObject<ChangeDesignApproach>({ mutation: normWireRequired(normWireLiteral("changeDesignApproach")), newDesignApproach: normWireRequired(parseApproachChoice) });
