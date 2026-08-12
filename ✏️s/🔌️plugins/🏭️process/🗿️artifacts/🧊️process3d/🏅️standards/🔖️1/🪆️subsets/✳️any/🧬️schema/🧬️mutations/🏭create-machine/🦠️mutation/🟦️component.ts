/** 🔹 `create-machine` mutation payload — adds a new workshop machine. */
export interface CreateMachine {
  index: number;
  machine: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`machine` kind=`create-machine` record=`CreatedMachine`. */
export const CreateMachineKind = "create-machine" as const;
