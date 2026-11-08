/** 📍️ `change-critical-region-axial-force-n` wire twin: the leaf payload `ChangeCriticalRegionAxialForceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCriticalRegionAxialForceN {
  mutation: "changeCriticalRegionAxialForceN";
  id: string;
  newAxialForceN: number;
}

export const parseChangeCriticalRegionAxialForceN: NormWireReader<ChangeCriticalRegionAxialForceN> = normWireObject<ChangeCriticalRegionAxialForceN>({ mutation: normWireRequired(normWireLiteral("changeCriticalRegionAxialForceN")), id: normWireRequired(normWireString), newAxialForceN: normWireRequired(normWireNumber) });
