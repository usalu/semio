DOC_ROOT = "✏s/🔌plugins"
FILES_ALLOWED = {"🦀.rs", "AGENTS.md", "README.md", "LICENSE.md"}
FS = re.compile(r"\b(?:std::fs::|tokio::fs::|fs::|read_dir\s*\(|read_to_string\s*\(|File::)", A)
ENV = re.compile(r"\b(?:std::env::|std::process::|Command::new\s*\(|temp_dir\s*\()", A)
NET = re.compile(r"\b(?:std::net::|reqwest::|ureq::|hyper::|TcpStream\b)", A)
TLS = re.compile(r"\bthread_local!\s*(?:\{|\()", A)
SMUT = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+mut\b", A)
SITEM = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+(?:mut\s+)?\w+\s*:", A)
LAZY = re.compile(r"\blazy_static!\s*\{", A)
INTERIOR = re.compile(r"\b(Mutex|RwLock|RefCell|Cell)\s*<|\b(Atomic(?:Bool|I8|I16|I32|I64|Isize|U8|U16|U32|U64|Usize))\b", A)
TSIO = re.compile(r"\b(fetch\s*\(|XMLHttpRequest\b|WebSocket\b|localStorage\b|sessionStorage\b|indexedDB\b)", A)
FNITEM = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+\w", A)
STORAGE = re.compile(r"\b(?:localStorage|sessionStorage|indexedDB)\b", A)
EPHEMERAL = re.compile(r"\bephemeral(?:Box|Map|Set|WeakMap)\b", A)
FAMILY = ["register_mesh_exporter", "register_mesh_importer", "register_mesh_dwg_export_handler", "register_mesh_dwg_import_handler", "register_solid_exporter", "register_solid_importer", "register_2d_export_handlers", "register_dwg_import_handler", "register_app_io", "register_os_media_export_handler_kind", "register_os_media_import_handler_kind", "register_composer_entries", "set_io_fallback_dispatcher", "register_subset_validator", "register_format_descriptors", "register_artifact_schema_descriptor", "register_artifact_inference_descriptor", "register_app_schema_descriptor", "register_language", "register_document_codec", "register_document_codec_for_app", "register_dialect_migration", "register_os_fixture_json", "register_artifact_descriptors", "register_artifact_descriptor"]
FAMILY_RE = re.compile(r"\b(" + "|".join(FAMILY) + r")\b", A)
DEP_ALLOWED = {"semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-3d", "semio-framework-schema"} | {"semio-framework-3d-brep-" + s for s in "polynomial vector tolerance curve surface representation intersect queries tessellation validation analysis operations engine".split()}
EFFECT_CAP = {"OpenWindow": "Window", "CloseWindow": "Window", "Notify": "Window", "ClipboardWrite": "Document", "RequestSync": "Backbone", "Navigate": "Window", "LoadDocument": "Document", "OpenExternalUrl": "Network", "SetPanel": "Window", "DownloadMediaExport": "Asset", "IconRenderExport": "Asset", "VideoRenderExport": "Asset", "RequestFileOpen": "Asset", "RequestMediaFrames": "Asset", "SpawnPluginInstance": "Window", "OpenPluginInstance": "Window", "SetActiveUtility": "Window", "SetActiveTool": "Window", "OpenDialog": "Window", "DispatchAction": "Window", "ReplayShellCommand": "Window", "InvokeExtension": "Engine"}
CEILING = {"plugin-closed-shape": 41, "plugin-dependency-allowlist": 105, "plugin-dependency-os-host": 10, "effect-capability-parity": 47}
EXCLUDED = ("compose/", "♻mit-bestand/", "🌎hub/", "🧰framework/🛍products/🦑repo/")
ADAPTERS = ("🧰framework/🔨modules/🖥platform/", "🧰framework/🛍products/💻os/🔨modules/🏪store/👷worker/🟦.ts", "🧰framework/🛍products/💻os/🔨modules/🏪store/👷worker/🗂config-lane/🟦.ts")
taxonomy_path = os.path.dirname(os.path.abspath(__file__))
while not os.path.exists(os.path.join(taxonomy_path, "🔣taxonomy.json")):
    taxonomy_path = os.path.dirname(taxonomy_path)
TAXONOMY = json.load(open(os.path.join(taxonomy_path, "🔣taxonomy.json"), encoding="utf8"))
ALLOWED_DIRS = set(TAXONOMY["pluginChildDirs"]) | {TAXONOMY["artifactsDirName"], TAXONOMY["packagesDirName"]} | set(TAXONOMY["rootDataDirNames"])
LEGACY = {"🎟capabilities", "🔧setup", "🛂manifest"}


def row(i, kind, priority, line=0):
    return [i, kind, priority, line]


