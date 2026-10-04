#!/usr/bin/env python3
"""🪆️ D20 (S4-AGNOSTIC, design §12/§20.15): extends the G12 harness (`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`) to child-lane
edits. The parent session driver is split into `acceptance_drive` (begin on a document or member store → draft → accept → replay →
finalize → commit) and the parent wrapper; `assert_child_history_edits_end_to_end` + macro `composed_child_history_law!` run the member
path on an edit the plugin's seed gestures land. Anchored, count-asserted, one write, re-read immediately before; idempotent.
Usage: [--apply]."""
import sys

ROOT = "/Users/ueli/Documents/semio"
HARNESS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
MARKER = "pub async fn assert_child_history_edits_end_to_end"

OLD_SESSION_HEAD = '''/// ✏️ One session on `app` over its first mutation: begin, draft `change` (else the first change of
/// [`acceptance_changes`] the editor accepts), accept, replay to a clean review, finalize, and commit as an overwrite or as the
/// new alternative `alternative`. Answers the change drafted and the edited operation the store now folds.
async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let target = app
        .store
        .mutation_ops()
        .map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?
        .first()
        .map(|op| op.mutation_id.0.clone())
        .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, vec![("mutationId", DslValue::String(target.clone()))]).await.map_err(AcceptanceVerdict::Fail)?;
'''

NEW_SESSION_HEAD = '''/// 🚗️ One session over the mutation `target` of the member `store` (`<slot>/<childId>`, design §12; `None`: the document's own
/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts), accept, replay to a clean
/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. Answers the change drafted.
async fn acceptance_drive<A, M>(app: &mut VcsArtifactApp<A, M>, target: &str, store: Option<&str>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<(String, DslValue), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.to_string()))];
    begin.extend(store.map(|store| (HISTORY_EDIT_ARG_STORE, DslValue::String(store.to_string()))));
    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin).await.map_err(AcceptanceVerdict::Fail)?;
'''

OLD_SESSION_TAIL = '''    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, commit).await.map_err(AcceptanceVerdict::Fail)?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    let (_, supersession)'''

NEW_SESSION_TAIL = '''    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, commit).await.map_err(AcceptanceVerdict::Fail)?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    Ok(drafted)
}

/// ✏️ [`acceptance_drive`] over `app`'s first document mutation, then the store's own account of it: the superseding input decoded
/// as the edited operation, unscoped for an overwrite, scoped to the new active alternative otherwise. Answers the change drafted
/// and the edited operation the store now folds.
async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let target = app
        .store
        .mutation_ops()
        .map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?
        .first()
        .map(|op| op.mutation_id.0.clone())
        .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    let drafted = acceptance_drive(app, &target, None, change, alternative).await?;
    let (_, supersession)'''

OLD_IMPORT = "    ActionArgDef, ArgSchema, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID,\n"
NEW_IMPORT = "    ActionArgDef, ArgSchema, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID,\n    HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID,\n"

OLD_SEED_FAULT = """            Self::Operation(index, reason) => format!("seed operation #{index}: {reason}"),
        }
    }
}
"""

NEW_SEED_FAULT = """            Self::Operation(index, reason) => format!("seed operation #{index}: {reason}"),
        }
    }
}

impl AcceptanceVerdict {
    /// 🗯️ The verdict's reason or summary in words.
    fn text(self) -> String {
        match self {
            Self::Pass(text) | Self::Skip(text) | Self::SkipCase(text) | Self::SkipBase(text) | Self::Fail(text) => text,
        }
    }
}
"""

OLD_RESOLUTION = '''/// 🔗️ Every breach of complete input-schema resolution among `A`'s document leaves'''

