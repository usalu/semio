/** 📐 `remove-imperfection` wire twin: the leaf payload `RemoveImperfection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveImperfection {
  id: string;
}

export const parseRemoveImperfection: NormWireReader<RemoveImperfection> = normWireObject<RemoveImperfection>({ id: normWireRequired(normWireString) });
