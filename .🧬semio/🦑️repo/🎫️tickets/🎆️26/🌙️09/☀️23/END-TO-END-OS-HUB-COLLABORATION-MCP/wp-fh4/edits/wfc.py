"""🀄️ wfc (P2-X): bitmap, 3d, grid3d drift sites — targets naming the element, the palette-index guard without its unused
binding, the connect no-op naming the edge that already relates the slots; tests, fixtures and the Python references move
to the frozen codes."""
BITMAP = "🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/"
WFC3D = "🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/"
M3 = WFC3D + "🧬️schema/🧬️mutations/"
MB = BITMAP + "🧬️schema/🧬️mutations/"

OVERRIDES = {
    (MB + "🖌️set-input-pixels/🔺️diff/🦀️.rs", 18): {"skip": True},
    (MB + "🖍️change-palette-color/🔺️diff/🦀️.rs", 8): {"target": '["palette".to_string(), payload.index.to_string()]'},
    (M3 + "⚖️change-tile-weight/🔺️diff/🦀️.rs", 15): {"at": "payload.id.clone()"},
    (M3 + "📌️pin-slot/🔺️diff/🦀️.rs", 15): {"at": "payload.id.clone()"},
    (M3 + "📍️unpin-slot/🔺️diff/🦀️.rs", 12): {"at": "payload.id.clone()"},
    (M3 + "📐️resize-slot/🔺️diff/🦀️.rs", 16): {"at": "payload.id.clone()"},
    (M3 + "🔗️connect-slots/🔺️diff/🦀️.rs", 26): {"skip": True},
    (M3 + "🕳️delete-slot/🔺️diff/🦀️.rs", 16): {"at": "payload.id.clone()"},
    (M3 + "🖼️change-tile-media/🔺️diff/🦀️.rs", 12): {"at": "payload.id.clone()"},
    (M3 + "🗑️delete-tile/🔺️diff/🦀️.rs", 29): {"at": "payload.id.clone()"},
    (M3 + "🚚️move-slot/🔺️diff/🦀️.rs", 12): {"at": "payload.id.clone()"},
}

EDITS = [
    (MB + "🖌️set-input-pixels/🔺️diff/🦀️.rs",
     '''    if let Some(index) = region.iter().find(|index| usize::from(**index) >= base.input.palette.len()) {
        return protocol::MutationOutcome::fatal("mutation.unknown-palette-color", format!("Palette index {index} is not in this document's palette."), ["pixels".to_string()]);
    }''',
     '''    if region.iter().any(|index| usize::from(*index) >= base.input.palette.len()) {
        return protocol::MutationOutcome::fatal(protocol::MutationCode::Invariant, ["pixels".to_string()]);
    }'''),
    (M3 + "🔗️connect-slots/🔺️diff/🦀️.rs",
     '''    if base
        .edges
        .iter()
        .any(|existing| existing.relation == edge.relation && ((existing.from_slot_id == edge.from_slot_id && existing.to_slot_id == edge.to_slot_id) || (existing.from_slot_id == edge.to_slot_id && existing.to_slot_id == edge.from_slot_id)))
    {
        return protocol::MutationOutcome::empty().warn("wfc3d.edge.already-connected", format!("\\"{}\\" already relates to \\"{}\\" as \\"{}\\".", edge.from_slot_id, edge.to_slot_id, edge.relation));
    }''',
     '''    if let Some(existing) = base
        .edges
        .iter()
        .find(|existing| existing.relation == edge.relation && ((existing.from_slot_id == edge.from_slot_id && existing.to_slot_id == edge.to_slot_id) || (existing.from_slot_id == edge.to_slot_id && existing.to_slot_id == edge.from_slot_id)))
    {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn(protocol::MutationCode::NoOp).at(vec![existing.id.clone()])]);
    }'''),
]

RENAMES = [
    (BITMAP, {"mutation.missing-target": "TargetMissing", "mutation.unknown-palette-color": "Invariant", "mutation.colour-in-use": "Invariant",
              "mutation.malformed-payload": "Invariant"}),
    (WFC3D, {"wfc3d.tile.missing": "TargetMissing", "wfc3d.slot.missing": "TargetMissing", "wfc3d.edge.missing": "TargetMissing",
             "wfc3d.rule.missing": "TargetMissing", "wfc3d.tile.duplicate-id": "DuplicateId", "wfc3d.edge.duplicate-id": "DuplicateId",
             "wfc3d.rule.duplicate-id": "DuplicateId", "wfc3d.slot.duplicate-id": "DuplicateId", "wfc3d.tile.weight-unchanged": "NoOp",
             "wfc3d.seed.unchanged": "NoOp", "wfc3d.slot.pin-unchanged": "NoOp", "wfc3d.slot.pin-absent": "NoOp",
             "wfc3d.slot.extent-unchanged": "NoOp", "wfc3d.edge.already-connected": "NoOp", "wfc3d.tile.media-unchanged": "NoOp",
             "wfc3d.slot.position-unchanged": "NoOp", "wfc3d.slot.edges-cascaded": "Cascade", "wfc3d.tile.references-cascaded": "Cascade",
             "wfc3d.tile.non-positive-weight": "Invariant", "wfc3d.slot.degenerate-box": "Invariant", "wfc3d.edge.self-loop": "Invariant",
             "wfc3d.rule.unknown-tile": "Invariant", "wfc3d.slot.unknown-pinned-tile": "Invariant"}),
]
