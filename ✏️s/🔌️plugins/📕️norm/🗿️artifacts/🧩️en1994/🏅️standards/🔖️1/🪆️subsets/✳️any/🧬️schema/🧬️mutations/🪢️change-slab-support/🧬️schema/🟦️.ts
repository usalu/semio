/** 🪢️ `change-slab-support` wire twin: the leaf payload `ChangeSlabSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SupportCondition, parseSupportCondition } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSlabSupport {
  id: string;
  newSupport: SupportCondition;
}

export const parseChangeSlabSupport: NormWireReader<ChangeSlabSupport> = normWireObject<ChangeSlabSupport>({ id: normWireRequired(normWireString), newSupport: normWireRequired(parseSupportCondition) });
