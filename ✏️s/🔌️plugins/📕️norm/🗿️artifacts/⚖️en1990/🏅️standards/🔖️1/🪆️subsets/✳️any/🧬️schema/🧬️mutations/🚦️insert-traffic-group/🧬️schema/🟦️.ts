/** 🚦 `insert-traffic-group` wire twin: the leaf payload `InsertTrafficGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990TrafficGroup, parseEn1990TrafficGroup } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTrafficGroup {
  mutation: "insertTrafficGroup";
  index?: number | null;
  item: En1990TrafficGroup;
}

export const parseInsertTrafficGroup: NormWireReader<InsertTrafficGroup> = normWireObject<InsertTrafficGroup>({ mutation: normWireRequired(normWireLiteral("insertTrafficGroup")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), item: normWireRequired(parseEn1990TrafficGroup) });
