/** 🪜️ `change-built-up-chain-spacing` wire twin: the leaf payload `ChangeBuiltUpChainSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpChainSpacing {
  id: string;
  newChainSpacing: number;
}

export const parseChangeBuiltUpChainSpacing: NormWireReader<ChangeBuiltUpChainSpacing> = normWireObject<ChangeBuiltUpChainSpacing>({ id: normWireRequired(normWireString), newChainSpacing: normWireRequired(normWireNumber) });
