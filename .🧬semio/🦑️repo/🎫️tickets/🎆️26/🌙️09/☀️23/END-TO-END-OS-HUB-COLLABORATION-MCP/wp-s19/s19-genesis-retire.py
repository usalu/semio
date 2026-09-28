#!/usr/bin/env python3
"""🌱️ S19 one-off codemod (set `gen-archive-load`, window-3 addition): hub creation of `2d.generation` / `3d.generation`.

Measured on hub 7800 (2026-09-28 21:1x, W4's open-plan probe, `.🧬semio/🌐hub/s13-w3-state-7800/capture.txt` l.178/359):
`genesis materialization failed: trusted artifact codec Input failed: guest trapped … ordered-map root must be explicitly
retired before drop`. The guest half of the hub's genesis is the SDK's ONE producer `artifact_app_genesis_pair` (the
`codec.genesis` export of every plugin): it prints the pair from `create_document_envelope(…).into_owners()` and then
plain-drops those owners — and with them `vcs.initial_snapshot`, whose procedural `host_snapshot.layout` is an
`OrderedMap` that aborts on a bare drop. The same plain drop sits in the kernel's `ArtifactEnvelope::retire_unadopted`
(used by `codec.print-mirror` and every refused candidate), which also popped `Edit`s whose procedural operations own
fail-closed roots. Fix: `retire_unadopted` retires the initial snapshot through the technology's own
`MutationDiff::retire_projection` and every popped edit through `retire_scratch_edits` (tail-first, as before); the genesis
producer hands its owners back to `retire_unadopted` on both its paths. One law per procedural artifact drives the exact
export. Idempotent. usage: s19-genesis-retire.py <root>"""
import os
import sys

root = sys.argv[1]
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
ARTIFACTS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts"


def edit(rel, change):
    path = os.path.join(root, rel)
    before = open(path, encoding="utf-8").read()
    after = change(before)
    if after != before:
        open(path, "w", encoding="utf-8").write(after)
        print(f"edited {rel}")


def once(old, new, marker):
    def change(text):
        if marker in text:
            return text
        assert text.count(old) == 1, f"anchor x{text.count(old)}: {old[:100]!r}"
        return text.replace(old, new)
    return change


edit(STORE, once(
    '''    /// same tail-first order {@link ArtifactStore::close_take_final_envelope_retirement} leaves the
    /// ledgers in, and nothing else about the refusal changes.
    pub fn retire_unadopted(self) {
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, mut edit_messages, conflicts, transitions } = self.into_owners();
        let ArtifactVcs { initial_snapshot, mut edits, mut changes, mut checkpoints, mut alternatives } = vcs;
        while edits.pop().is_some() {}
        while changes.pop().is_some() {}
        while checkpoints.pop().is_some() {}
        while alternatives.pop().is_some() {}
        while edit_messages.pop().is_some() {}
        drop((schema, id, initial_snapshot, edits, changes, checkpoints, alternatives, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, edit_messages, conflicts, transitions));
    }''',
    '''    /// same tail-first order {@link ArtifactStore::close_take_final_envelope_retirement} leaves the
    /// ledgers in, and nothing else about the refusal changes. The initial snapshot and every popped
    /// edit are retired through the technology's own vocabulary, never dropped: a procedural
    /// snapshot's `OrderedMap` layout and its operations' roots abort the guest on a bare drop (hub
    /// genesis of `3d.generation`, 2026-09-28, ticket 26/09/23 slice S19).
    pub fn retire_unadopted(self)
    where
        Mutation: self::Mutation<P>,
    {
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, mut edit_messages, conflicts, transitions } = self.into_owners();
        let ArtifactVcs { initial_snapshot, mut edits, mut changes, mut checkpoints, mut alternatives } = vcs;
        while let Some(edit) = edits.pop() {
            retire_scratch_edits::<P, Mutation>([edit]);
        }
        while changes.pop().is_some() {}
        while checkpoints.pop().is_some() {}
        while alternatives.pop().is_some() {}
        while edit_messages.pop().is_some() {}
        drop((schema, id, edits, changes, checkpoints, alternatives, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, edit_messages, conflicts, transitions));
        retire_replayed_projection::<P, Mutation>(initial_snapshot);
    }''',
    "retire_replayed_projection::<P, Mutation>(initial_snapshot);",
))

edit(PLUGIN, once(
    '''            || !zero_cursor
        {
            return Err(store::VcsError::ValidationFailed("artifact genesis produced nonzero history".into()));
        }
        store::print_document_pack(&envelope).await
    }''',
    '''            || !zero_cursor
        {
            store::ArtifactEnvelope::from_owners(envelope).retire_unadopted();
            return Err(store::VcsError::ValidationFailed("artifact genesis produced nonzero history".into()));
        }
        let printed = store::print_document_pack(&envelope).await;
        store::ArtifactEnvelope::from_owners(envelope).retire_unadopted();
        printed
    }''',
    """            store::ArtifactEnvelope::from_owners(envelope).retire_unadopted();
            return Err(store::VcsError::ValidationFailed("artifact genesis produced nonzero history".into()));""",
))

VARIANTS = [("🧊️generation3d", "Generation3d", "generation3d"), ("🌀️generation2d", "Generation2d", "generation2d")]
for folder, T, v in VARIANTS:
    law_file = f"{ARTIFACTS}/{folder}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
    LAW = f'''
//#region 🌱️HubGenesis
/// 🌱️ The hub creates a `{v}` document through this plugin's `codec.genesis` export — the SDK's one producer
/// `artifact_app_genesis_pair` — and the pair it prints must parse back to a live document. Measured on hub 7800
/// (2026-09-28, W4 open-plan probe): creation failed `ordered-map root must be explicitly retired before drop`, the
/// producer plain-dropped the initial snapshot whose `host_snapshot.layout` is an `OrderedMap` (ticket 26/09/23 slice S19).
#[semio_framework_async_macros::async_test]
async fn a_hub_genesis_pair_is_produced_and_parses_back_without_trapping() {{
    let pair = semio_framework_plugin::artifact_app_genesis_pair::<semio_framework_plugin::EditorApp<{T}PlayApp>>("artifact-0123456789abcdef0123456789abcdef").await.expect("the genesis export produces a pair");
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "a genesis pair carries both a pack and an SPR");
    let snapshot = semio_framework_plugin::artifact_pair_snapshot::<crate::standards::v1::subsets::any::schema::snapshot::{T}Snapshot, crate::standards::v1::subsets::any::schema::mutations::{T}Mutation>(&pair.pack, &pair.spr)
        .await
        .expect("the genesis pair parses back");
    let initial = <{T}PlayApp as semio_framework_plugin::ArtifactEditor>::initial_snapshot();
    let expected: Vec<String> = initial.host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    let loaded: Vec<String> = snapshot.host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert_eq!(loaded, expected, "the genesis document is exactly the editor's initial document");
    initial.retire_cold();
    snapshot.retire_cold();
}}
//#endregion 🌱️HubGenesis
'''

    def law_change(text, LAW=LAW):
        if "fn a_hub_genesis_pair_is_produced_and_parses_back_without_trapping" in text:
            return text
        return text.rstrip("\n") + "\n" + LAW

    edit(law_file, law_change)
