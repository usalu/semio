/** 🧷️ `change-restraint-end-support` wire twin: the leaf payload `ChangeRestraintEndSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RestraintSupport, parseRestraintSupport } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeRestraintEndSupport {
  id: string;
  newEndSupport: RestraintSupport;
}

export const parseChangeRestraintEndSupport: NormWireReader<ChangeRestraintEndSupport> = normWireObject<ChangeRestraintEndSupport>({ id: normWireRequired(normWireString), newEndSupport: normWireRequired(parseRestraintSupport) });
