/** 📏️ `change-member-ltb-rest-spacing` wire twin: the leaf payload `ChangeMemberLtbRestSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLtbRestSpacing {
  id: string;
  newLtbRestraintSpacing: number;
}

export const parseChangeMemberLtbRestSpacing: NormWireReader<ChangeMemberLtbRestSpacing> = normWireObject<ChangeMemberLtbRestSpacing>({ id: normWireRequired(normWireString), newLtbRestraintSpacing: normWireRequired(normWireNumber) });
