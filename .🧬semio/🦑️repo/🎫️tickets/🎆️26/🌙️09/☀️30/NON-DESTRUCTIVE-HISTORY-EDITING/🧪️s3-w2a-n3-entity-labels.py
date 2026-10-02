"""🧪️ S3-W2A gap N3: a generic, framework-owned default for reference chips (`🔌️plugin/⏪️time-travel/🦀️.rs`), applied in
one write: the app's `ArtifactApp::entity_label` first, then the referenced entity's own name in the previewed document,
then "<Kind> <short id>" with the kind word from the framework input-label glossary in every locale."""

import pathlib
import sys

FILE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs")

EDITS = [
    (
        """/// 🧹️ Discarded draft operations cold-retired per driver or close turn.
const TIME_TRAVEL_DISCARD_OPS_PER_TURN: usize = 64;""",
        """/// 🧹️ Discarded draft operations cold-retired per driver or close turn.
const TIME_TRAVEL_DISCARD_OPS_PER_TURN: usize = 64;
/// 🔎️ Document values one reference-name lookup visits at most ([`time_travel_entity_names`]).
pub const TIME_TRAVEL_ENTITY_NAME_VISITS: usize = 262_144;
/// ✂️ Characters of an entity id a fallback chip shows before an ellipsis ([`time_travel_reference_fallback_label`]).
pub const TIME_TRAVEL_SHORT_ID_CHARS: usize = 12;""",
    ),
    (
        """/// 🧾️ The JSON text of `value`.
fn time_travel_json(value: &DslValue) -> String {
    dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(value))
}
//#endregion 🔖️Pointer""",
        """/// 🧾️ The JSON text of `value`.
fn time_travel_json(value: &DslValue) -> String {
    dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(value))
}

/// 🔎️ The names the document value gives the entities `ids` — the generic default behind reference chips (gap N3): the
/// first object at any depth whose `id` is one of `ids` is named by its first `label`, `name`, `title` or `text` that is a
/// non-empty string (data in every locale) or an `{en, de}` pair. The walk visits at most
/// [`TIME_TRAVEL_ENTITY_NAME_VISITS`] values and stops once every id is named.
pub fn time_travel_entity_names(document: &DslValue, ids: &BTreeSet<&str>) -> BTreeMap<String, LocalizedLabel> {
    let text = |value: &DslValue| value.as_str().map(str::trim).filter(|text| !text.is_empty()).map(str::to_string);
    let name = |value: &DslValue| match value {
        DslValue::Object(fields) => {
            let locale = |key: &str| fields.iter().find(|(field, _)| field == key).and_then(|(_, value)| text(value));
            Some(LocalizedLabel::native(&locale("en")?, &locale("de")?))
        }
        other => text(other).map(LocalizedLabel::data),
    };
    let mut names = BTreeMap::new();
    let mut stack = vec![document];
    let mut visits = 0;
    while let Some(value) = stack.pop() {
        visits += 1;
        if visits > TIME_TRAVEL_ENTITY_NAME_VISITS || names.len() == ids.len() {
            break;
        }
        match value {
            DslValue::Object(fields) => {
                let id = fields.iter().find(|(key, _)| key == "id").and_then(|(_, id)| id.as_str()).filter(|id| ids.contains(id) && !names.contains_key(*id));
                if let Some((id, label)) = id.and_then(|id| Some((id, ["label", "name", "title", "text"].iter().find_map(|key| fields.iter().find(|(field, _)| field == key).and_then(|(_, value)| name(value)))?))) {
                    names.insert(id.to_string(), label);
                }
                stack.extend(fields.iter().rev().map(|(_, value)| value));
            }
            DslValue::Array(items) => stack.extend(items.iter().rev()),
            _ => {}
        }
    }
    names
}

/// 🏷️ The chip of an entity `id` that neither the app nor the document names (gap N3): the reference's first declared kind
/// as a word — its last segment through the framework input-label glossary in every locale, else that segment spelled out —
/// then a short id (the id before any `!` qualifier, cut to [`TIME_TRAVEL_SHORT_ID_CHARS`] characters). Without a declared
/// kind the short id alone.
pub fn time_travel_reference_fallback_label(kinds: &[String], id: &str) -> LocalizedLabel {
    let bare = id.split('!').next().unwrap_or(id);
    let short = if bare.chars().count() > TIME_TRAVEL_SHORT_ID_CHARS { format!("{}\\u{2026}", bare.chars().take(TIME_TRAVEL_SHORT_ID_CHARS).collect::<String>()) } else { bare.to_string() };
    let Some(segment) = kinds.first().and_then(|kind| kind.split('@').next()).and_then(|kind| kind.rsplit(['.', ':', '/', '#']).next()).filter(|segment| !segment.is_empty()) else {
        return LocalizedLabel::data(short);
    };
    let word = semio_framework::input_label_glossary().get(segment).cloned().unwrap_or_else(|| LocalizedLabel::data(time_travel_spelled_out(segment)));
    LocalizedLabel::from_fn(move |terminology, locale| format!("{} {short}", word.resolve(terminology, locale)))
}

/// 🔤️ An identifier segment as words with a leading capital: `targetRegion`, `target-region` and `target_region` read
/// "Target region".
fn time_travel_spelled_out(segment: &str) -> String {
    let mut words = String::new();
    for (index, character) in segment.chars().enumerate() {
        match character {
            '-' | '_' => words.push(' '),
            upper if upper.is_uppercase() && index > 0 => words.extend([' '].into_iter().chain(upper.to_lowercase())),
            first if index == 0 => words.extend(first.to_uppercase()),
            other => words.push(other),
        }
    }
    words
}
//#endregion 🔖️Pointer""",
    ),
]

