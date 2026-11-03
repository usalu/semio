/** 🗄️ `change-wall-vertical-permanent` wire twin: the leaf payload `ChangeWallVerticalPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallVerticalPermanent {
  mutation: "changeWallVerticalPermanent";
  id: string;
  newVerticalPermanent: number;
}

export const parseChangeWallVerticalPermanent: NormWireReader<ChangeWallVerticalPermanent> = normWireObject<ChangeWallVerticalPermanent>({ mutation: normWireRequired(normWireLiteral("changeWallVerticalPermanent")), id: normWireRequired(normWireString), newVerticalPermanent: normWireRequired(normWireNumber) });
