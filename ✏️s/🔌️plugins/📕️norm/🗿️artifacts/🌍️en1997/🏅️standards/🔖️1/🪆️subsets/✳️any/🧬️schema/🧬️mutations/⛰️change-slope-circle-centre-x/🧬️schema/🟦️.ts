/** ⛰️ `change-slope-circle-centre-x` wire twin: the leaf payload `ChangeSlopeCircleCentreX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeCircleCentreX {
  mutation: "changeSlopeCircleCentreX";
  id: string;
  newCircleCentreX: number;
}

export const parseChangeSlopeCircleCentreX: NormWireReader<ChangeSlopeCircleCentreX> = normWireObject<ChangeSlopeCircleCentreX>({ mutation: normWireRequired(normWireLiteral("changeSlopeCircleCentreX")), id: normWireRequired(normWireString), newCircleCentreX: normWireRequired(normWireNumber) });
