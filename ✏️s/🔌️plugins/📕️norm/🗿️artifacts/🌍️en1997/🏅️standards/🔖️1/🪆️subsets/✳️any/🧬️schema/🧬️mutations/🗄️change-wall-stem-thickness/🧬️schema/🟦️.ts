/** 🗄️ `change-wall-stem-thickness` wire twin: the leaf payload `ChangeWallStemThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallStemThickness {
  mutation: "changeWallStemThickness";
  id: string;
  newStemThickness: number;
}

export const parseChangeWallStemThickness: NormWireReader<ChangeWallStemThickness> = normWireObject<ChangeWallStemThickness>({ mutation: normWireRequired(normWireLiteral("changeWallStemThickness")), id: normWireRequired(normWireString), newStemThickness: normWireRequired(normWireNumber) });
