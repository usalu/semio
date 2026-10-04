#!/usr/bin/env python3
"""🔄️ AUDIT-TOOLS F22 (S4-AGNOSTIC, staged per fleet rule 45; lands at CARGO OPEN together with `🧪️s4-agnostic-harness-leafless.py`): the child-lane
law `composed_child_history_law!` proves save → fresh load of documents that CARRY child-lane history. `acceptance_child_reload_difference`
saves a document as its recursive archive, loads it into a fresh instance through the app's own store initializer and compares head, owned
children, window renders (`acceptance_view_difference`) and member head packs. It runs (1) on the seeded document (child-lane history, before
any history edit), (2) after the overwrite finalize, (3) after the new-alternative finalize — and the alternative session itself runs on the
seeded document AFTER a save → fresh load (time travel over a reloaded document, design §20.15). Anchored on the exact current text,
count-asserted, one write, re-read immediately before; idempotent. Usage: [--apply]."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
MARKER = "async fn acceptance_child_reload_difference"

OLD_SCENARIO = """/// 🎞️ The child-lane scenario behind [`assert_child_history_edits_end_to_end`]; every instance is closed on every path.
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
"""

NEW_SCENARIO = """/// 🔄️ Where `app`'s document, saved as its recursive archive and loaded into a fresh instance through the app's own store initializer
/// ([`acceptance_reloaded`]), departs from the live one — its head, owned children or window renders ([`acceptance_view_difference`]), or its
/// member head packs (every member folds its own log, edits included, from scratch) — else `None`; the fresh instance is closed on every path.
async fn acceptance_child_reload_difference<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App) -> Result<Option<String>, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let saved = acceptance_document_view(app, manifest).await?;
    let heads = acceptance_child_heads(app).await?;
    let mut reloaded = acceptance_reloaded(app, manifest).await?;
    let fresh = acceptance_document_view(&mut reloaded, manifest).await;
    let fresh_heads = acceptance_child_heads(&reloaded).await;
    acceptance_close(&mut reloaded).await;
    if let Some(difference) = acceptance_view_difference(&saved, &fresh?) {
        return Ok(Some(difference));
    }
    Ok((fresh_heads? != heads).then(|| "the member head packs differ from the live members'".to_string()))
}

/// 🎞️ The child-lane scenario behind [`assert_child_history_edits_end_to_end`]; every instance is closed on every path.
async fn acceptance_child_scenario<A, M>(manifest: fn() -> App, seed: &[(&str, &str)]) -> Result<String, String>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut overwrite = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the seed does not land: {reason}"))?;
    let outcome = async {
        if let Some(difference) = acceptance_child_reload_difference(&mut overwrite, manifest).await? {
            return Err(format!("the seeded document with child-lane history does not reload identically: {difference}"));
        }
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
        if let Some(difference) = acceptance_child_reload_difference(&mut overwrite, manifest).await? {
            return Err(format!("the overwritten document does not fold freshly to the same state after save and load: {difference}"));
        }
        Ok::<_, String>((change, store, label, heads))
    }
    .await;
    acceptance_close(&mut overwrite).await;
    let (change, store, label, heads) = outcome?;
    let mut seeded = acceptance_child_seeded::<A, M>(manifest, seed).await.map_err(|reason| format!("the alternative instance does not seed: {reason}"))?;
    let reloaded = acceptance_reloaded(&seeded, manifest).await;
    acceptance_close(&mut seeded).await;
    let mut branched = reloaded.map_err(|reason| format!("the seeded document does not reload for the alternative: {reason}"))?;
    let alternative = async {
        let (target, _, _) = acceptance_child_row(&mut branched).await?.ok_or("the reloaded seeded document lists no editable child-lane mutation")?;
        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE)).await {
            Ok(_) => {}
            Err(verdict) => return Err(format!("the change the overwrite accepted fails as a new alternative on the reloaded document: {}", verdict.text())),
        }
        if acceptance_child_heads(&branched).await? != heads {
            return Err("the new alternative's member heads differ from the overwrite's".into());
        }
        if let Some(difference) = acceptance_child_reload_difference(&mut branched, manifest).await? {
            return Err(format!("the new alternative does not fold freshly to the same state after save and load: {difference}"));
        }
        Ok::<(), String>(())
    }
    .await;
    acceptance_close(&mut branched).await;
    alternative?;
    Ok(format!("{store}: {} = {:?}, row \\"{label}\\", seeded, overwritten and alternative documents reload identically", change.0, change.1))
}
"""

OLD_DOC = """/// 👶️ LAW (design §12, §20.15 — D20): a child-lane mutation, the edit the plugin's `seed` gestures land in an owned member store, is
/// edited in history end to end through the generic member path (`historyEditBegin{mutationId, store}`): the overwrite changes the
/// member's head; the overwritten document saved and freshly loaded (its member folds the edited log from scratch) has the same head,
/// owned children, renders and member heads; the same change committed as a new alternative on a second instance reaches the same
/// member heads; the row is labelled in every locale. Answers the summary; panics naming `plugin` otherwise.
"""

NEW_DOC = """/// 👶️ LAW (design §12, §20.15 — D20, audit F22): a child-lane mutation, the edit the plugin's `seed` gestures land in an owned member
/// store, is edited in history end to end through the generic member path (`historyEditBegin{mutationId, store}`). The seeded document
/// (carrying child-lane history) saved and freshly loaded through the app's own store initializer has the same head, owned children,
/// renders and member heads; the overwrite changes the member's head and the overwritten document reloads identically; the same change
/// committed as a new alternative on a second instance — whose session runs on the seeded document after a save → fresh load — reaches the
/// same member heads and reloads identically; the row is labelled in every locale. Answers the summary; panics naming `plugin` otherwise.
"""


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if MARKER in before:
        print("SKIP: already carries the reload-after-child-edit law")
        return
    for name, old in (("scenario", OLD_SCENARIO), ("doc", OLD_DOC)):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR {name}: {before.count(old)} matches")
    after = before.replace(OLD_SCENARIO, NEW_SCENARIO).replace(OLD_DOC, NEW_DOC)
    if "--apply" not in sys.argv:
        print(f"WOULD rewrite the child-lane scenario (+{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
