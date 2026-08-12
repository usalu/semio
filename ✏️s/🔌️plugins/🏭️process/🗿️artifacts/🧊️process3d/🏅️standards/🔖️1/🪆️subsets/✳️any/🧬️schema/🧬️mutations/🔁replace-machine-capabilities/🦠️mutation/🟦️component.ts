/** 🔹 `replace-machine-capabilities` mutation payload — whole-value swaps a workshop machine's capability list. */
export interface ReplaceMachineCapabilities {
  id: string;
  newCapabilities: unknown[];
}

/** 🔖️ Semantic descriptor mirror: verb=`replace` entity=`machine` kind=`replace-machine-capabilities` record=`ReplacedMachineCapabilities`. */
export const ReplaceMachineCapabilitiesKind = "replace-machine-capabilities" as const;
