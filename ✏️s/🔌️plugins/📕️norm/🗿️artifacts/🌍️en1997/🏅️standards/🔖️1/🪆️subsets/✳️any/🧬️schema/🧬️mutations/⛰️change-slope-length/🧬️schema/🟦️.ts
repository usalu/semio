/** ⛰️ `change-slope-length` wire twin: the leaf payload `ChangeSlopeLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeLength {
  mutation: "changeSlopeLength";
  id: string;
  newLength: number;
}

export const parseChangeSlopeLength: NormWireReader<ChangeSlopeLength> = normWireObject<ChangeSlopeLength>({ mutation: normWireRequired(normWireLiteral("changeSlopeLength")), id: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
