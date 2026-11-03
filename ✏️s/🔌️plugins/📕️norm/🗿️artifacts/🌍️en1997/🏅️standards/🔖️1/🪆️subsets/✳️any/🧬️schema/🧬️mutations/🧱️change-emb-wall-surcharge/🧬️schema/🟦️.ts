/** 🧱 `change-emb-wall-surcharge` wire twin: the leaf payload `ChangeEmbWallSurcharge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallSurcharge {
  mutation: "changeEmbWallSurcharge";
  id: string;
  newSurcharge: number;
}

export const parseChangeEmbWallSurcharge: NormWireReader<ChangeEmbWallSurcharge> = normWireObject<ChangeEmbWallSurcharge>({ mutation: normWireRequired(normWireLiteral("changeEmbWallSurcharge")), id: normWireRequired(normWireString), newSurcharge: normWireRequired(normWireNumber) });
