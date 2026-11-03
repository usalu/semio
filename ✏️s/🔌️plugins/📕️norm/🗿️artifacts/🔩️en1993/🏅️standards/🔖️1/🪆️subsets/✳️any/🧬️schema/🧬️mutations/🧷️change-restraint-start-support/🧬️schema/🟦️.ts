/** 🧷️ `change-restraint-start-support` wire twin: the leaf payload `ChangeRestraintStartSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RestraintSupport, parseRestraintSupport } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeRestraintStartSupport {
  id: string;
  newStartSupport: RestraintSupport;
}

export const parseChangeRestraintStartSupport: NormWireReader<ChangeRestraintStartSupport> = normWireObject<ChangeRestraintStartSupport>({ id: normWireRequired(normWireString), newStartSupport: normWireRequired(parseRestraintSupport) });
