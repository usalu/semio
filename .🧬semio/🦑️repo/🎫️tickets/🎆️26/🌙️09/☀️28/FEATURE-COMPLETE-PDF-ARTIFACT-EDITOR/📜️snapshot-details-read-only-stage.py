"""👁️ Stage schema-owned read-only rendering for observed PDF details."""

import hashlib
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "snapshot-read-only"
DETAILS = Path("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details")


def change(text, before, after, count=1):
    """🎯 Replace exactly the specified number of source anchors."""
    assert text.count(before) == count, before
    return text.replace(before, after)


def sources(pdf):
    """📚 Compose the annotation, provider, view and neutral test."""
    mutations = {}
    def update(path, edit):
        current = path.read_bytes()
        next_text = edit(current.decode("utf8"))
        mutations[path] = (current, next_text.encode("utf8"))

    schema_helpers = (TICKET / "📜️snapshot-read-only-path.rs").read_text(encoding="utf8")
    update(DETAILS / "🧬️schema-resolution/🦀️.rs", lambda text: change(text, "pub(super) fn schema_at_path<'a>(", schema_helpers + "\n" + "pub(super) fn schema_at_path<'a>("))

    def provider(text):
        point = "    fn creation_schema_at_path<'b>"
        text = change(text, point, '''    fn path_read_only(&self, root: &DslValue, path: &[SnapshotDetailPathSegment]) -> bool {
        schema_path_read_only(root, path, &mut |path| self.scalar(path), &mut |path| self.shape(path))
    }

''' + point)
        point = '        if matches!(path.last(), Some(SnapshotDetailPathSegment::Key(key)) if key == "schema") {'
        text = change(text, point, point[:-2] + '\n            || self.schema.as_deref().is_some_and(|root| self.path_read_only(root, path)) {')
        point = "        let Some(root) = self.schema.as_ref() else { return true };\n        let Some(schema) = self.schema_at_path(root, path) else { return true };\n        let length"
        text = change(text, point, '''        let Some(root) = self.schema.as_ref() else { return true };
        if self.path_read_only(root, path) { return false; }
        let Some(schema) = self.schema_at_path(root, path) else { return true };
        let length''')
        point = '''                self.shape(&property).is_none()'''
        return change(text, point, '''                (!schema_read_only(root, schema) && self.shape(&property).is_none())''', count=2)
    update(DETAILS / "🗂️dsl-provider/🦀️.rs", provider)

    def view(text):
        text = change(text, '''    let template = provider.creation_template(path, false);
    let enum_values = provider.enum_values(path);
    let allows_untyped_creation = provider.allows_untyped_creation(path);
    let capabilities = provider.item_capabilities(path);''', '''    let capabilities = provider.item_capabilities(path);
    let (template, enum_values, allows_untyped_creation) = if capabilities.edit {
        (provider.creation_template(path, false), provider.enum_values(path), provider.allows_untyped_creation(path))
    } else {
        (None, Vec::new(), false)
    };''')
        text = change(text, '''    if capabilities.edit {
        if let Some(value) = value {
            controls.push(scalar_control(id, path, value, template, enum_values, allows_untyped_creation, labels, controller_id, publication_revision));
        }
    }''', '''    if let Some(value) = value {
        controls.push(if capabilities.edit {
            scalar_control(id, path, value, template, enum_values, allows_untyped_creation, labels, controller_id, publication_revision)
        } else {
            read_only_value(&format!("{id}-read-only"), value)
        });
    }''')
        text = change(text, '''    let controls = if path_is_bindable(&path_pointer) {
        item_controls''', '''    let immutable = !(capabilities.edit || capabilities.rename || capabilities.remove || capabilities.reorder);
    let controls = if path_is_bindable(&path_pointer) || immutable {
        item_controls''')
        text = change(text, '''        if path_is_bindable(&path_pointer) {
            builder = builder.try_child(collection_controls''', '''        if capabilities.edit && path_is_bindable(&path_pointer) {
            builder = builder.try_child(collection_controls''')
        return change(text, '''        let controls = provider.variant_count(&[], locale) > 0
            || (provider.allows_collection_insert(&[])
                && (provider.missing_property_count(&[]) > 0 || (collection_insert_path(provider, &[], &value).is_some() && (provider.allows_untyped_creation(&[]) || provider.creation_template(&[], true).is_some()))));''', '''        let controls = provider.item_capabilities(&[]).edit && (provider.variant_count(&[], locale) > 0
            || (provider.allows_collection_insert(&[])
                && (provider.missing_property_count(&[]) > 0 || (collection_insert_path(provider, &[], &value).is_some() && (provider.allows_untyped_creation(&[]) || provider.creation_template(&[], true).is_some())))));''')
    update(DETAILS / "🌲️detail-tree-and-window/🦀️.rs", view)

    update(DETAILS / "🏷️labels-and-controls/🦀️.rs", lambda text: change(text, '''    readonly_long_text(id, value)
}''', '''    if UiText::try_from_str(&value).is_some() {
        return ui_contract::text(label(&value)?).try_id(id).map_err(|_| error("ui.snapshot-details.read-only-id"))?
            .try_build().map_err(|_| error("ui.snapshot-details.read-only-value"));
    }
    readonly_long_text(id, value)
}'''))

    def snapshot(text):
        for name in ("schema", "declaredVersion", "admittedStreamRoles"):
            old = f'    "{name}": {{'
            text = change(text, old, old + ' "readOnly": true,')
        for name, en, de in (
            ("properties", "Marked content properties", "Eigenschaften markierter Inhalte"),
            ("admittedStreamRoles", "Recognized stream roles", "Erkannte Streamrollen"),
        ):
            point = f'    "{name}": {{'
            begin = text.index(point)
            insertion = len(point) + (len(' "readOnly": true,') if name == "admittedStreamRoles" else 0)
            marker = json.dumps({"x-semio-ui": {"label": {"en": en, "de": de}}}, ensure_ascii=False)[1:-1]
            text = text[:begin + insertion] + ' ' + marker + ',' + text[begin + insertion:]
        assert json.loads(text)["properties"]["admittedStreamRoles"]["readOnly"] is True
        return text
    update(pdf / "🧬️schema/📸️snapshot/🔣️.json", snapshot)

    law = (TICKET / "📜️snapshot-details-read-only-law.rs").read_text(encoding="utf8")
    update(DETAILS / "🧪️tests/🔬️unit/🔀️union-source-and-window-laws/🦀️.rs", lambda text: text.rstrip() + "\n\n" + law)
    fixture = DETAILS / "🧪️tests/🧫️fixtures/🔒️read-only/🔣️.json"
    assert not fixture.exists()
    mutations[fixture] = (None, (TICKET / "🔣️snapshot-details-read-only.json").read_bytes())
    return mutations


def digest(content):
    """🔑 Identify one complete source body."""
    return None if content is None else hashlib.sha256(content).hexdigest()


def stage(pdf):
    """📦 Record all source preimages and the staged content."""
    OUT.mkdir(parents=True, exist_ok=True)
    roster = {}
    for index, (path, (before, after)) in enumerate(sources(pdf).items()):
        target = OUT / f"source-{index}{path.suffix}"
        assert len(str(path.resolve()).encode("utf-16-le")) // 2 <= 256, path
        assert (path.read_bytes() if path.is_file() else None) == before, path
        target.write_bytes(after)
        roster[str(path)] = {"before": digest(before), "after": digest(after), "staged": str(target.resolve())}
    (OUT / "stage.json").write_text(json.dumps(roster, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Staged {len(roster)} read-only detail paths")


if __name__ == "__main__":
    stage(Path(sys.argv[1]))
