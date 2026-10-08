//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the EN1993 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};

/// 📚 Every path an EN1993 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
    ],
    insert_item: &[
        InsertItemRule { path: "bridgeFatigue", kind: "insert-bridge-fatigue", selectors: &[], index: "index", item: "bridgeFatigueItem" },
        InsertItemRule { path: "coldFormedMembers", kind: "insert-cold-formed-member", selectors: &[], index: "index", item: "coldFormedMember" },
        InsertItemRule { path: "craneRunways", kind: "insert-crane-runway", selectors: &[], index: "index", item: "craneRunway" },
        InsertItemRule { path: "fatigueDetails", kind: "insert-fatigue-detail", selectors: &[], index: "index", item: "fatigueDetail" },
        InsertItemRule { path: "fireExposures", kind: "insert-fire-exposure", selectors: &[], index: "index", item: "fireExposure" },
        InsertItemRule { path: "joints", kind: "insert-joint", selectors: &[], index: "index", item: "joint" },
        InsertItemRule { path: "loadCases", kind: "insert-load-case", selectors: &[], index: "index", item: "loadCase" },
        InsertItemRule { path: "materials", kind: "insert-material", selectors: &[], index: "index", item: "material" },
        InsertItemRule { path: "members", kind: "insert-member", selectors: &[], index: "index", item: "member" },
        InsertItemRule { path: "memberActions", kind: "insert-member-action", selectors: &[], index: "index", item: "memberAction" },
        InsertItemRule { path: "piles", kind: "insert-pile", selectors: &[], index: "index", item: "pile" },
        InsertItemRule { path: "platedPanels", kind: "insert-plated-panel", selectors: &[], index: "index", item: "platedPanel" },
        InsertItemRule { path: "sections", kind: "insert-section", selectors: &[], index: "index", item: "section" },
        InsertItemRule { path: "siloShells", kind: "insert-silo-shell", selectors: &[], index: "index", item: "siloShell" },
        InsertItemRule { path: "tensionComponents", kind: "insert-tension-component", selectors: &[], index: "index", item: "tensionComponent" },
        InsertItemRule { path: "towerLegs", kind: "insert-tower-leg", selectors: &[], index: "index", item: "towerLeg" },
    ],
    remove_item: &[
        RemoveItemRule { path: "bridgeFatigue", kind: "remove-bridge-fatigue", selectors: &[], index: "index" },
        RemoveItemRule { path: "coldFormedMembers", kind: "remove-cold-formed-member", selectors: &[], index: "index" },
        RemoveItemRule { path: "craneRunways", kind: "remove-crane-runway", selectors: &[], index: "index" },
        RemoveItemRule { path: "fatigueDetails", kind: "remove-fatigue-detail", selectors: &[], index: "index" },
        RemoveItemRule { path: "fireExposures", kind: "remove-fire-exposure", selectors: &[], index: "index" },
        RemoveItemRule { path: "joints", kind: "remove-joint", selectors: &[], index: "index" },
        RemoveItemRule { path: "loadCases", kind: "remove-load-case", selectors: &[], index: "index" },
        RemoveItemRule { path: "materials", kind: "remove-material", selectors: &[], index: "index" },
        RemoveItemRule { path: "members", kind: "remove-member", selectors: &[], index: "index" },
        RemoveItemRule { path: "memberActions", kind: "remove-member-action", selectors: &[], index: "index" },
        RemoveItemRule { path: "piles", kind: "remove-pile", selectors: &[], index: "index" },
        RemoveItemRule { path: "platedPanels", kind: "remove-plated-panel", selectors: &[], index: "index" },
        RemoveItemRule { path: "sections", kind: "remove-section", selectors: &[], index: "index" },
        RemoveItemRule { path: "siloShells", kind: "remove-silo-shell", selectors: &[], index: "index" },
        RemoveItemRule { path: "tensionComponents", kind: "remove-tension-component", selectors: &[], index: "index" },
        RemoveItemRule { path: "towerLegs", kind: "remove-tower-leg", selectors: &[], index: "index" },
    ],
};
