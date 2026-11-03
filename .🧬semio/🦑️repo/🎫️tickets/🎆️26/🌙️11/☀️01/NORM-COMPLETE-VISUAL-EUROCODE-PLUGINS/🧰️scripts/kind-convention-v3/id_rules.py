"""🆔 Rewrites one artifact's edit rules so that a list is removed by row id: the `RemoveItemRule` of the list becomes a shared `IdRemoval`, the family gets (or keeps) `ID_REMOVALS` and `resolve_edit` through `resolve_with_id_removals`.

📎 [shared id removals](../../../../../../../../✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs)
"""
import re

OLD_STRUCT = re.compile(r"/// 🆔️ A collection whose remove kind addresses the row by its id instead of its position\.\n#\[derive\(Clone, Copy, Debug\)\]\npub struct IdRemoval \{[^}]*\}\n\n")
OLD_ROW = re.compile(r'IdRemoval \{ path: "([\w\[\].]+)", kind: "([\w-]+)", id: "(\w+)" \}')
OLD_RESOLVE = re.compile(r"\n/// 🎯️ Resolves an editor edit into the concrete typed kind mutation[^\n]*\npub fn resolve\([^\n]*\n[^\n]*\n\}\n")
BLOCK = '''
/// 🆔️ Every collection of this family removed by row id, never by position.
pub const ID_REMOVALS: &[IdRemoval] = &[
%s];
'''
RESOLVE = '''
/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals by [`ID_REMOVALS`], every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &%sSnapshot, edit: &NormEdit) -> Result<Vec<%sMutation>, String> {
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
'''


def rewrite(text, short, P, rules):
    """`rules`: [(path, kind, id_field)] of the `RemoveItemRule`s (selectors `&[]` only) to turn into id removals."""
    entries = ""
    for path, kind, field in rules:
        old = '        RemoveItemRule { path: "%s", kind: "%s", selectors: &[], index: "index" },\n' % (path, kind)
        if old not in text:
            continue
        text = text.replace(old, "")
        entries += '        IdRemoval { path: "%s", kind: "%s", selectors: &[], id: "%s" },\n' % (path, kind, field)
    if not entries:
        return text
    text = OLD_STRUCT.sub("", text)
    text = OLD_ROW.sub(r'IdRemoval { path: "\1", kind: "\2", selectors: &[], id: "\3" }', text)
    if "pub const ID_REMOVALS" in text:
        text = re.sub(r"(pub const ID_REMOVALS: &\[IdRemoval\] = &\[\n)", lambda m: m.group(1) + entries, text, 1)
    else:
        text = text.rstrip("\n") + "\n" + BLOCK % entries
    if "pub fn resolve_edit" not in text or "resolve_with_id_removals" not in text:
        text = OLD_RESOLVE.sub("", text)
        text = re.sub(r"\n/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals first[^\n]*\npub fn resolve_edit\(.*\n\}\n", "", text, flags=re.S)
        text = text.rstrip("\n") + "\n" + RESOLVE % (P, P)
    names = {"IdRemoval", "InsertItemRule", "NormEdit", "NormEditRules", "SelectorKey", "SetFieldRule"} | ({"RemoveItemRule"} if "RemoveItemRule {" in text else set())
    text = re.sub(r"use semio_s_plugin_norm_app_surface::\{[^}]*\};", "use semio_s_plugin_norm_app_surface::{%s};" % ", ".join(sorted(names)), text, 1)
    for use in ("use crate::%sMutation;" % P, "use semio_s_artifact_norm_%s_model::%sSnapshot;" % (short, P)):
        if use not in text:
            text = re.sub(r"(use semio_s_plugin_norm_app_surface::\{[^}]*\};\n)", lambda m: m.group(1) + use + "\n", text, 1)
    return text
