//! 🚦️ Retained-editor integration contracts over lower live command definitions.

use crate::*;

#[test]
fn live_tool_contracts_cover_every_live_id_without_retained_overlap() {
    assert_eq!(crate::NORM_LIVE_PUBLICATION_CONTRACTS.iter().map(|c| c.tool_id).collect::<Vec<_>>(), NORM_LIVE_TOOL_IDS);
    assert!(NORM_LIVE_TOOL_IDS.iter().all(|id| !crate::NORM_RETAINED_TOOL_IDS.contains(id)));
}

#[test]
fn the_editor_tool_ids_are_the_retained_ids_then_the_live_ids_with_matching_contracts() {
    let expected: Vec<&str> = crate::NORM_RETAINED_TOOL_IDS.iter().chain(crate::NORM_LIVE_TOOL_IDS).copied().collect();
    assert_eq!(crate::NORM_EDITOR_TOOL_IDS, expected.as_slice());
    let contracts: Vec<&str> = crate::NORM_EDITOR_PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(contracts, expected);
    for retained in crate::NORM_PUBLICATION_CONTRACTS {
        let editor = crate::NORM_EDITOR_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == retained.tool_id).expect("retained contract is kept");
        assert_eq!(editor.lanes, retained.lanes);
    }
    for live in crate::NORM_LIVE_PUBLICATION_CONTRACTS {
        let editor = crate::NORM_EDITOR_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == live.tool_id).expect("live contract is kept");
        assert!(live.lanes.iter().all(|lane| editor.lanes.contains(lane)));
    }
}

