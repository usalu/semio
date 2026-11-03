/** ➖️ `remove-tension-component` wire twin: the leaf payload `RemoveTensionComponent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTensionComponent {
  index: number;
}

export const parseRemoveTensionComponent: NormWireReader<RemoveTensionComponent> = normWireObject<RemoveTensionComponent>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
