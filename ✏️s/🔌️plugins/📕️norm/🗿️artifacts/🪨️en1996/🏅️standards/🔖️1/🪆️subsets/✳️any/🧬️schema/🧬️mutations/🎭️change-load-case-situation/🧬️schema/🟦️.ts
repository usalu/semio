/** 🎭️ `change-load-case-situation` wire twin: the leaf payload `ChangeLoadCaseSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996LoadCaseSituation, parseEn1996LoadCaseSituation } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeLoadCaseSituation {
  wallId: string;
  id: string;
  newDesignSituation: En1996LoadCaseSituation;
}

export const parseChangeLoadCaseSituation: NormWireReader<ChangeLoadCaseSituation> = normWireObject<ChangeLoadCaseSituation>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDesignSituation: normWireRequired(parseEn1996LoadCaseSituation) });
