/** 🦴️ `change-conn-load-eccentricity` wire twin: the leaf payload `ChangeConnLoadEccentricity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnLoadEccentricity {
  mutation: "changeConnLoadEccentricity";
  aluminiumConnectionId: string;
  newLoadEccentricity: number;
}

export const parseChangeConnLoadEccentricity: NormWireReader<ChangeConnLoadEccentricity> = normWireObject<ChangeConnLoadEccentricity>({ mutation: normWireRequired(normWireLiteral("changeConnLoadEccentricity")), aluminiumConnectionId: normWireRequired(normWireString), newLoadEccentricity: normWireRequired(normWireNumber) });
