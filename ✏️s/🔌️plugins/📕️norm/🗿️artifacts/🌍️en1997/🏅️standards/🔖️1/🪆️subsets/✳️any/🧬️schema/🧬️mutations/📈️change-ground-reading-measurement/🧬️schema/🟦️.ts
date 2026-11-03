/** 📈 `change-ground-reading-measurement` wire twin: the leaf payload `ChangeGroundReadingMeasurement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundReadingMeasurement {
  mutation: "changeGroundReadingMeasurement";
  investigationId: string;
  id: string;
  newMeasurement: number;
}

export const parseChangeGroundReadingMeasurement: NormWireReader<ChangeGroundReadingMeasurement> = normWireObject<ChangeGroundReadingMeasurement>({ mutation: normWireRequired(normWireLiteral("changeGroundReadingMeasurement")), investigationId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMeasurement: normWireRequired(normWireNumber) });
