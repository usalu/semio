/** 🧪 `change-test-theory-variation` wire twin: the leaf payload `ChangeTestTheoryVariation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestTheoryVariation {
  mutation: "changeTestTheoryVariation";
  id: string;
  newTheoryVariation: number;
}

export const parseChangeTestTheoryVariation: NormWireReader<ChangeTestTheoryVariation> = normWireObject<ChangeTestTheoryVariation>({ mutation: normWireRequired(normWireLiteral("changeTestTheoryVariation")), id: normWireRequired(normWireString), newTheoryVariation: normWireRequired(normWireNumber) });
