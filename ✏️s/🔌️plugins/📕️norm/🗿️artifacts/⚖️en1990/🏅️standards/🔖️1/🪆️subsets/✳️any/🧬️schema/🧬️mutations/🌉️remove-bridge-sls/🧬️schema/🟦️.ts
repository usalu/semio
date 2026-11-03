/** 🌉 `remove-bridge-sls` wire twin: the leaf payload `RemoveBridgeSls`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBridgeSls {
  mutation: "removeBridgeSls";
  index: number;
}

export const parseRemoveBridgeSls: NormWireReader<RemoveBridgeSls> = normWireObject<RemoveBridgeSls>({ mutation: normWireRequired(normWireLiteral("removeBridgeSls")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
