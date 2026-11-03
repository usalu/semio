/** 🔄️ `change-fatigue-n-cycles` wire twin: the leaf payload `ChangeFatigueNCycles`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueNCycles {
  mutation: "changeFatigueNCycles";
  fatigueDetailId: string;
  newNCycles: number;
}

export const parseChangeFatigueNCycles: NormWireReader<ChangeFatigueNCycles> = normWireObject<ChangeFatigueNCycles>({ mutation: normWireRequired(normWireLiteral("changeFatigueNCycles")), fatigueDetailId: normWireRequired(normWireString), newNCycles: normWireRequired(normWireNumber) });
