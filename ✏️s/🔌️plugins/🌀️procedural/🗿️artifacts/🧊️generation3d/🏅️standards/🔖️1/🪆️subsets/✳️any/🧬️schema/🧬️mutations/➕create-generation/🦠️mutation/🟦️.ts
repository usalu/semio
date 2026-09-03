/** ➕ generation3d direct `create-generation` payload mirror of `CreateGeneration`. */
/** @description Mirror of `flow::playbook::FormGeneration`. */
export interface FormGeneration {
  id: string;
  name: string;
  values: Record<string, unknown>;
}

export interface CreateGeneration {
  generation: FormGeneration;
}
