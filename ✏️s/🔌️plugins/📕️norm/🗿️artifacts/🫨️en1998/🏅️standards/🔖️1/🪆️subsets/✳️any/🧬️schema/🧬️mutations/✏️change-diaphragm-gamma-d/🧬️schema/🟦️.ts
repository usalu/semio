/** ✏️ `change-diaphragm-gamma-d` wire twin: the leaf payload `ChangeDiaphragmGammaD`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmGammaD {
  mutation: "changeDiaphragmGammaD";
  id: string;
  newGammaD: number;
}

export const parseChangeDiaphragmGammaD: NormWireReader<ChangeDiaphragmGammaD> = normWireObject<ChangeDiaphragmGammaD>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmGammaD")), id: normWireRequired(normWireString), newGammaD: normWireRequired(normWireNumber) });
