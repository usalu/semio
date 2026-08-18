# 📓️ Lease request — P6-actions-policy needs `🌉️mcp/🗂️catalog/🦀️component.rs` fixed (not by me)

**Status: pending, cross-referencing P8's own flag — this is the SAME defect, now confirmed blocking
a second packet.**

## What's broken

`🌉️mcp/🗂️catalog/🦀️component.rs` (P2-catalog, closed) does not compile against the current
`🛂️manifest/🦀️component.rs`:

```
error[E0308]: mismatched types
   --> 🌉️mcp/🗂️catalog/🦀️component.rs:615:95
615 |    insert_capability(&mut entries, capability_from_contribution(&plugin_id, "infer", entry, CapabilityKind::Query))?;
    |                                    expected `&DescriptorEntry`, found `&ContributedInferenceMetadata`
(+ lines 618 `mutate`/`&ContributedMutationMetadata`, 621 `io`/`&IoEntryDescriptor`,
  624 `compose`/`&ComposerEntryDescriptor` — same function, 4 mismatches total)
```

Root cause (confirmed independently, matching `📓️terra-P8-report.md` §4 exactly):
`manifest::ContributionSet.inference_services/mutation_services/io_entries/composer_entries` are now
the real typed `Vec<ContributedInferenceMetadata>`/`Vec<ContributedMutationMetadata>`/
`Vec<IoEntryDescriptor>`/`Vec<ComposerEntryDescriptor>` (landed by the peer ticket's E1/E2 work),
replacing the old untyped `Vec<DescriptorEntry>` placeholder `capability_from_contribution`
(`🗂️catalog/🦀️component.rs:364`) still expects for all four.

## Why this blocks P6, not just P8

`🌉️mcp/🗂️catalog` is a compile-time dependency of every other facet in this crate, including
`🎬️actions`/`🛡️policy` (this packet). `cargo test -p semio-framework-os-mcp` cannot produce ANY
result — not even for code that never touches contributions — until this compiles. This packet's own
`🎬️actions`/`🛡️policy`/root-`🦀️component.rs` code has been verified by direct reading and by every
`cargo check` pass getting past MY facets with zero errors reported against them (only warnings from
`🏠️workspace`, P7's file, and these 4 pre-existing errors in `🗂️catalog`) — see
`📓️terra-P6-report.md` for the exact transcripts.

## Not fixed by me

`🌉️mcp/🗂️catalog/**` is not in P6's `path_scope` either (§2 of the brief explicitly lists it under
"Do NOT edit"). I did not attempt a temporary local patch-and-revert to unblock my own verification —
`📌️important.md` rule 1 (auto-commit bot + concurrent live sessions) makes even a temporary edit to a
file this contended a real risk, not a shortcut.

## Proposed fix shape (for whoever picks this up — P2 follow-up or sol)

`capability_from_contribution`'s single generic-`DescriptorEntry` signature no longer fits: none of
the four new typed rows carries a bare `.id`. A real fix needs ~4 small, kind-specific projections
(not one generic function), each deriving a sensible capability id/title from the new type's actual
fields — e.g. `ContributedMutationMetadata.mutation_id` directly for `mutate`;
`ContributedInferenceMetadata` has no id-like field at all (only `owner`/`artifact_kind`/
`document_schema`/`contributor` etc.) so its capability id needs synthesizing from those;
`IoEntryDescriptor`/`ComposerEntryDescriptor` are structurally identified (`owner`/`counterpart` or
`writes`/`reads` dialect pairs), not by a per-entry id, so an index or a dialect-pair-derived slug is
needed. This is a real (if bounded) design decision inside P2's own domain, which is why I did not
apply anything myself — P8's `spawn_task` (`task_b39ce04b`, "Fix broken semio-framework-os-mcp
catalog compiler") already carries the full repro; this file adds P6's own confirmation that it
also blocks the actions/policy packet (and P7's own `📓️lease-P7-catalog-contribution-types.md`,
filed within a minute of this one, makes THREE independent packets now blocked on it) — so sol can
raise its priority. Read the four new types' own field lists
(`🛂️manifest/🦀️component.rs:3731,3746,4497,4510`) before applying anything below; this is advisory,
not verified against the current tree by anyone but me.

### A concrete starting point (advisory only — not applied, not tested)

Replace the single generic helper with one small per-kind id/title derivation, called with an index
(`.enumerate()`) at each of the 4 call sites since three of the four new types have no per-entry
`.id` at all:

```rust
fn capability_from_descriptor_row(plugin_id: &str, category: &str, local_id: &str, title: String, description: String) -> CapabilityDefinition {
    let id = format!("{plugin_id}.{category}.{local_id}");
    CapabilityDefinition {
        id: CapabilityRef(id.clone()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: plugin_id.to_string(), app_id: None, window_kind_id: None, mode_id: None },
        kind: CapabilityKind::Query, // caller overrides via a `kind` param exactly like today
        title,
        description,
        artifact_kind: None,
        use_when: Vec::new(),
        input_schema: serde_json::json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "$id": format!("semio://capability/{id}/input"), "type": "object" }),
        output_schema: generic_output_schema(&id),
        effects: manifest::CapabilityEffects::default(),
        policy: manifest::CapabilityPolicy::default(),
        execution: manifest::CapabilityExecution { class: manifest::ExecutionClass::Job, ..Default::default() },
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: Some(category.to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Descriptor { category: category.to_string(), id: local_id.to_string() },
    }
}
```

Call-site sketch (`compile()`, replacing lines 614-625):

```rust
for entry in &descriptor.contributions.inference_services {
    // no per-entry id in ContributedInferenceMetadata — derive one from what IS unique per row
    let local_id = format!("{}-{}", entry.document_schema, entry.inference_schema);
    insert_capability(&mut entries, capability_from_descriptor_row(&plugin_id, "infer", &local_id, humanize(&local_id), format!("infer contribution from {plugin_id}")))?;
}
for entry in &descriptor.contributions.mutation_services {
    // mutation_id IS already the natural unique id
    insert_capability(&mut entries, capability_from_descriptor_row(&plugin_id, "mutate", &entry.mutation_id, humanize(&entry.mutation_id), format!("mutate contribution from {plugin_id}")))?;
}
for (index, entry) in descriptor.contributions.io_entries.iter().enumerate() {
    let local_id = format!("{}-{}-{index}", entry.owner, entry.counterpart); // needs ArtifactDialect: Display or a .to_string() equivalent — check what's already in scope
    insert_capability(&mut entries, capability_from_descriptor_row(&plugin_id, "io", &local_id, humanize(&local_id), format!("io contribution from {plugin_id}")))?;
}
for (index, entry) in descriptor.contributions.composer_entries.iter().enumerate() {
    let local_id = format!("{}-{index}", entry.writes);
    insert_capability(&mut entries, capability_from_descriptor_row(&plugin_id, "compose", &local_id, humanize(&local_id), format!("compose contribution from {plugin_id}")))?;
}
```

`kind` per category (`Query` for infer, `Job` for mutate/io/compose) needs threading back in — the
sketch above dropped it for brevity; wire it exactly like the original call sites did
(`CapabilityKind::Query`/`CapabilityKind::Job` passed in, not hardcoded in the helper). Whoever
applies this: verify `ArtifactDialect` actually has a cheap string form before using it in a
`format!` — if not, substitute whatever field IS a plain string on that type.
