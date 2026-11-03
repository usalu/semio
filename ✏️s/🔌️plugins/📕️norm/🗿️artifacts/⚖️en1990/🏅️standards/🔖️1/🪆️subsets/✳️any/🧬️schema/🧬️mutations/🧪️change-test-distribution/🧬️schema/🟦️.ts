/** 🧪 `change-test-distribution` wire twin: the leaf payload `ChangeTestDistribution`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990TestDistribution, parseEn1990TestDistribution } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTestDistribution {
  mutation: "changeTestDistribution";
  id: string;
  newDistribution: En1990TestDistribution;
}

export const parseChangeTestDistribution: NormWireReader<ChangeTestDistribution> = normWireObject<ChangeTestDistribution>({ mutation: normWireRequired(normWireLiteral("changeTestDistribution")), id: normWireRequired(normWireString), newDistribution: normWireRequired(parseEn1990TestDistribution) });
