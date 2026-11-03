/** 🔥️ `change-fire-theta-a` wire twin: the leaf payload `ChangeFireThetaA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireThetaA {
  mutation: "changeFireThetaA";
  fireScenarioId: string;
  newThetaA: number;
}

export const parseChangeFireThetaA: NormWireReader<ChangeFireThetaA> = normWireObject<ChangeFireThetaA>({ mutation: normWireRequired(normWireLiteral("changeFireThetaA")), fireScenarioId: normWireRequired(normWireString), newThetaA: normWireRequired(normWireNumber) });