OLD_RESOLVER_HEAD = """    pub(crate) fn resolve_time_travel_reference_labels(&self, panel: &mut TimeTravelPanel) {
        let Some(editor) = panel.editor.as_mut().filter(|_| self.time_travel.member.is_none()) else { return };
        let committed = self.store.snapshot_owner();
        let shown = self.time_travel.render_snapshot_or(&self.tool_runs, &committed);
        for row in &editor.rows {
            let ArgSchema::Reference { kinds, .. } = &row.input.schema else { continue };
            let ids: Vec<String> = match &row.value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for id in ids {
                if let Some(label) = A::entity_label(shown.as_ref(), kinds, &id) {
                    editor.reference_labels.insert(id, label);
                }
            }
        }
    }"""

NEW_RESOLVER_HEAD = """    pub(crate) fn resolve_time_travel_reference_labels(&self, panel: &mut TimeTravelPanel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        let member = self.time_travel.member.is_some();
        let committed = self.store.snapshot_owner();
        let shown = self.time_travel.render_snapshot_or(&self.tool_runs, &committed);
        let mut unnamed: Vec<(String, Vec<String>)> = Vec::new();
        for row in &editor.rows {
            let ArgSchema::Reference { kinds, .. } = &row.input.schema else { continue };
            let ids: Vec<String> = match &row.value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for id in ids {
                match (!member).then(|| A::entity_label(shown.as_ref(), kinds, &id)).flatten() {
                    Some(label) => {
                        editor.reference_labels.insert(id, label);
                    }
                    None => unnamed.push((id, kinds.clone())),
                }
            }
        }
        if unnamed.is_empty() {
            return;
        }
        let document = match member {
            true => match self.time_travel_query(TimeTravelStoreQuery::ShownValue(self.time_travel.session.stage)) {
                Some(TimeTravelQueryOutput::Value(Some(value))) => value,
                _ => DslValue::Null,
            },
            false => protocol::ToValue::to_value(shown.as_ref()),
        };
        let names = time_travel_entity_names(&document, &unnamed.iter().map(|(id, _)| id.as_str()).collect());
        for (id, kinds) in unnamed {
            let label = names.get(&id).cloned().unwrap_or_else(|| time_travel_reference_fallback_label(&kinds, &id));
            editor.reference_labels.insert(id, label);
        }
    }"""

OLD_DOC = """    /// 🏷️ Labels every id the editor's reference rows name with the app's entity label in the document the session
    /// previews (`ArtifactApp::entity_label` over the reference's kinds); a session on a composed member keeps the ids,
    /// since the app reads only its own document."""

NEW_DOC = """    /// 🏷️ Labels every id the editor's reference rows name, in the document the session previews: the app's entity label
    /// (`ArtifactApp::entity_label` over the reference's kinds — the hook a plugin refines chips by), else the entity's own
    /// name in that document ([`time_travel_entity_names`]), else its kind word and a short id
    /// ([`time_travel_reference_fallback_label`], gap N3). A session on a composed member skips the app's hook, which reads
    /// only its own document, and names the member's entities from the member's preview."""


def main():
    text = FILE.read_text(encoding="utf-8")
    for old, new in EDITS + [(OLD_RESOLVER_HEAD, NEW_RESOLVER_HEAD), (OLD_DOC, NEW_DOC)]:
        if text.count(old) != 1:
            sys.exit(f"anchor not unique ({text.count(old)}): {old[:90]!r}")
        text = text.replace(old, new)
    FILE.write_text(text, encoding="utf-8")
    print("applied N3")


if __name__ == "__main__":
    main()
