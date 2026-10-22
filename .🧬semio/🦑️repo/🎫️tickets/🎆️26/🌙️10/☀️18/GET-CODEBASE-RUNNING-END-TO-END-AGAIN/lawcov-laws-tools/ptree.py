import posixpath

_tax = os.path.dirname(os.path.abspath(__file__))
while not os.path.exists(os.path.join(_tax, "🔣taxonomy.json")):
    _tax = os.path.dirname(_tax)
TAXONOMY = json.load(open(os.path.join(_tax, "🔣taxonomy.json"), encoding="utf8"))
RS, TS = "🦀.rs", "🟦.ts"
PLUGINS = "✏s/🔌plugins"


def strip_non_ascii(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def utf16_index(text, idx):
    return len(text[:idx].encode("utf-16-le")) // 2


def subset_id(name):
    if name == TAXONOMY["subsetAnyDirName"]:
        return "*"
    prefix = TAXONOMY["subsetDirPrefix"]
    if not name.startswith(prefix):
        return None
    rest = name[len(prefix):]
    return rest if re.search(TAXONOMY["subsetSlugPattern"], rest) else None


def top_level_subsets(t):
    out = []
    for plugin, is_dir in t.children(PLUGINS).items():
        if not is_dir:
            continue
        arts = "%s/%s/%s" % (PLUGINS, plugin, TAXONOMY["artifactsDirName"])
        for art, a_dir in t.children(arts).items():
            if not a_dir:
                continue
            standards = "%s/%s/%s" % (arts, art, TAXONOMY["standardsDirName"])
            for std, s_dir in t.children(standards).items():
                if not s_dir or not std.startswith(TAXONOMY["standardDirPrefix"]):
                    continue
                subsets = "%s/%s/%s" % (standards, std, TAXONOMY["subsetsDirName"])
                for sub, u_dir in t.children(subsets).items():
                    if u_dir and subset_id(sub) is not None:
                        out.append("%s/%s" % (subsets, sub))
    return out


STDIO = "✏s/🔌plugins/🗄stdio/🗿artifacts"


def dialects(t):
    out = []
    for plugin, pd in sorted(t.children(PLUGINS).items()):
        if not pd:
            continue
        arts = "%s/%s/🗿artifacts" % (PLUGINS, plugin)
        for art, ad in sorted(t.children(arts).items()):
            if not ad:
                continue
            art_rel = arts + "/" + art
            for std, sd in sorted(t.children(art_rel + "/🏅standards").items()):
                if not sd or not std.startswith(TAXONOMY["standardDirPrefix"]):
                    continue
                std_rel = "%s/🏅standards/%s" % (art_rel, std)
                for sub, ud in sorted(t.children(std_rel + "/🪆subsets").items()):
                    if ud:
                        out.append({"artRel": art_rel, "standardRel": std_rel, "standardSlug": std[len(TAXONOMY["standardDirPrefix"]):], "subsetRel": "%s/🪆subsets/%s" % (std_rel, sub)})
    return out


def stdio_schema_owning(t):
    out = []
    for d in dialects(t):
        if d["artRel"].startswith(STDIO + "/") and t.exists(d["subsetRel"] + "/🧬schema/📸snapshot"):
            out.append({**d, "artifactId": strip_non_ascii(d["artRel"][len(STDIO) + 1:])})
    return out
