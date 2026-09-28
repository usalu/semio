/** 🔍️ Selection projection shared by the inspector and its language-neutral contract. */
export interface FormsInspection {
  scope: "form" | "step" | "question";
  ids: string[];
  fields: string[];
}
