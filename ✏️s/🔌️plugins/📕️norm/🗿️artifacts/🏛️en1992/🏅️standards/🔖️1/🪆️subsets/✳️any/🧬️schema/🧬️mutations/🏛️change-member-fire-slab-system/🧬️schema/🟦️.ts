/** 🏛️ `change-member-fire-slab-system` wire twin: the leaf payload `ChangeMemberFireSlabSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SlabFireSystem, parseSlabFireSystem } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberFireSlabSystem {
  memberId: string;
  newFireSlabSystem: SlabFireSystem;
}

export const parseChangeMemberFireSlabSystem: NormWireReader<ChangeMemberFireSlabSystem> = normWireObject<ChangeMemberFireSlabSystem>({ memberId: normWireRequired(normWireString), newFireSlabSystem: normWireRequired(parseSlabFireSystem) });
