/** 📐️ `change-section-fabrication` wire twin: the leaf payload `ChangeSectionFabrication`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type HollowFabrication, parseHollowFabrication } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSectionFabrication {
  id: string;
  newFabrication: HollowFabrication;
}

export const parseChangeSectionFabrication: NormWireReader<ChangeSectionFabrication> = normWireObject<ChangeSectionFabrication>({ id: normWireRequired(normWireString), newFabrication: normWireRequired(parseHollowFabrication) });
