/** 🔄 `change-fatigue-cycles` wire twin: the leaf payload `ChangeFatigueCycles`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueCycles {
  mutation: "changeFatigueCycles";
  id: string;
  newCycles: number;
}

export const parseChangeFatigueCycles: NormWireReader<ChangeFatigueCycles> = normWireObject<ChangeFatigueCycles>({ mutation: normWireRequired(normWireLiteral("changeFatigueCycles")), id: normWireRequired(normWireString), newCycles: normWireRequired(normWireNumber) });
