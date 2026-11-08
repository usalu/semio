/** 📈️ ISO 16757 viewer — diagrams window: typed twin of `🦀️.rs`, the eccentricity, reduction factor, capacity, slenderness, strength, shear, lateral, fire and M–N diagrams with table twins. */

export interface Iso16757DiagramsViewModel {
  windowKindId: "norm-iso16757-diagrams";
  bodyKey: "norm.iso16757.view.diagrams";
}

export const ISO16757_VIEW_DIAGRAMS_WINDOW_KIND_ID = "norm-iso16757-diagrams" as const;
export const ISO16757_VIEW_DIAGRAMS_BODY_KEY = "norm.iso16757.view.diagrams" as const;
