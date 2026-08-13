/** 🔄️ `change-coefficient` — sets a numeric leaf's value in the equation tree, addressed by a stable `EquationNodeLabel` (never a positional path). `numer`/`denom` are decimal-integer lexemes; `denom === "1"` is a plain integer coefficient. */
export interface ChangeCoefficient {
  label: number;
  numer: string;
  denom: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`coefficient` kind=`change-coefficient` record=`ChangedCoefficient`. */
export const ChangeCoefficientKind = "change-coefficient" as const;
