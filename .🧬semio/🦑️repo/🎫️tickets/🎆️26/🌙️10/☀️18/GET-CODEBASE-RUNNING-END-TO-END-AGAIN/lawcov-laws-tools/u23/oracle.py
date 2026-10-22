K = "stdio-artifacts/"
SLUG = re.compile(TAXONOMY["standardSlugPattern"])
SUB_SLUG = re.compile(TAXONOMY["subsetSlugPattern"])


def row(i, kind):
    return [i, K + kind, "high", 0]


def sub_name(sid):
    if sid == "*":
        return TAXONOMY["subsetAnyDirName"]
    return TAXONOMY["subsetDirPrefix"] + sid if SUB_SLUG.search(sid) else None


def coverage(t):
    out = []
    for d in dialects(t):
        name = d["subsetRel"].split("/")[-1]
        if not SLUG.search(d["standardSlug"]):
            out.append(row("standards-slug-" + d["standardRel"], "standards-coverage"))
        if subset_id(name) is None:
            out.append(row("standards-subset-prefix-" + d["subsetRel"], "standards-coverage"))
        for c in TAXONOMY["standardChildDirs"]:
            if c != "🪆subsets" and not t.exists("%s/%s" % (d["standardRel"], c)):
                out.append(row("standards-standard-child-%s-%s" % (d["standardRel"], c), "standards-coverage"))
        for c in TAXONOMY["subsetChildDirs"]:
            if not t.exists("%s/%s" % (d["subsetRel"], c)):
                out.append(row("standards-subset-child-%s-%s" % (d["subsetRel"], c), "standards-coverage"))
    return out


def has_impl(body, trait):
    s = re.sub(r"//[^\n]*", " ", re.sub(r"/\*.*?\*/", " ", body, flags=re.S))
    s = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
    return re.search(r"\bimpl\s*(?:<[^>{]*>\s*)?%s\b\s*(?:<[^>{]*>\s*)?for\b" % trait, s, A) is not None, s


def vocabulary(t):
    out, groups = [], {}
    for d in dialects(t):
        groups.setdefault(d["standardRel"], []).append(d)
    manifest_name = kind_file_json()
    for std_rel, ds in groups.items():
        art = ds[0]["artRel"]
        stdio = art.startswith(STDIO + "/")
        manifest = std_rel + "/🪆subsets/" + manifest_name
        v = lambda k: row("standards-subset-vocabulary-%s-%s" % (k, std_rel), "standards-subset-vocabulary")
        if manifest not in t.files:
            out.append(row("standards-subset-vocabulary-missing-" + std_rel, "standards-subset-vocabulary"))
            continue
        try:
            parsed = json.loads(t.files[manifest])
        except ValueError:
            out.append(v("invalid"))
            continue
        ids = list((parsed.get("subsets") or {}).keys())
        if parsed.get("standard") != ds[0]["standardSlug"]:
            out.append(v("standard-mismatch"))
        if "*" not in ids:
            out.append(v("missing-any"))
        for i in ids:
            if i != "*" and not SUB_SLUG.search(i):
                out.append(row("standards-subset-vocabulary-bad-id-%s-%s" % (std_rel, i), "standards-subset-vocabulary"))
        if not stdio and [i for i in ids if i != "*"]:
            out.append(v("domain-real-subset"))
        declared = {sub_name(i) for i in ids if sub_name(i) is not None}
        for i in ids:
            if sub_name(i) is None:
                out.append(row("standards-subset-vocabulary-unmapped-id-%s-%s" % (std_rel, i), "standards-subset-vocabulary"))
        actual = {d["subsetRel"].split("/")[-1] for d in ds}
        for name in sorted(actual - declared, key=lambda s: s.encode("utf-16-be")):
            out.append(row("standards-subset-vocabulary-undeclared-dir-%s-%s" % (std_rel, name), "standards-subset-vocabulary"))
        for name in sorted(declared - actual, key=lambda s: s.encode("utf-16-be")):
            out.append(row("standards-subset-vocabulary-missing-dir-%s-%s" % (std_rel, name), "standards-subset-vocabulary"))
        if stdio:
            for d in ds:
                name = d["subsetRel"].split("/")[-1]
                if (subset_id(name) or name) == "*":
                    continue
                body = t.files.get(d["subsetRel"] + "/🚪io/🦀.rs", "")
                impl, stripped = has_impl(body, "SubsetValidator")
                if not (impl and re.search(r"register_subset_validator\s*\(", stripped)):
                    out.append(row("standards-subset-vocabulary-validator-missing-" + d["subsetRel"], "standards-subset-vocabulary"))
    return out


def kind_file_json():
    k = TAXONOMY["fileKinds"][TAXONOMY["subsetsManifestFileKindId"]]
    return k["emoji"] + k["extensionChains"][0]


def run(c):
    t = Tree(expand(c))
    r = coverage(t) if c["rule"] == "policyStandardsCoverageBreaches" else vocabulary(t)
    return sorted(r, key=lambda x: json.dumps(x, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