def closed_shape(files):
    out, owners = [], {}
    for path in files:
        parts = path.split("/")
        if len(parts) > 3 and "/".join(parts[:2]) == DOC_ROOT:
            owners.setdefault(parts[2], {}).setdefault(parts[3], len(parts) > 4)
    for owner, children in owners.items():
        owner_rel = DOC_ROOT + "/" + owner
        for name, is_dir in children.items():
            rel = owner_rel + "/" + name
            if is_dir:
                if name in ALLOWED_DIRS:
                    continue
                out.append(row("plugin-closed-shape-" + ("junk" if name in ("node_modules", ".DS_Store") else "dir") + "-" + rel, "taxonomy/plugin-closed-shape", "medium"))
            elif name not in FILES_ALLOWED:
                out.append(row("plugin-closed-shape-file-" + rel, "taxonomy/plugin-closed-shape", "medium"))
    return out


def purity(files):
    out = []
    for path in walk_files(files):
        if not path.endswith(".rs") or not path.startswith(DOC_ROOT + "/"):
            continue
        lines = split_lines(files[path])
        skip = spans_of_mods(lines) + test_fn_spans(lines)
        depth, fn_depths = 0, []
        for i, raw in enumerate(lines):
            no = i + 1
            co = code_only(raw)
            delta = co.count("{") - co.count("}")
            if in_spans(skip, no):
                depth += delta
                while fn_depths and depth < fn_depths[-1]:
                    fn_depths.pop()
                continue
            in_fn = any(depth >= d for d in fn_depths)
            opens = (not in_fn) and FNITEM.search(raw) and "{" in co

            def emit(sub):
                out.append(row("plugin-purity-%s-%s-%d" % (sub, path, no), "taxonomy/plugin-purity-" + sub, "medium", no))

            if FS.search(co):
                emit("filesystem-io")
            if ENV.search(co):
                emit("env-process-io")
            if NET.search(co):
                emit("network-io")
            if (not in_fn) or SITEM.search(raw):
                if TLS.search(co):
                    emit("thread-local-state")
                if SMUT.search(raw):
                    emit("static-mut-state")
                if LAZY.search(co):
                    emit("lazy-static-state")
                m = INTERIOR.search(co)
                if m:
                    emit("interior-mutability-" + (m.group(1) or m.group(2)).lower())
            if opens:
                fn_depths.append(depth + 1)
            depth += delta
            while fn_depths and depth < fn_depths[-1]:
                fn_depths.pop()
    for path in ts_files(files, DOC_ROOT):
        for i, raw in enumerate(split_lines(files[path])):
            m = TSIO.search(re.sub(r"//.*$", "", raw))
            if m:
                out.append(row("plugin-purity-ts-side-effect-%s-%d" % (path, i + 1), "taxonomy/plugin-purity-ts-side-effect", "medium", i + 1))
    return out


def test_fn_spans(lines):
    spans, depth, pending, recording, record_depth, open_line = [], 0, False, False, 0, -1
    for i, raw in enumerate(lines):
        co = code_only(raw)
        if not recording:
            if re.match(r"^\s*#\[(?:test|tokio::test|async_std::test|wasm_bindgen_test)\b", raw, A):
                pending = True
            elif pending and re.match(r"^\s*#\[", raw):
                pass
            elif pending and re.search(r"\bfn\s+\w", raw, A):
                if "{" in co:
                    recording, record_depth, open_line = True, depth, i + 1
                pending = False
            elif pending:
                pending = False
        depth += co.count("{") - co.count("}")
        if recording and depth <= record_depth:
            spans.append((open_line, i + 1))
            recording = False
    return spans


def ts_files(files, root):
    found = []
    for path in walk_files(files, root):
        parts = path.split("/")
        if "🧪tests" in parts[:-1] or parts[-1] == "🟦.ts":
            continue
        if parts[-1].endswith(".ts") or parts[-1].endswith(".tsx"):
            found.append(path)
    return found


def state_lane(files):
    out, seen = [], set()
    for root in ("🧰framework", "✏s"):
        for path in ts_files(files, root):
            if path in seen:
                continue
            seen.add(path)
            if path.startswith(EXCLUDED) or re.search(r"\.test\.tsx?$", path) or "/🧪" in path:
                continue
            adapter = path.startswith(ADAPTERS)
            for i, raw in enumerate(split_lines(files[path])):
                stripped = raw.lstrip()
                if stripped.startswith(("*", "/*", "//")):
                    continue
                co = re.sub(r"/\*.*?\*/", "", re.sub(r"//.*$", "", mask(raw)))
                if not adapter and STORAGE.search(co):
                    out.append(row("state-lane-storage-%s-%d" % (path, i + 1), "taxonomy/state-lane-storage-outside-config-lane", "medium", i + 1))
                if EPHEMERAL.search(co):
                    out.append(row("state-lane-ephemeral-box-%s-%d" % (path, i + 1), "taxonomy/state-lane-ephemeral-box", "medium", i + 1))
    return out


