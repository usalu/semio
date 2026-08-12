/** 🖨️ `change-print-target` — sets the document's `print_target` scalar (`None` clears it). */
export interface ChangePrintTarget {
  newPrintTarget: string | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`print-target` kind=`change-print-target` record=`ChangedPrintTarget`. */
export const ChangePrintTargetKind = "change-print-target" as const;
