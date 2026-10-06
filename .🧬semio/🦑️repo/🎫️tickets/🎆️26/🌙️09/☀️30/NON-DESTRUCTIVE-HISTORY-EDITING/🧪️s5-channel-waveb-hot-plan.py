#!/usr/bin/env python3
"""🔥️ S5-CHANNEL wave B: the Edit-tool plan for the two hot files the driver never writes (fleet rule 17) and for the
hand-edited readers the driver cannot see (`EXTRA`: the composed-member history label that read `Edit.description`, its
fixture/schema/twin, the canonical-edit schema and twin).

Each row is `(old, new, count)`: `old` must occur exactly `count` times in the live file (count > 1 = Edit `replace_all`).
The script never writes a source file. It simulates the rows in memory and proves the result is closed under the driver:
`🧪️s4-bump-waveb.py`'s own `transform` finds nothing left to remove and no `description` token survives outside the reviewed
allowlist. `--verify` runs the same closure proof on the LIVE files (after the Edit-tool pass).
`--snapshot` copies the hand-edited files before the pass; `--restore` is the rule-51 rollback (byte-identical pre-state,
only for a file that differs from its snapshot by rows of this plan alone).
`--apply-extra` writes the `EXTRA` rows by exact anchors (seven files that are not on the rule-17 hot list); the two hot
files (`PLAN`) are edited with the Edit tool only.
Usage: python3 🧪️s5-channel-waveb-hot-plan.py [--verify | --snapshot | --restore | --apply-extra]
"""
from __future__ import annotations

import importlib.util
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TICKET = pathlib.Path(__file__).resolve().parent
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
MUTATION = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs"

