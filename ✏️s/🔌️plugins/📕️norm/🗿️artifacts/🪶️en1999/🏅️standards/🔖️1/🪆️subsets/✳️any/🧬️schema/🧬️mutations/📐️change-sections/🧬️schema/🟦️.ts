/** 📐️ `change-sections` wire twin: the leaf payload `ChangeSections`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumSection, parseAluminiumSection } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSections {
  mutation: "changeSections";
  sections: AluminiumSection[];
}

export const parseChangeSections: NormWireReader<ChangeSections> = normWireObject<ChangeSections>({ mutation: normWireRequired(normWireLiteral("changeSections")), sections: normWireRequired(normWireArray(parseAluminiumSection)) });
