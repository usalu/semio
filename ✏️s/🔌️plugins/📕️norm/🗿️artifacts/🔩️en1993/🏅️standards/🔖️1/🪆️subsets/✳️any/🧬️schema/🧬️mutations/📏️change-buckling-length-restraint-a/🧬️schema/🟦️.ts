/** 📏️ `change-buckling-length-restraint-a` wire twin: the leaf payload `ChangeBucklingLengthRestraintA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type NodeARestraint, parseNodeARestraint } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBucklingLengthRestraintA {
  id: string;
  newRestraintA: NodeARestraint;
}

export const parseChangeBucklingLengthRestraintA: NormWireReader<ChangeBucklingLengthRestraintA> = normWireObject<ChangeBucklingLengthRestraintA>({ id: normWireRequired(normWireString), newRestraintA: normWireRequired(parseNodeARestraint) });
