/** 🛤 `change-snow-guard-roof-id` wire twin: the leaf payload `ChangeSnowGuardRoofId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSnowGuardRoofId {
  id: string;
  newRoofId: string;
}

export const parseChangeSnowGuardRoofId: NormWireReader<ChangeSnowGuardRoofId> = normWireObject<ChangeSnowGuardRoofId>({ id: normWireRequired(normWireString), newRoofId: normWireRequired(normWireString) });