NEW_CHILD_LAW = '''/// 🧷️ The first applied, editable, unedited child-lane mutation of `app`'s history (`HistoryMutationEntry.store` names its owned
/// member, `<slot>/<childId>`): its id, its store, and its label in every locale (refused when a locale resolves empty).
async fn acceptance_child_row<A, M>(app: &mut VcsArtifactApp<A, M>) -> Result<Option<(String, String, String)>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let patch = app.history_patch(true).await.map_err(|fault| format!("the history does not project: {fault:?}"))?;
    let Some(row) = patch.upserts.iter().filter(|entry| entry.applied).flat_map(|entry| entry.mutations.iter()).find(|row| row.store.is_some() && row.editable && !row.superseded && !row.withdrawn && !row.pending) else {
        return Ok(None);
    };
    let (en, de) = (row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string());
    if en.trim().is_empty() || de.trim().is_empty() {
        return Err(format!("the child-lane row {} is labelled {en:?} / {de:?}, not in every locale", row.mutation_id));
    }
    Ok(Some((row.mutation_id.clone(), row.store.clone().unwrap_or_default(), format!("{en} / {de}"))))
}

/// 🌰️ A registered instance on the app's initial document after the plugin's `seed` gestures (`(action id, JSON object args)`, each
/// dispatched as the acceptance actor and published to completion) — the gestures that land a child-lane edit in an owned member.
async fn acceptance_child_seeded<A, M>(manifest: fn() -> App, seed: &[(&str, &str)]) -> Result<VcsArtifactApp<A, M>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = artifact_app_laws::new_app_with_registry_and_members::<A, M>(manifest).await;
    app.bind_instance_id(artifact_app_laws::meta(ACCEPTANCE_ACTOR).instance_id).await;
    let seeded = async {
        app.store.set_local_actor_id(Some(ACCEPTANCE_ACTOR.to_string())).map_err(|error| format!("the actor is refused: {error:?}"))?;
        for (action, args) in seed {
            let args = semio_framework_pack_json::parse(args, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the seed args of {action} are not JSON: {error:?}"))?;
            let DslValue::Object(args) = semio_framework_pack_json::to_dsl_value(&args) else {
                return Err(format!("the seed args of {action} are not a JSON object"));
            };
            acceptance_verb(&mut app, action, args.iter().map(|(key, value)| (key.as_str(), value.clone())).collect()).await?;
            acceptance_pump(&mut app, |app| !app.has_pending_typed_operations()).await?;
        }
        app.refresh_cache().await.map_err(|fault| format!("the history does not backfill: {fault:?}"))
    }
    .await;
    match seeded {
        Ok(()) => Ok(app),
        Err(reason) => {
            acceptance_close(&mut app).await;
            Err(reason)
        }
    }
}

/// 🧵️ The head snapshot pack of every owned member of `app`, sorted by slot and child id.
async fn acceptance_child_heads<A, M>(app: &VcsArtifactApp<A, M>) -> Result<Vec<(String, String, Vec<u8>)>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    Ok(PluginApp::child_head_packs(app).await.map_err(|fault| format!("the owned children do not pack their heads: {fault:?}"))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.head_pack)).collect())
}

/// 🎞️ The child-lane scenario behind [`assert_child_history_edits_end_to_end`]; every instance is closed on every path.
async fn acceptance_child_scenario<A, M>(manifest: fn() -> App, seed: &[(&str, &str)]) -> Result<String, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut overwrite = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the seed does not land: {reason}"))?;
    let outcome = async {
        let (target, store, label) = acceptance_child_row(&mut overwrite).await?.ok_or("the seed lands no editable child-lane mutation in the history")?;
        let before = acceptance_child_heads(&overwrite).await?;
        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None).await {
            Ok(change) => change,
            Err(verdict) => return Err(format!("the overwrite of {target} in {store}: {}", verdict.text())),
        };
        let heads = acceptance_child_heads(&overwrite).await?;
        if heads == before {
            return Err(format!("the overwrite {} = {:?} of {target} leaves every member head unchanged", change.0, change.1));
        }
        let saved = acceptance_document_view(&mut overwrite, manifest).await?;
        let mut reloaded = acceptance_reloaded(&overwrite, manifest).await?;
        let fresh = acceptance_document_view(&mut reloaded, manifest).await;
        let fresh_heads = acceptance_child_heads(&reloaded).await;
        acceptance_close(&mut reloaded).await;
        if let Some(difference) = acceptance_view_difference(&saved, &fresh?) {
            return Err(format!("the overwritten document does not fold freshly to the same state after save and load: {difference}"));
        }
        if fresh_heads? != heads {
            return Err("a fresh fold of the edited member log has other member heads than the overwrite".into());
        }
        Ok::<_, String>((change, store, label, heads))
    }
    .await;
    acceptance_close(&mut overwrite).await;
    let (change, store, label, heads) = outcome?;
    let mut branched = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the alternative instance does not seed: {reason}"))?;
    let alternative = async {
        let (target, _, _) = acceptance_child_row(&mut branched).await?.ok_or("the alternative instance lists no editable child-lane mutation")?;
        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE)).await {
            Ok(_) => {}
            Err(verdict) => return Err(format!("the change the overwrite accepted fails as a new alternative: {}", verdict.text())),
        }
        if acceptance_child_heads(&branched).await? != heads {
            return Err("the new alternative's member heads differ from the overwrite's".into());
        }
        Ok::<(), String>(())
    }
    .await;
    acceptance_close(&mut branched).await;
    alternative?;
    Ok(format!("{store}: {} = {:?}, row \\"{label}\\"", change.0, change.1))
}

/// 👶️ LAW (design §12, §20.15 — D20): a child-lane mutation, the edit the plugin's `seed` gestures land in an owned member store, is
/// edited in history end to end through the generic member path (`historyEditBegin{mutationId, store}`): the overwrite changes the
/// member's head; the overwritten document saved and freshly loaded (its member folds the edited log from scratch) has the same head,
/// owned children, renders and member heads; the same change committed as a new alternative on a second instance reaches the same
/// member heads; the row is labelled in every locale. Answers the summary; panics naming `plugin` otherwise.
pub async fn assert_child_history_edits_end_to_end<A, M>(plugin: &str, manifest: fn() -> App, seed: &[(&str, &str)]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_child_scenario::<A, M>(manifest, seed).await {
        Ok(summary) => format!("{plugin}: child lane {summary}"),
        Err(reason) => panic!("{plugin}: the child-lane history edit breaks the generic mechanism: {reason}"),
    }
}

/// 🔗️ Every breach of complete input-schema resolution among `A`'s document leaves'''

