//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the EN1990 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};

/// 📚 Every path an EN1990 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "referencePeriodYears", kind: "change-reference-period-years", selectors: &[], value: "newReferencePeriodYears" },
        SetFieldRule { path: "consequenceClass", kind: "change-consequence-class", selectors: &[], value: "newConsequenceClass" },
        SetFieldRule { path: "altitudeM", kind: "change-altitude-m", selectors: &[], value: "newAltitudeM" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "reliabilityClass", kind: "change-reliability-class", selectors: &[], value: "newReliabilityClass" },
        SetFieldRule { path: "projectId", kind: "change-project-id", selectors: &[], value: "newProjectId" },
        SetFieldRule { path: "supervisionLevel", kind: "change-supervision-level", selectors: &[], value: "newSupervisionLevel" },
        SetFieldRule { path: "designWorkingLifeCategory", kind: "change-design-working-life-category", selectors: &[], value: "newDesignWorkingLifeCategory" },
        SetFieldRule { path: "designWorkingLifeYears", kind: "change-design-working-life-years", selectors: &[], value: "newDesignWorkingLifeYears" },
        SetFieldRule { path: "betaComputed", kind: "change-beta-computed", selectors: &[], value: "newBetaComputed" },
        SetFieldRule { path: "inspectionLevel", kind: "change-inspection-level", selectors: &[], value: "newInspectionLevel" },
    ],
    insert_item: &[
        InsertItemRule { path: "permanents", kind: "insert-permanent", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "seismics", kind: "insert-seismic", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "accidentals", kind: "insert-accidental", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "effects", kind: "insert-effect", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "variables", kind: "insert-variable", selectors: &[], index: "index", item: "item" },
        InsertItemRule { path: "members", kind: "insert-member", selectors: &[], index: "index", item: "item" },
    ],
    remove_item: &[
        RemoveItemRule { path: "effects", kind: "remove-effect", selectors: &[], index: "index" },
        RemoveItemRule { path: "permanents", kind: "remove-permanent", selectors: &[], index: "index" },
        RemoveItemRule { path: "variables", kind: "remove-variable", selectors: &[], index: "index" },
        RemoveItemRule { path: "seismics", kind: "remove-seismic", selectors: &[], index: "index" },
        RemoveItemRule { path: "accidentals", kind: "remove-accidental", selectors: &[], index: "index" },
        RemoveItemRule { path: "members", kind: "remove-member", selectors: &[], index: "index" },
    ],
};
