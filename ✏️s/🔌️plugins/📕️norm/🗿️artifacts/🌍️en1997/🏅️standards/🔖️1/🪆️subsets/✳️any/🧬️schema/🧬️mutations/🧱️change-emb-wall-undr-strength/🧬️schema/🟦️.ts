/** 🧱 `change-emb-wall-undr-strength` wire twin: the leaf payload `ChangeEmbWallUndrStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallUndrStrength {
  mutation: "changeEmbWallUndrStrength";
  id: string;
  newUndrainedStrength: number;
}

export const parseChangeEmbWallUndrStrength: NormWireReader<ChangeEmbWallUndrStrength> = normWireObject<ChangeEmbWallUndrStrength>({ mutation: normWireRequired(normWireLiteral("changeEmbWallUndrStrength")), id: normWireRequired(normWireString), newUndrainedStrength: normWireRequired(normWireNumber) });
