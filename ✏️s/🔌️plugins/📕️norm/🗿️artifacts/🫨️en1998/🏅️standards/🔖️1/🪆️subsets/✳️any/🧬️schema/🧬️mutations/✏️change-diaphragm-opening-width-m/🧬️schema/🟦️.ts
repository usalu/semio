/** ✏️ `change-diaphragm-opening-width-m` wire twin: the leaf payload `ChangeDiaphragmOpeningWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmOpeningWidthM {
  mutation: "changeDiaphragmOpeningWidthM";
  id: string;
  newOpeningWidthM: number;
}

export const parseChangeDiaphragmOpeningWidthM: NormWireReader<ChangeDiaphragmOpeningWidthM> = normWireObject<ChangeDiaphragmOpeningWidthM>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmOpeningWidthM")), id: normWireRequired(normWireString), newOpeningWidthM: normWireRequired(normWireNumber) });