PLAN: dict[str, list[tuple[str, str, int]]] = {
    MUTATION: [
        ("    pub mutation_meta: Vec<MutationMeta>,\n    pub description: Option<String>,\n", "    pub mutation_meta: Vec<MutationMeta>,\n", 1),
        ("/// fields with no explicit `#[serde(default)]` (`description`/`finished_at`) still default to", "/// fields with no explicit `#[serde(default)]` (`finished_at`) still default to", 1),
        ("        if self.description.is_some() {\n            entries.push((\"description\".to_string(), crate::value::ToValue::to_value(&self.description)));\n        }\n", "", 1),
        ("        let mut description = None;\n        let mut verb = None;", "        let mut verb = None;", 1),
        ("                \"description\" => description = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under(\"description\"))?,\n", "", 1),
        ("            mutation_meta,\n            description,\n            verb,", "            mutation_meta,\n            verb,", 1),
    ],
    PLUGIN: [
        ("description: None, transaction: None", "transaction: None", 6),
        ("ArtifactCommand::Apply { mutations, description: None, transaction }", "ArtifactCommand::Apply { mutations, transaction }", 1),
        ("description: Some(\"genesis\".to_string()), transaction: None", "transaction: None", 1),
        ("description: None, lane: HistoryLane::Interaction", "lane: HistoryLane::Interaction", 1),
        ("        mutation: Option<M>,\n        description: Option<String>,\n        authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,", "        mutation: Option<M>,\n        authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,", 1),
        ("        fn preflight(&self, mutation: &M, description: Option<&str>, lane: HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {\n            if lane != HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {\n                return Err(format!(\"{}-lane-or-description-envelope\", self.prefix));",
         "        fn preflight(&self, mutation: &M, lane: HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {\n            if lane != HistoryLane::Document {\n                return Err(format!(\"{}-lane\", self.prefix));", 1),
        ("                mutation: Some(request.mutation),\n                description: request.description,\n", "                mutation: Some(request.mutation),\n", 1),
        ("            if self.description.take().is_some() {\n                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });\n            }\n", "", 1),
        ("self.mutation.is_none() && self.description.is_none() && self.authority.is_none()", "self.mutation.is_none() && self.authority.is_none()", 1),
        ("&child_emits, None, effects, events, ui_scope, meta, None, transaction)", "&child_emits, effects, events, ui_scope, meta, None, transaction)", 1),
        ("children: Vec<ChildEmit>, description: Option<String>, origin: protocol::MutationOrigin", "children: Vec<ChildEmit>, origin: protocol::MutationOrigin", 1),
        ("&ops, &children, description, Vec::new(), Vec::new(), UiDirtyScope::Full", "&ops, &children, Vec::new(), Vec::new(), UiDirtyScope::Full", 1),
        ("self.commit_transaction_group(&txn_id, ops, children, None, origin, meta)", "self.commit_transaction_group(&txn_id, ops, children, origin, meta)", 1),
        ("            child_emits: &[ChildEmit],\n            description: Option<String>,\n            effects: Vec<Effect>,", "            child_emits: &[ChildEmit],\n            effects: Vec<Effect>,", 1),
        ("GroupMeta { actor: Some(meta.actor.clone()), description, group_id, transaction }", "GroupMeta { actor: Some(meta.actor.clone()), group_id, transaction }", 1),
        ("&pending.child_emits, None, Vec::new(), Vec::new(), UiDirtyScope::None", "&pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None", 1),
        ("mutations, None, self.artifact_one_item_factory.as_ref(), transaction)", "mutations, self.artifact_one_item_factory.as_ref(), transaction)", 1),
        ("                                mutations,\n                                None,\n                                HistoryLane::Document,\n                                self.artifact_one_item_factory.as_ref(),", "                                mutations,\n                                HistoryLane::Document,\n                                self.artifact_one_item_factory.as_ref(),", 1),
        ("                            std::mem::take(&mut emit.config_mutations),\n                            None,\n                            HistoryLane::Document,", "                            std::mem::take(&mut emit.config_mutations),\n                            HistoryLane::Document,", 1),
        ("                            std::mem::take(&mut emit.draft_mutations),\n                            None,\n                            HistoryLane::Document,", "                            std::mem::take(&mut emit.draft_mutations),\n                            HistoryLane::Document,", 1),
        ("let (reason, mutations, _) = rejected.into_owners();", "let (reason, mutations) = rejected.into_owners();", 3),
        ("        /// 🎛️ `meta.actor`/`description` are honored; a group is always one plain edit per member.", "        /// 🎛️ `meta.actor` is honored; a group is always one plain edit per member.", 1),
        ("    /// operations is named by its id. An edit's stored description is never a label (design §20.6).", "    /// operations is named by its id. An edit stores no description (design §20.6).", 1),
    ],
}
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
TIME_TRAVEL = f"{OSM}/🔌️plugin/⏪️time-travel/🦀️.rs"
CHILD_HISTORY = f"{OSM}/🔌️plugin/🧫️fixtures/🧫️composed-child-history"
CANONICAL = f"{OSM}/🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json"
EXTRA: dict[str, list[tuple[str, str, int]]] = {
    TIME_TRAVEL: [
        ("    pub edit_id: String,\n    pub description: Option<String>,\n    pub started_at: String,", "    pub edit_id: String,\n    pub started_at: String,", 1),
        ("                edit_id: edit.id.clone(),\n                description: edit.description.clone(),\n", "                edit_id: edit.id.clone(),\n", 1),
        ("        let label = match (history.description.clone(), history.mutations.first()) {\n            (Some(description), _) => LocalizedLabel::data(description),\n            (None, Some(first)) => history_leaf_row_label(&first.label, history.op_count),\n            (None, None) => LocalizedLabel::data(history.op_lines.first().cloned().unwrap_or_else(|| history.edit_id.clone())),\n        };",
         "        let label = history.mutations.first().map_or_else(|| LocalizedLabel::data(history.op_lines.first().cloned().unwrap_or_else(|| history.edit_id.clone())), |first| history_leaf_row_label(&first.label, history.op_count));", 1),
    ],
    f"{OSM}/🔌️plugin/🧪️tests/🧪️composed-child-history/🦀️.rs": [
        ("        description: value[\"description\"].as_str().map(str::to_string),\n", "", 1),
        ("/// German from the first leaf, the description, or the first printed operation.", "/// German from the first leaf or the first printed operation.", 1),
    ],
    f"{OSM}/🔌️plugin/🧪️tests/🧪️composed-child-history/🟦️.ts": [
        ("at: number; description: string | null; startedAt: string;", "at: number; startedAt: string;", 1),
        ("/** 🏷️ A row's label: the description verbatim, else its first leaf with", "/** 🏷️ A row's label: its first leaf with", 1),
        ("  if (history.description !== null) return { en: history.description, de: history.description };\n", "", 1),
    ],
    f"{CHILD_HISTORY}/🔣️.json": [
        ("\"description\": null, ", "", 7),
        ("\"description\": \"Imported\", ", "", 1),
        ("\"editIds\": [\"e1\"], \"label\": { \"en\": \"Imported\", \"de\": \"Imported\" }", "\"editIds\": [\"e1\"], \"label\": { \"en\": \"Set snapshot\", \"de\": \"Schnappschuss setzen\" }", 1),
    ],
    f"{CHILD_HISTORY}/🧬️schema/🔣️.json": [
        ("moment (physical milliseconds), description, operation count", "moment (physical milliseconds), operation count", 1),
        ("\"required\": [\"store\", \"editId\", \"transaction\", \"at\", \"description\", \"startedAt\", \"opCount\", \"opLines\"]", "\"required\": [\"store\", \"editId\", \"transaction\", \"at\", \"startedAt\", \"opCount\", \"opLines\"]", 1),
        ("        \"description\": { \"type\": [\"string\", \"null\"] },\n", "", 1),
    ],
    CANONICAL: [
        ("        \"mutationMeta\",\n        \"description\",\n        \"sequenceNumber\",", "        \"mutationMeta\",\n        \"sequenceNumber\",", 2),
        ("        \"description\": {\n          \"type\": \"string\"\n        },\n        \"sequenceNumber\": {", "        \"sequenceNumber\": {", 2),
    ],
    f"{OSM}/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts": [
        ("      ...text(edit, \"actor\"),\n      ...text(edit, \"description\"),\n", "      ...text(edit, \"actor\"),\n", 1),
    ],
}
ALLOWED = re.compile(r"action_describe|tree_item|\.description\(|semantics\.description|entry\.get\(\"description\"\)|history_mutation_description|let mut description = if entry\.mutations|description = if description\.is_empty|if !description\.is_empty|if let Some\(description\) = description|UiText::clipped\(&description\)|fn field<|self\.field\(id, label, description, control\)|let description = (?:description\.into|entry\.get|time_travel::)")


