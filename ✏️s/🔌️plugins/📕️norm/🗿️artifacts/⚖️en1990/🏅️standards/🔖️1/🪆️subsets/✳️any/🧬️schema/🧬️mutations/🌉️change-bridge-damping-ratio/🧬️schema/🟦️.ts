/** 🌉 `change-bridge-damping-ratio` wire twin: the leaf payload `ChangeBridgeDampingRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDampingRatio {
  mutation: "changeBridgeDampingRatio";
  id: string;
  newDampingRatio: number;
}

export const parseChangeBridgeDampingRatio: NormWireReader<ChangeBridgeDampingRatio> = normWireObject<ChangeBridgeDampingRatio>({ mutation: normWireRequired(normWireLiteral("changeBridgeDampingRatio")), id: normWireRequired(normWireString), newDampingRatio: normWireRequired(normWireNumber) });
