"""📖 One-off splitter of the shell chrome string match into contiguous group lookups: usage shell-vocab-split.py [APPLY]."""
import os, re, sys
V = "️"
F = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📖️shell-vocab/🦀️.rs"
text = open(F, encoding="utf8").read()
lines = text.split("\n")
assert lines[32].strip() == "match (key, is_de) {" and lines[631].strip() == "(other, _) => other,", (lines[32], lines[631])
groups = [
    ("🏠", "host-conflict-and-plugin-vocab", "host_conflict_and_plugin", 34, 153, "Host, conflict and plugin vocabulary: the host, display, common, block list, conflict, plugin and icon chrome strings.", "🧭", "Looks up the host, conflict and plugin chrome strings."),
    ("📄", "document-and-settings-vocab", "document_and_settings", 154, 339, "Document and settings vocabulary: the document, open artifact, surface and settings chrome strings.", "🗒", "Looks up the document and settings chrome strings."),
    ("🛍", "marketplace-chat-and-panel-vocab", "marketplace_chat_and_panel", 340, 461, "Marketplace, chat and panel vocabulary: the marketplace, tool, chat, fullscreen, panel toggle and task manager chrome strings.", "🧰", "Looks up the marketplace, chat and panel chrome strings."),
    ("🔎", "window-sync-and-search-vocab", "window_sync_and_search", 462, 583, "Window, sync and search vocabulary: the window, shell command, introduction, sync, hub, example, overlay, search, find, tree and command chrome strings.", "🔭", "Looks up the window, sync and search chrome strings."),
    ("✅", "check-in-and-palette-vocab", "check_in_and_palette", 584, 631, "Check in and palette vocabulary: the check in, command and palette group chrome strings.", "🎼", "Looks up the check in and palette chrome strings."),
]
base = os.path.dirname(F)
files = []
mods = []
chain = []
for emo, dirname, fn, a, b, desc, demo, fdoc in groups:
    conv = []
    for l in lines[a - 1:b]:
        m = re.match(r'^(\s*\(.*\) => )(".*"),$', l)
        assert m, l
        conv.append(m.group(1) + "Some(" + m.group(2) + "),")
    body = ["//! %s%s %s See [`super`]." % (emo, V, desc), "", "/// %s %s" % (demo, fdoc), "pub(super) fn shell_chrome_%s(key: &'static str, is_de: bool) -> Option<&'static str> {" % fn, "    match (key, is_de) {"] + conv + ["        _ => None,", "    }", "}", ""]
    d = emo + V + dirname
    files.append((os.path.join(base, d, "🦀️.rs"), "\n".join(body)))
    mods += ['#[path = "%s/🦀️.rs"]' % d, "mod %s;" % fn, "use %s::shell_chrome_%s;" % (fn, fn), ""]
    chain.append("shell_chrome_%s" % fn)
call = ["pub fn shell_chrome_string(key: &'static str, is_de: bool) -> &'static str {", "    " + chain[0] + "(key, is_de)"] + ["        .or_else(|| %s(key, is_de))" % c for c in chain[1:]] + ["        .unwrap_or(key)", "}"]
region = next(i for i, l in enumerate(lines) if l.startswith("//#region"))
sig = next(i for i, l in enumerate(lines) if l.startswith("pub fn shell_chrome_string"))
endfn = 634
assert lines[endfn - 1] == "}" and lines[endfn - 2].strip() == "}", (lines[endfn - 2], lines[endfn - 1])
new = lines[:region] + mods + lines[region:sig] + call + lines[endfn:]
print("root lines", len(new), "children", [(os.path.basename(os.path.dirname(p)), t.count("\n")) for p, t in files])
if len(sys.argv) > 1 and sys.argv[1] == "APPLY":
    for p, t in files:
        assert not os.path.exists(p), p
        os.makedirs(os.path.dirname(p)); open(p, "w", encoding="utf8").write(t)
    open(F, "w", encoding="utf8").write("\n".join(new))
    print("APPLIED")
