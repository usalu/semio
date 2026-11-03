/** ➖️ `remove-plated-panel` wire twin: the leaf payload `RemovePlatedPanel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePlatedPanel {
  index: number;
}

export const parseRemovePlatedPanel: NormWireReader<RemovePlatedPanel> = normWireObject<RemovePlatedPanel>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
