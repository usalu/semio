# Engagements Retained Publication

## Result

WGPU now reads `WindowEngagement` through the plugin producer's canonical reserved
`framework.section.engagements` retained surface on native and browser targets. The dead native
empty-map exchange and optional browser `windowEngagements` lookup are removed. Both targets use
`ProgramBridgeEntry::render_with_document`, matching the existing Measures section transport.

Shell decodes the retained document through `window_engagements_from_section`, retires the source
lease before publishing per-window Actions/Search documents, and reports producer or schema faults
through the surface fault ledger. A failed refresh preserves the last valid snapshot. The Search
toggle follows React's input-or-possibles predicate, while the Search body still requires an input.

## Contract coverage

The language-neutral fixture `🧫️fixtures/🎬️window-engagements/🔣️.json` carries:

- an input-only window: Search toggle and body;
- a possibles-only window: Search toggle with no blank body;
- an absent window: neither toggle nor body.

The ProgramBridge laws build the fixture with the real plugin `paged_text_carrier`, publish a
`UiDocumentLease`, decode the actual camel-case schema, and compare the round trip with
`serde_json::Value`. A malformed non-map payload must refuse.

The Shell laws install that production document, exercise `window_has_search_spec`,
`build_window_search_ui`, and `refresh_window_action_panes`, then close the input owner and verify its
Search document reaches terminal retirement. A malformed replacement retires its transport owner,
records one explicit fault, and preserves the valid snapshot.

## Verification

Passed:

```text
bun -e '<JSON fixture census>'
engagement fixture oracle: 2 section entries / 3 presence cases
```

```text
rustfmt --edition 2021 --emit stdout <ProgramBridge production> >/dev/null
rustfmt --edition 2021 --emit stdout <Shell production> >/dev/null
rustfmt --edition 2021 --emit stdout <ProgramBridge engagement law> >/dev/null
rustfmt --edition 2021 --emit stdout <Shell Actions/Search law> >/dev/null
```

All four read-only syntax passes exited `0`. `git diff --check` passed for the bounded packet.

The new native laws are mounted but were not run in this packet because root owns the active Cargo,
native, and WASM build lanes:

- `window_engagements_section_round_trips_the_actual_camel_case_schema`
- `window_engagements_section_refuses_a_non_map_payload`
- `canonical_engagements_publication_drives_search_presence_body_and_retirement`
- `malformed_engagements_retire_the_transport_lease_and_preserve_the_last_valid_snapshot`
