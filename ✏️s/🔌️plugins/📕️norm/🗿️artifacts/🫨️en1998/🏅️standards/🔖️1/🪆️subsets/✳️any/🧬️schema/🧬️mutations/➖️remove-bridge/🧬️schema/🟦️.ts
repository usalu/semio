/** ➖️ `remove-bridge` wire twin: the leaf payload `RemoveBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBridge {
  mutation: "removeBridge";
  id: string;
}

export const parseRemoveBridge: NormWireReader<RemoveBridge> = normWireObject<RemoveBridge>({ mutation: normWireRequired(normWireLiteral("removeBridge")), id: normWireRequired(normWireString) });
