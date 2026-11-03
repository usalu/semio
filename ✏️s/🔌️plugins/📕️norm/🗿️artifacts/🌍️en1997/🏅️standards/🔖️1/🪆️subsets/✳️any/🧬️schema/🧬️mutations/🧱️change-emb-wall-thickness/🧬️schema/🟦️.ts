/** 🧱 `change-emb-wall-thickness` wire twin: the leaf payload `ChangeEmbWallThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallThickness {
  mutation: "changeEmbWallThickness";
  id: string;
  newThickness: number;
}

export const parseChangeEmbWallThickness: NormWireReader<ChangeEmbWallThickness> = normWireObject<ChangeEmbWallThickness>({ mutation: normWireRequired(normWireLiteral("changeEmbWallThickness")), id: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
