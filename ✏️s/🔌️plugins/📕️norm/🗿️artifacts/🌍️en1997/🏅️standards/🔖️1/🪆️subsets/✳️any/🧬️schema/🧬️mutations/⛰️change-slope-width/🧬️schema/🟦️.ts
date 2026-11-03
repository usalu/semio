/** ⛰️ `change-slope-width` wire twin: the leaf payload `ChangeSlopeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeWidth {
  mutation: "changeSlopeWidth";
  id: string;
  newWidth: number;
}

export const parseChangeSlopeWidth: NormWireReader<ChangeSlopeWidth> = normWireObject<ChangeSlopeWidth>({ mutation: normWireRequired(normWireLiteral("changeSlopeWidth")), id: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
