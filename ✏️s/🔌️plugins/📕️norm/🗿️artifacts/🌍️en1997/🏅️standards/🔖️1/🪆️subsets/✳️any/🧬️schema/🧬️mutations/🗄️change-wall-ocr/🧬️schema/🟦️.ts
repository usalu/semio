/** 🗄️ `change-wall-ocr` wire twin: the leaf payload `ChangeWallOcr`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallOcr {
  mutation: "changeWallOcr";
  id: string;
  newOcr: number;
}

export const parseChangeWallOcr: NormWireReader<ChangeWallOcr> = normWireObject<ChangeWallOcr>({ mutation: normWireRequired(normWireLiteral("changeWallOcr")), id: normWireRequired(normWireString), newOcr: normWireRequired(normWireNumber) });
