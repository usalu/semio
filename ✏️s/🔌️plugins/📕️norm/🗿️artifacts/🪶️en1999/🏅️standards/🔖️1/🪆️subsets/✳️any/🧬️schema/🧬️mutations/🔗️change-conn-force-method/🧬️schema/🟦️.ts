/** 🔗️ `change-conn-force-method` wire twin: the leaf payload `ChangeConnForceMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FastenerForceMethod, parseFastenerForceMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnForceMethod {
  mutation: "changeConnForceMethod";
  connectionId: string;
  newForceMethod: FastenerForceMethod;
}

export const parseChangeConnForceMethod: NormWireReader<ChangeConnForceMethod> = normWireObject<ChangeConnForceMethod>({ mutation: normWireRequired(normWireLiteral("changeConnForceMethod")), connectionId: normWireRequired(normWireString), newForceMethod: normWireRequired(parseFastenerForceMethod) });
