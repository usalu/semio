from pathlib import Path

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
core = next(c for c in space.iterdir() if "core" in c.name) / "🦀️.rs"
text = core.read_text()

needle = "    pub origin: &'static str,\n    /// 🛂️ The CALLING"
if "pub data_class:" not in text[text.find("pub struct HomeSpaceRow"): text.find("pub struct HomeSpaceRow") + 900]:
    if needle not in text:
        raise SystemExit("origin+shield needle missing")
    text = text.replace(
        needle,
        "    pub origin: &'static str,\n"
        "    /// 📂️ Persistence data class — hub=persistedShared; local catalog=persistedLocalOnly;\n"
        "    /// ephemeral draft studios (empty backbone_uri)=ephemeralLocalOnly.\n"
        "    pub data_class: &'static str,\n"
        "    /// 🛂️ The CALLING",
        1,
    )
    print("inserted data_class field")
else:
    print("data_class field already present")

old_hub = (
    '            origin: "hub",\n'
    "            role: caller_role(space, user_id),\n"
)
new_hub = (
    '            origin: "hub",\n'
    '            data_class: "persistedShared",\n'
    "            role: caller_role(space, user_id),\n"
)
if old_hub not in text:
    raise SystemExit("hub origin/role needle missing")
text = text.replace(old_hub, new_hub, 1)
print("patched hub row")

old_local = (
    '            origin: "local",\n'
    "            // 🏠️ The local-only catalog is single-user by construction and carries no directory\n"
    "            // membership; a local row therefore never offers a directory-owned affordance.\n"
    "            role: None,\n"
)
new_local = (
    '            origin: "local",\n'
    '            data_class: if entry.backbone_uri.is_empty() { "ephemeralLocalOnly" } else { "persistedLocalOnly" },\n'
    "            // 🏠️ The local-only catalog is single-user by construction and carries no directory\n"
    "            // membership; a local row therefore never offers a directory-owned affordance.\n"
    "            role: None,\n"
)
if old_local not in text:
    raise SystemExit("local origin/role needle missing: " + repr(text[text.find('origin: "local"'):text.find('origin: "local"')+250]))
text = text.replace(old_local, new_local, 1)
print("patched local row")

core.write_text(text)
print("OK", core)
