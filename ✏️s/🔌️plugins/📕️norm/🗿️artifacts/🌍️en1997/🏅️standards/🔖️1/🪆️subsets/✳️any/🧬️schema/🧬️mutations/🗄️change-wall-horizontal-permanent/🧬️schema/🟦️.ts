/** 🗄️ `change-wall-horizontal-permanent` wire twin: the leaf payload `ChangeWallHorizontalPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallHorizontalPermanent {
  mutation: "changeWallHorizontalPermanent";
  id: string;
  newHorizontalPermanent: number;
}

export const parseChangeWallHorizontalPermanent: NormWireReader<ChangeWallHorizontalPermanent> = normWireObject<ChangeWallHorizontalPermanent>({ mutation: normWireRequired(normWireLiteral("changeWallHorizontalPermanent")), id: normWireRequired(normWireString), newHorizontalPermanent: normWireRequired(normWireNumber) });
