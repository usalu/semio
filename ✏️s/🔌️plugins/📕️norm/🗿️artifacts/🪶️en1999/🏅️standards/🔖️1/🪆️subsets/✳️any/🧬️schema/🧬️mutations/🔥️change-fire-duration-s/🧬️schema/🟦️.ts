/** 🔥️ `change-fire-duration-s` wire twin: the leaf payload `ChangeFireDurationS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireDurationS {
  mutation: "changeFireDurationS";
  fireScenarioId: string;
  newDurationS: number;
}

export const parseChangeFireDurationS: NormWireReader<ChangeFireDurationS> = normWireObject<ChangeFireDurationS>({ mutation: normWireRequired(normWireLiteral("changeFireDurationS")), fireScenarioId: normWireRequired(normWireString), newDurationS: normWireRequired(normWireNumber) });
