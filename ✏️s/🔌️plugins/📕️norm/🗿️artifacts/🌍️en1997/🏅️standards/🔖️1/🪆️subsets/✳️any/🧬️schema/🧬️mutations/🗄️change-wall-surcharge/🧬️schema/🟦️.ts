/** 🗄️ `change-wall-surcharge` wire twin: the leaf payload `ChangeWallSurcharge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallSurcharge {
  mutation: "changeWallSurcharge";
  id: string;
  newSurcharge: number;
}

export const parseChangeWallSurcharge: NormWireReader<ChangeWallSurcharge> = normWireObject<ChangeWallSurcharge>({ mutation: normWireRequired(normWireLiteral("changeWallSurcharge")), id: normWireRequired(normWireString), newSurcharge: normWireRequired(normWireNumber) });
