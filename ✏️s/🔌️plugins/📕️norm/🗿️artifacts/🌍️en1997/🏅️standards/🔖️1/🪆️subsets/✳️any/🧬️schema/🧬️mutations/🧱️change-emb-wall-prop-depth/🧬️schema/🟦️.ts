/** 🧱 `change-emb-wall-prop-depth` wire twin: the leaf payload `ChangeEmbWallPropDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEmbWallPropDepth {
  mutation: "changeEmbWallPropDepth";
  id: string;
  newPropDepth: number;
}

export const parseChangeEmbWallPropDepth: NormWireReader<ChangeEmbWallPropDepth> = normWireObject<ChangeEmbWallPropDepth>({ mutation: normWireRequired(normWireLiteral("changeEmbWallPropDepth")), id: normWireRequired(normWireString), newPropDepth: normWireRequired(normWireNumber) });
