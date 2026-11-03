/** 🪢️ `change-beam-support` wire twin: the leaf payload `ChangeBeamSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SupportCondition, parseSupportCondition } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBeamSupport {
  id: string;
  newSupport: SupportCondition;
}

export const parseChangeBeamSupport: NormWireReader<ChangeBeamSupport> = normWireObject<ChangeBeamSupport>({ id: normWireRequired(normWireString), newSupport: normWireRequired(parseSupportCondition) });
