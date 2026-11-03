/** ✏️ `change-silo-filling-ratio` wire twin: the leaf payload `ChangeSiloFillingRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloFillingRatio {
  mutation: "changeSiloFillingRatio";
  id: string;
  newFillingRatio: number;
}

export const parseChangeSiloFillingRatio: NormWireReader<ChangeSiloFillingRatio> = normWireObject<ChangeSiloFillingRatio>({ mutation: normWireRequired(normWireLiteral("changeSiloFillingRatio")), id: normWireRequired(normWireString), newFillingRatio: normWireRequired(normWireNumber) });
