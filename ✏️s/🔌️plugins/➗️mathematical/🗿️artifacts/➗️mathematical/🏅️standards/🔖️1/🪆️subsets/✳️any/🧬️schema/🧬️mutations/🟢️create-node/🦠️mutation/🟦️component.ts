/** 🟢️ `create-node` — brings a new id-keyed graph node into existence. */
export interface CreateNode {
  id: string;
  label: string;
  x: number;
  y: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`node` kind=`create-node` record=`CreatedNode`. */
export const CreateNodeKind = "create-node" as const;
