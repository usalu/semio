/** 📈 `change-ground-reading-depth` wire twin: the leaf payload `ChangeGroundReadingDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundReadingDepth {
  mutation: "changeGroundReadingDepth";
  investigationId: string;
  id: string;
  newDepth: number;
}

export const parseChangeGroundReadingDepth: NormWireReader<ChangeGroundReadingDepth> = normWireObject<ChangeGroundReadingDepth>({ mutation: normWireRequired(normWireLiteral("changeGroundReadingDepth")), investigationId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });
