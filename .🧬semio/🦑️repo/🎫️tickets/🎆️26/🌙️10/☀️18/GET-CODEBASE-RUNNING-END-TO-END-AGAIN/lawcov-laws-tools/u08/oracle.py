TREE_ALLOW = {"puzzle#3d", "puzzle#5d"}
LABELS_ALLOW = {"cad#CadLabels", "puzzle#Puzzle2dLabels", "puzzle#Puzzle3dLabels", "puzzle#Puzzle5dLabels"}
K = "app-plugin/"


def row(i, kind, line, prio="medium"):
    return [i, K + kind, prio, line]


def line_of(content, idx):
    return content[:idx].count("\n") + 1


def region_format(scope, lines):
    out, events = [], region_events(lines)
    for ev in events:
        if ev["spaceAfterSlashes"]:
            out.append(row("region-format-space-%s-%d" % (scope, ev["line"]), "region-marker-format", ev["line"]))
        if not ev["label"]:
            out.append(row("region-format-missing-label-%s-%d" % (scope, ev["line"]), "region-marker-format", ev["line"]))
    for span in pair_spans(events):
        if re.match(r"^tests?$", label_name(span["label"]), re.I) and span["label"] != "🧪Tests":
            out.append(row("region-tests-label-%s-%d" % (scope, span["startLine"]), "region-tests-label", span["startLine"]))
        if span["closeLabel"] and span["closeLabel"] != span["label"]:
            out.append(row("region-label-mismatch-%s-%d" % (scope, span["startLine"]), "region-marker-format", span["endLine"]))
    return out


def manifest_region(scope, lines):
    spans, tests, out = pair_spans(region_events(lines)), spans_of_mods(lines), []
    for i, line in enumerate(lines):
        if not re.search(r"App::builder\(", line):
            continue
        no = i + 1
        if in_spans(tests, no):
            continue
        if not any(s["label"] == "🔖Manifest" and s["startLine"] <= no <= s["endLine"] for s in spans):
            out.append(row("manifest-region-%s-%d" % (scope, no), "manifest-region", no))
    return out


def struct_naming(scope, content):
    ids = {m.group(1): m.group(2) for m in re.finditer(r'const\s+([A-Z][A-Z0-9_]*_APP_ID)\s*:\s*&str\s*=\s*"([^"]+)"', content, A)}
    out = []
    for m in re.finditer(r"impl\s+DocumentApp\s+for\s+(\w+)\s*\{", content, A):
        window = content[m.end(): m.end() + 2000]
        cm = re.search(r"\b([A-Z][A-Z0-9_]*_APP_ID)\b", window, A)
        ident = ids.get(cm.group(1)) if cm else None
        if not ident:
            continue
        if m.group(1) != pascal(ident):
            out.append(row("struct-naming-%s-%s" % (scope, m.group(1)), "struct-naming", line_of(content, m.start())))
    return out


def mod_layout(scope, lines):
    impls = [i + 1 for i, l in enumerate(lines) if re.search(r"impl\s+DocumentApp\s+for\s+\w+\s*\{", l, A)]
    if len(impls) < 2:
        return []
    spans = mod_spans(lines)
    return [row("mod-layout-%s-%d" % (scope, n), "mod-layout", n, "high") for n in impls if not any(s["startLine"] <= n <= s["endLine"] for s in spans)]


def selection_ids(scope, content):
    am = re.search(r"\bselection_ids\s+as\s+(\w+)\b", content, A)
    alias = am.group(1) if am else None
    out = []
    for m in re.finditer(r"fn\s+selection_ids\s*\(", content, A):
        body = extract_body(content, m.start())
        if "semio_framework_plugin::selection_ids(" in body or (alias and re.search(r"\b%s\(" % alias, body, A)):
            continue
        n = line_of(content, m.start())
        out.append(row("sdk-selection-ids-%s-%d" % (scope, n), "sdk-selection-ids", n))
    return out


def delegate(scope, content):
    out = []
    specs = [(r"fn\s+new_app_with_registry\s*[<(]", r"artifact_app_laws::new_app_with_registry(?:::<[^>]*>)?\s*\("), (r"fn\s+new_app\s*[<(]", r"artifact_app_laws::new_app(?:::<[^>]*>)?\s*\("), (r"fn\s+meta\s*\(", r"artifact_app_laws::meta\s*\(")]
    for rx, must in specs:
        for m in re.finditer(rx, content, A):
            if re.search(must, extract_body(content, m.start()), A):
                continue
            n = line_of(content, m.start())
            out.append(row("sdk-artifact-app-laws-delegate-%s-%d" % (scope, n), "sdk-artifact-app-laws-delegate", n))
    return out


def tree_item(crate, content, lines):
    scope, out, spans = crate["dir"], [], mod_spans(lines)
    for m in re.finditer(r"fn\s+tree_item_with_action\s*\(", content, A):
        n = line_of(content, m.start())
        mod = mod_at(spans, n) or app_id(scope)
        if ("%s#%s" % (crate["pluginId"], mod)) in TREE_ALLOW:
            continue
        out.append(row("sdk-tree-item-redefinition-%s-%d" % (scope, n), "sdk-tree-item", n))
    for m in re.finditer(r"fn\s+(tree_item_\w+)\s*\(", content, A):
        if m.group(1) == "tree_item_with_action":
            continue
        end = content.find(")", m.start())
        sig = "" if end == -1 else content[m.start(): content.find("{", end)]
        if "ActionDescriptor" not in sig:
            continue
        if "tree_item_with_action(" in extract_body(content, m.start()):
            continue
        n = line_of(content, m.start())
        out.append(row("sdk-tree-item-wrapper-%s-%d" % (scope, n), "sdk-tree-item", n))
    return out


def labels_struct(crate, content):
    scope, out = crate["dir"], []
    for m in re.finditer(r"struct\s+(\w*Labels)\s*\{", content, A):
        n = line_of(content, m.start())
        before = content.split("\n")[max(0, n - 8): n - 1]
        if any("app_labels!" in l for l in before):
            continue
        allowed = ("%s#%s" % (crate["pluginId"], m.group(1))) in LABELS_ALLOW
        out.append(row("sdk-labels-struct-%s-%s" % (scope, m.group(1)), "sdk-labels-struct", n, "low" if allowed else "medium"))
    return out


def run(c):
    lines = c["source"]
    content = "\n".join(lines) + "\n"
    r = c["rule"]
    return {
        "policyRegionFormatBreaches": lambda: region_format(c["scope"], lines), "policyManifestRegionBreaches": lambda: manifest_region(c["scope"], lines),
        "policyStructNamingBreaches": lambda: struct_naming(c["scope"], content), "policyModLayoutBreaches": lambda: mod_layout(c["scope"], lines),
        "policySelectionIdsBreaches": lambda: selection_ids(c["scope"], content), "policyArtifactAppLawDelegateBreaches": lambda: delegate(c["scope"], content),
        "policyTreeItemBreaches": lambda: tree_item(c["crate"], content, lines), "policyLabelsStructBreaches": lambda: labels_struct(c["crate"], content),
    }[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
