/** 🌉 `change-bridge-sls` wire twin: the leaf payload `ChangeBridgeSls`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990BridgeSls, parseEn1990BridgeSls } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBridgeSls {
  mutation: "changeBridgeSls";
  newBridgeSls: En1990BridgeSls[];
}

export const parseChangeBridgeSls: NormWireReader<ChangeBridgeSls> = normWireObject<ChangeBridgeSls>({ mutation: normWireRequired(normWireLiteral("changeBridgeSls")), newBridgeSls: normWireRequired(normWireArray(parseEn1990BridgeSls)) });
