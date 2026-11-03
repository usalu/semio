/** 📍️ `remove-placement` wire twin: the leaf payload `RemovePlacement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePlacement {
  mutation: "removePlacement";
  id: string;
}

export const parseRemovePlacement: NormWireReader<RemovePlacement> = normWireObject<RemovePlacement>({ mutation: normWireRequired(normWireLiteral("removePlacement")), id: normWireRequired(normWireString) });