OLD_MACRO_TAIL = '''            ::std::println!("[documents-reload] {}: {summary}", $plugin);
        }
    };
}'''

NEW_MACRO_TAIL = '''            ::std::println!("[documents-reload] {}: {summary}", $plugin);
        }
    };
}

/// 🧩️ Wires [`assert_child_history_edits_end_to_end`] into a composed plugin's test target (design §12, §20.15): `plugin` names the
/// failure, `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$seed` the `[(action id, JSON object args)]` gestures
/// that land a child-lane edit in an owned member of the editor's initial document.
#[macro_export]
macro_rules! composed_child_history_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $seed:expr) => {
        /// 🍼️ LAW (design §12, §20.15): a child-lane mutation of this editor is edited in history end to end on its member store.
        #[test]
        fn child_history_edits_end_to_end() {
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_child_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &$seed,
            ));
            ::std::println!("[child-history-edit] {summary}");
        }
    };
}'''


def main() -> None:
    apply = "--apply" in sys.argv
    with open(f"{ROOT}/{HARNESS}", encoding="utf-8") as handle:
        before = handle.read()
    if MARKER in before:
        print("SKIP: already carries the child-lane law")
        return
    after = before
    for old, new in [(OLD_IMPORT, NEW_IMPORT), (OLD_SEED_FAULT, NEW_SEED_FAULT), (OLD_SESSION_HEAD, NEW_SESSION_HEAD), (OLD_SESSION_TAIL, NEW_SESSION_TAIL), (OLD_RESOLUTION, NEW_CHILD_LAW), (OLD_MACRO_TAIL, NEW_MACRO_TAIL)]:
        count = after.count(old)
        if count != 1:
            sys.exit(f"ANCHOR: {count} matches for {old[:90]!r}")
        after = after.replace(old, new)
    if not apply:
        print(f"WOULD write {HARNESS} (+{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(f"{ROOT}/{HARNESS}", encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(f"{ROOT}/{HARNESS}", "w", encoding="utf-8") as handle:
        handle.write(after)
    print(f"WROTE {HARNESS}")


if __name__ == "__main__":
    main()
