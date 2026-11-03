/** 🧪️ `change-lab-test-swelling-index` wire twin: the leaf payload `ChangeLabTestSwellingIndex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLabTestSwellingIndex {
  mutation: "changeLabTestSwellingIndex";
  id: string;
  newSwellingIndex: number;
}

export const parseChangeLabTestSwellingIndex: NormWireReader<ChangeLabTestSwellingIndex> = normWireObject<ChangeLabTestSwellingIndex>({ mutation: normWireRequired(normWireLiteral("changeLabTestSwellingIndex")), id: normWireRequired(normWireString), newSwellingIndex: normWireRequired(normWireNumber) });