def registration(files):
    out = []
    for path in walk_files(files):
        if not path.endswith(".rs") or not path.startswith(DOC_ROOT + "/"):
            continue
        text = files[path]
        pos = 0
        for b in re.finditer(r"Plugin::builder\s*\(", text):
            end = text.find(".build()", b.start())
            chain = text[b.start(): len(text) if end == -1 else end] if end != -1 else text[b.start(): min(len(text), b.start() + 4000)]
            s = re.search(r"\.setup\s*\(", chain)
            if s:
                line = text[: b.start() + s.start()].count("\n") + 1
                out.append(row("plugin-registration-setup-callback-%s-%d" % (path, line), "taxonomy/plugin-registration-setup-callback", "medium", line))
        a = path.find("/🗿artifacts/")
        engine = a != -1 and path.find("/⚙engine/", a) != -1
        for i, raw in enumerate(split_lines(text)):
            co = code_only(raw)
            for sub, hit in (("family-call", FAMILY_RE.search(co)), ("os-host-path", re.search(r"\bsemio_framework_os::", co, A))):
                if hit:
                    kind = "taxonomy/plugin-registration-" + ("engine-backlog" if engine else "violation")
                    out.append(row("plugin-registration-%s-%s-%s-%d" % ("engine-backlog" if engine else "violation", sub, path, i + 1), kind, "medium", i + 1))
    return out


def dependencies(files):
    out = []
    for path in walk_files(files):
        if os.path.basename(path) != "Cargo.toml" or not path.startswith(DOC_ROOT + "/") or "/📦packages/🦀rust/" not in path:
            continue
        content = files[path]
        headers = [(m.start(), m.end()) for m in re.finditer(r"^\[[^\]]*\][ \t\r]*$", content, re.M)]
        for h in re.finditer(r"^\[(?:dependencies|target\.[^\]]*\.dependencies)\][ \t\r]*$", content, re.M):
            start = h.end()
            end = next((a for a, _ in headers if a > start), len(content))
            block = content[start:end]
            for m in re.finditer(r"^([A-Za-z0-9_.-]+)[ \t]*=[ \t]*\{([^}]*)\}[ \t\r]*$", block, re.M):
                p = re.search(r'path\s*=\s*"([^"]*)"', m.group(2))
                if not p or "🧰framework" not in p.group(1):
                    continue
                pkg = re.search(r'package\s*=\s*"([^"]*)"', m.group(2))
                name = pkg.group(1) if pkg else m.group(1)
                if name in DEP_ALLOWED or re.fullmatch(r"serde(_json|-wasm-bindgen)?", name):
                    continue
                line = content[: start + m.start()].count("\n") + 1
                host = name == "semio-framework-os"
                out.append(row("plugin-dependency-allowlist-%s-%s" % (path, name), "taxonomy/plugin-dependency-" + ("os-host" if host else "allowlist"), "medium", line))
    return out


def effects(files):
    out = []
    owners = sorted({p.split("/")[2] for p in files if p.startswith(DOC_ROOT + "/") and len(p.split("/")) > 3})
    for owner in owners:
        owner_rel = DOC_ROOT + "/" + owner
        declared = set()
        leaf = files.get(owner_rel + "/🦀.rs")
        if leaf is not None:
            declared |= set(re.findall(r"\.capability\s*\(\s*ArtifactKind::(\w+)", leaf, A))
            if re.search(r"\.local_backbone_storage\s*\(", leaf, A):
                declared.add("Backbone")
        first = {}
        count = {}
        for path in walk_files(files, owner_rel):
            if not path.endswith(".rs"):
                continue
            lines = split_lines(files[path])
            spans = spans_of_mods(lines)
            for i, raw in enumerate(lines):
                if in_spans(spans, i + 1):
                    continue
                for m in re.finditer(r"\bEffect::(\w+)\b", code_only(raw), A):
                    if m.group(1) in EFFECT_CAP:
                        first.setdefault(m.group(1), (path, i + 1))
        for variant, (path, line) in first.items():
            if EFFECT_CAP[variant] not in declared:
                out.append(row("effect-capability-parity-%s-%s" % (owner_rel, variant), "taxonomy/effect-capability-parity", "medium", line))
    return out


def ratchet(rows):
    key = lambda kind: "plugin-purity" if kind.startswith("taxonomy/plugin-purity-") else (kind[len("taxonomy/"):] if kind.startswith("taxonomy/") else kind)
    groups = {}
    for r in rows:
        groups.setdefault(key(r[1]), []).append(r)
    out = []
    for k, lst in groups.items():
        ceil = CEILING.get(k)
        for i, r in enumerate(lst):
            out.append(r if ceil is None or i < ceil else [r[0], r[1], "high", r[3]])
    return out


def apa(files):
    return ratchet(closed_shape(files) + purity(files) + state_lane(files) + registration(files) + dependencies(files) + effects(files))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
result = {}
for c in cases:
    rows = (state_lane if c["rule"] == "policyStateLaneExhaustivenessBreaches" else apa)(expand(c))
    result[c["id"]] = {p: sum(1 for r in rows if r[2] == p) for p in sorted({r[2] for r in rows})} if "expectCounts" in c else rows
output(result)
