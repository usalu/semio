/** 🎭️ `change-design-situation` wire twin: the leaf payload `ChangeDesignSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996DesignSituation, parseEn1996DesignSituation } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDesignSituation {
  newDesignSituation: En1996DesignSituation;
}

export const parseChangeDesignSituation: NormWireReader<ChangeDesignSituation> = normWireObject<ChangeDesignSituation>({ newDesignSituation: normWireRequired(parseEn1996DesignSituation) });
