/** ✏️ `change-diaphragm-shear-str-mpa` wire twin: the leaf payload `ChangeDiaphragmShearStrMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmShearStrMpa {
  mutation: "changeDiaphragmShearStrMpa";
  id: string;
  newShearStrengthMpa: number;
}

export const parseChangeDiaphragmShearStrMpa: NormWireReader<ChangeDiaphragmShearStrMpa> = normWireObject<ChangeDiaphragmShearStrMpa>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmShearStrMpa")), id: normWireRequired(normWireString), newShearStrengthMpa: normWireRequired(normWireNumber) });
