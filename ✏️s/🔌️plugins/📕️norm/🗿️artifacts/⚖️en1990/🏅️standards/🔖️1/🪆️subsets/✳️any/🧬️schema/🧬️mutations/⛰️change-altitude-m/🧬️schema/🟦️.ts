/** ⛰️ `change-altitude-m` wire twin: the leaf payload `ChangeAltitudeM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAltitudeM {
  mutation: "changeAltitudeM";
  newAltitudeM: number;
}

export const parseChangeAltitudeM: NormWireReader<ChangeAltitudeM> = normWireObject<ChangeAltitudeM>({ mutation: normWireRequired(normWireLiteral("changeAltitudeM")), newAltitudeM: normWireRequired(normWireNumber) });