def driver():
    spec = importlib.util.spec_from_file_location("waveb", TICKET / "🧪️s4-bump-waveb.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def closure(waveb, path: str, text: str) -> list[str]:
    problems = []
    result, log = waveb.transform(path, text)
    if result != text:
        problems += [f"driver still removes at {line}: {what}" for line, what in log]
    mask = waveb.fast_code_mask(text)
    for match in re.finditer(r"\bdescription\b", text):
        if not mask[match.start()]:
            continue
        start, end = text.rfind("\n", 0, match.start()) + 1, text.find("\n", match.start())
        line = text[start:end]
        if not ALLOWED.search(line):
            problems.append(f"description survives at {text.count(chr(10), 0, match.start()) + 1}: {line.strip()[:140]}")
    return problems


SNAPSHOT = TICKET / "🗑️generated" / "s5-channel" / "waveb-backup" / "hand"


def settled(rows: list[tuple[str, str, int]], text: str) -> str:
    for old, new, _ in rows:
        text = text.replace(old, new)
    return text


def snapshot() -> None:
    """📸️ Copies every hand-edited file before the Edit pass (the rule-51 "pre-acquire state")."""
    for path in {**PLAN, **EXTRA}:
        target = SNAPSHOT / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text((ROOT / path).read_text())
    print(f"[DEBUG] snapshot of {len(PLAN) + len(EXTRA)} hand-edited files under {SNAPSHOT}")


def restore() -> None:
    """♻️ Rollback of the Edit pass: a file goes back to its snapshot only when it differs from it by rows of this plan alone
    (any subset applied); a file someone else edited meanwhile is left untouched and listed."""
    conflicts = []
    for path, rows in {**PLAN, **EXTRA}.items():
        copy = SNAPSHOT / path
        if not copy.is_file():
            raise SystemExit(f"[DEBUG] no snapshot of {path}: refusing")
    for path, rows in {**PLAN, **EXTRA}.items():
        before, current = (SNAPSHOT / path).read_text(), (ROOT / path).read_text()
        if current == before:
            continue
        if settled(rows, current) != settled(rows, before):
            conflicts.append(path)
            continue
        (ROOT / path).write_text(before)
        print(f"   restored {path}")
    for path in conflicts:
        print(f"   CONFLICT {path}")
    if conflicts:
        raise SystemExit(1)


def apply_extra() -> None:
    """✍️ Writes the `EXTRA` rows (the files outside the rule-17 hot list): every anchor exactly its count, else nothing."""
    results: dict[str, str] = {}
    for path, rows in EXTRA.items():
        text = (ROOT / path).read_text()
        for old, new, count in rows:
            if text.count(old) != count:
                raise SystemExit(f"[DEBUG] {path}: anchor found {text.count(old)}x, expected {count}x, nothing written: {old[:100]!r}")
            text = text.replace(old, new)
        results[path] = text
    for path, text in results.items():
        (ROOT / path).write_text(text)
    print(f"[DEBUG] {len(results)} hand-edit files outside the hot list written")


def main() -> None:
    if "--snapshot" in sys.argv:
        return snapshot()
    if "--restore" in sys.argv:
        return restore()
    if "--apply-extra" in sys.argv:
        return apply_extra()
    verify = "--verify" in sys.argv
    waveb = driver()
    failed = False
    for path in (MUTATION, PLUGIN):
        if not (ROOT / path).is_file():
            raise SystemExit(f"[DEBUG] hot file missing: {path}")
    for path in (MUTATION, PLUGIN):
        source = (ROOT / path).read_text()
        if "request.description" in source:
            waveb.PREPARED_ALL.update(waveb.preparation_types(source) - {"Self"})
    for path, rows in PLAN.items():
        text = (ROOT / path).read_text()
        print(f"== {path}")
        if not verify:
            for index, (old, new, count) in enumerate(rows, 1):
                found = text.count(old)
                status = "ok" if found == count else "MISMATCH"
                failed |= found != count
                where = text.count("\n", 0, text.find(old)) + 1 if found else 0
                print(f"   {index:2} {status} x{found}/{count} L{where}{' replace_all' if count > 1 else ''}: {old.splitlines()[0].strip()[:110]}")
                text = text.replace(old, new)
        for problem in closure(waveb, path, text):
            failed = True
            print(f"   OPEN {problem}")
    for path, rows in EXTRA.items():
        if not (ROOT / path).is_file():
            raise SystemExit(f"[DEBUG] hand-edit file missing: {path}")
        text = (ROOT / path).read_text()
        print(f"== {path}")
        for index, (old, new, count) in enumerate(rows, 1):
            found = text.count(old)
            wanted = 0 if verify else count
            failed |= found != wanted
            print(f"   {index:2} {'ok' if found == wanted else ('OPEN' if verify else 'MISMATCH')} x{found}/{wanted}{' replace_all' if count > 1 else ''}: {old.splitlines()[0].strip()[:110]}")
    print(f"[DEBUG] hot plan {'verify' if verify else 'simulation'}: {'NOT closed' if failed else 'closed (driver finds nothing, no stray description)'}")
    if failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
