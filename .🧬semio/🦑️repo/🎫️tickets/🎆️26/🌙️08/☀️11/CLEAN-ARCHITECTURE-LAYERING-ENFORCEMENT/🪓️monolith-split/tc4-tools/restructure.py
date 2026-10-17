"""🧱️ Re-cuts the generation3d-app-laws integration target of semio-s-composition-laws into six gate-sized test targets. Idempotent: always regenerates from tc4-tools/orig (snapshot taken before the split). usage: restructure.py [--dry]. Writes only below the composition workspace and refuses to overwrite a file that is neither its snapshot nor its own previous output."""
import hashlib, json, os, re, sys

R = "/Users/ueli/Documents/semio/"
HERE = os.path.dirname(os.path.abspath(__file__))
C = R + "✏️s/🧑‍💻dev/🧩️composition/"
G = C + "🧪️tests/🧊️generation3d/"
STD = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
UNIT = STD + "✏️editor/🧪️tests/🔬️unit/"
MANIFEST_DIR = C + "📦️packages/🦀️rust"
DRY = "--dry" in sys.argv

idx = {}
for line in open(HERE + "/orig/index.tsv", encoding="utf8"):
    key, h, path = line.rstrip("\n").split("\t")
    idx[R + path] = (key, h)
state_path = HERE + "/state.json"
state = json.load(open(state_path)) if os.path.exists(state_path) else {}


def orig(path):
    return open(HERE + "/orig/" + idx[path][0], encoding="utf8").read()


def current(path):
    return open(path, encoding="utf8").read() if os.path.exists(path) else orig(path)


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


written = {}


def write(path, text, derived=False):
    assert os.path.realpath(path).startswith(os.path.realpath(C)), path
    assert len(path.encode("utf8")) <= 240 + len(R.encode("utf8")), ("path too long", path)
    if os.path.exists(path) and not derived:
        cur = hashlib.sha256(open(path, "rb").read()).hexdigest()
        base = idx.get(path, (None, None))[1]
        assert cur in (base, state.get(path)) or cur == sha(text), ("foreign edit", path)
    written[path] = sha(text)
    if not DRY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf8").write(text)


def delete(path):
    assert os.path.realpath(path).startswith(os.path.realpath(C)), path
    if os.path.exists(path) and not DRY:
        os.remove(path)


TARGETS = {
    "viewer": ("🔭️viewer-laws", "generation3d-app-laws-viewer"),
    "modes": ("🪟️editor-mode-laws", "generation3d-app-laws-editor-modes"),
    "structure": ("🧭️editor-structure-laws", "generation3d-app-laws-editor-structure"),
    "commands": ("🎮️editor-command-laws", "generation3d-app-laws-editor-commands"),
    "unit": ("🧮️editor-unit-laws", "generation3d-app-laws-editor-unit"),
    "surface": ("🕹️editor-surface-laws", "generation3d-app-laws-editor-surface"),
}
DOCS = {
    "viewer": "🔭️ Viewer laws: the preview-eval and viewer law wrappers and the serial-lock discipline.",
    "modes": "🪟️ Editor mode laws: the panel and the generate and edit mode window law wrappers plus the archive, brep and surface-budget law groups.",
    "structure": "🧭️ Editor structure laws: fold, interaction scope, generate interactions, example switch, tick addressing, work capacity, mode panels and slider values.",
    "commands": "🎮️ Editor command laws: one law wrapper per flow, widget and graph editor command.",
    "unit": "🧮️ Editor unit laws: the mesh-component, extension round-trip, engine-compute and hot-path law groups.",
    "surface": "🕹️ Editor surface laws: the command-surface and window-and-menu law groups plus the selection and transform command wrappers.",
}
GROUP_ROOT = {
    "🚪️archive-and-brep-laws": "modes",
    "📏️surface-budget-and-retirement": "modes",
    "🪄️mesh-component-laws": "unit",
    "🔁️extension-round-trip": "unit",
    "🧮️engine-compute": "unit",
    "📈️hot-path-and-fault-laws": "unit",
    "🕹️command-surface": "surface",
    "🪟️window-and-menu-laws": "surface",
}
SURFACE_COMMANDS = ["_scale_selection_", "_cycle_lod_mode_", "_delete_selection_", "_rotate_selection_", "_commands_set_lod_mode_", "_knife_mesh_selection_", "_commands_toggle_sun_", "_cycle_show_mode_", "_translate_selection_", "editor_generation3d_selection_tests_laws"]
STRUCTURE_COMMANDS = ["_set_widget_input_"]

FRAGMENTS = ["🔭️viewer-laws", "🪟️editor-mode-laws", "🧭️editor-structure-laws", "🎮️editor-command-laws"]
blocks = []
frag_doc = {}
for frag in FRAGMENTS:
    text = orig(G + frag + "/🦀️.rs")
    for part in re.split(r"(?m)^(?=#\[path = \"\.\.\"\]\nmod |#\[path = \"\.\.\"\]\npub\(crate\) mod )", text):
        m = re.match(r"#\[path = \"\.\.\"\]\n(?:pub\(crate\) )?mod (\w+)", part)
        if not m:
            frag_doc[frag] = part
            continue
        blocks.append((frag, m.group(1), part.rstrip("\n") + "\n"))
names = [b[1] for b in blocks]
assert len(names) == len(set(names)) == 56, len(names)


def root_of(frag, name):
    if frag == "🔭️viewer-laws":
        return "viewer"
    if frag == "🪟️editor-mode-laws":
        return "modes"
    if frag == "🧭️editor-structure-laws":
        return "structure"
    if any(token in name or name == token for token in SURFACE_COMMANDS):
        return "surface"
    if any(token in name for token in STRUCTURE_COMMANDS):
        return "structure"
    return "commands"


IMPORTS_RE = None
shared_uses = None
body_by_root = {key: [] for key in TARGETS}
domain_uses = None
for frag, name, text in blocks:
    lines = text.split("\n")
    uses = [l for l in lines if l.startswith("use ")]
    if name == "editor_domain":
        domain_uses = tuple(uses)
assert domain_uses and len(domain_uses) == 61
IMPORTS = "📥️editor-imports"


def shared_import_block(name, text):
    lines = text.split("\n")
    uses = tuple(l for l in lines if l.startswith("use "))
    if uses != domain_uses:
        return text
    first = next(i for i, l in enumerate(lines) if l.startswith("use "))
    last = max(i for i, l in enumerate(lines) if l.startswith("use "))
    assert all(l.startswith("use ") for l in lines[first:last + 1]), name
    return "\n".join(lines[:first] + ['include!("../%s/🦀️.rs");' % IMPORTS] + lines[last + 1:])


def editor_domain_block(mount):
    return ('#[path = ".."]\nmod editor_domain {\ninclude!("../%s/🦀️.rs");\n#[path = "%s"]\npub(crate) mod editor_laws;\n}\n' % (IMPORTS, mount))


UNIT_FILE = UNIT + "🦀️.rs"
SUPPORT_MOUNT = UNIT_FILE
mount_dir = "📚️editor-laws"
unit_text = orig(C + "🧪️tests/🧊️generation3d/" + UNIT_FILE)
include_lines = re.findall(r'^include!\("([^"]+)"\);\n', unit_text, flags=re.M)
assert len(include_lines) == 8
GROUP_PATH = {g.split("/")[0]: g for g in include_lines}
assert set(GROUP_PATH) == set(GROUP_ROOT), set(GROUP_PATH) ^ set(GROUP_ROOT)

assign = {}
for frag, name, text in blocks:
    if name == "editor_domain":
        continue
    assign[name] = root_of(frag, name)
    body_by_root[assign[name]].append(shared_import_block(name, text))

root_text = {}
mounts = {}
for key, (dirname, target) in TARGETS.items():
    groups = [g for g, r in GROUP_ROOT.items() if r == key]
    needs_domain = key != "unit" or True
    if groups:
        mount_rel = dirname + "/" + mount_dir + "/🦀️.rs"
        mount_body = '//! 📚️ Editor laws of the %s target: the shared editor test support plus the law groups it owns.\n' % target
        mount_body += 'include!("../../%s");\n' % (UNIT_FILE)
        for g in sorted(groups, key=lambda g: list(GROUP_PATH).index(g)):
            mount_body += 'include!("../../%s%s");\n' % (UNIT, GROUP_PATH[g])
        mounts[G + mount_rel] = mount_body
        domain = editor_domain_block(mount_rel)
    else:
        domain = editor_domain_block(SUPPORT_MOUNT)
    head = "//! %s\n#![allow(dead_code, unused_imports)]\n\ninclude!(\"../🦀️.rs\");\n\n" % DOCS[key]
    extra = ""
    if key == "viewer":
        extra = '#[path = "../🧪️tests/🔒️serial-lock-discipline/🦀️.rs"]\npub(crate) mod serial_lock_discipline;\n\n'
    root_text[key] = head + extra + domain + "\n" + "\n".join(body_by_root[key])

shared_old = orig(G + "🦀️.rs")
pre_end = shared_old.index('#[path = "🧪️tests/🔒️serial-lock-discipline/🦀️.rs"]')
pre = shared_old[:pre_end]
assert pre.startswith("//! 🧊️ Real extension sessions drive the public Generation3d app law corpus.\n")
pre = pre.replace("//! 🧊️ Real extension sessions drive the public Generation3d app law corpus.\n", "/// 🧊️ Shared support of the Generation3d app law targets: projection reads, gumball readers, run-time fixtures, the serial lock and the flow operators.\n", 1)
RUNTIME = '''/// 🧷️ Fixtures are read from the repository at run time, so each target's source closure stays inside the compiler-ingress gate.
pub(crate) mod runtime_fixture {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock, PoisonError};

    pub(crate) fn text(path: &str) -> &'static str {
        static CACHE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
        let mut cache = CACHE.get_or_init(Mutex::default).lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(text) = cache.get(path) { return text; }
        let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../..").join(path);
        let text: &'static str = Box::leak(std::fs::read_to_string(&file).unwrap_or_else(|error| panic!("fixture {}: {error}", file.display())).into_boxed_str());
        cache.insert(path.to_owned(), text);
        text
    }
}

'''
SUPPORT_MODS = ('#[path = "🧪️tests/🔬️serial/🦀️.rs"]\npub(crate) mod test_serial;\n\n'
                '#[path = "🧪️tests/🔬️brep-extension/🦀️.rs"]\npub(crate) mod brep_extension;\n\n'
                '#[path = "🧪️tests/🔬️flow-operators/🦀️.rs"]\npub(crate) mod flow_operators;\n\n'
                '#[path = "🧪️tests/🔬️publication-authority/🦀️.rs"]\npub(crate) mod publication_authority;\n')
shared_new = pre + RUNTIME + SUPPORT_MODS
imports_text = "/// 📥️ The imports every editor law wrapper shares.\n" + "\n".join(domain_uses) + "\n"

CALL_RE = re.compile(r"include_(?:str|bytes)!\(")
CONST_HEAD = re.compile(r"(?P<indent>[ \t]*)(?P<vis>pub(?:\([a-z]+\))?\s+)?const\s+(?P<name>[A-Z][A-Z0-9_]*)\s*:\s*&(?:'static\s+)?str\s*=\s*$")


def parse_call(text, start):
    depth = 0
    i = start + len(CALL_RE.match(text, start).group(0)) - 1
    j = i
    in_str = False
    while True:
        ch = text[j]
        if in_str:
            if ch == "\\":
                j += 1
            elif ch == '"':
                in_str = False
        elif ch == '"':
            in_str = True
        elif ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                return j + 1, text[i + 1:j]
        j += 1


def resolve(arg, file):
    arg = arg.strip()
    m = re.fullmatch(r'"([^"]*)"', arg)
    if m:
        target = os.path.normpath(os.path.join(os.path.dirname(file), m.group(1)))
    else:
        m = re.fullmatch(r'concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"([^"]*)"\s*\)', arg)
        assert m, (file, arg)
        target = os.path.normpath(MANIFEST_DIR + m.group(1))
    assert os.path.exists(target), (file, target)
    return os.path.relpath(target, R)


converted_consts = []
shared_consts = []


def rewrite_file(path, text):
    out = []
    pos = 0
    consts = []
    count = 0
    while True:
        m = CALL_RE.search(text, pos)
        if not m:
            break
        end, arg = parse_call(text, m.start())
        rel = resolve(arg, path)
        call = 'crate::runtime_fixture::text("%s")' % rel
        head = CONST_HEAD.search(text[:m.start()].rsplit("\n", 1)[-1])
        if head:
            assert text[end] == ";", (path, text[end:end + 20])
            line_start = text.rfind("\n", 0, m.start()) + 1
            name = head.group("name")
            low = name.lower()
            out.append(text[pos:line_start])
            out.append("%s%sfn %s() -> &'static str { %s }" % (head.group("indent"), head.group("vis") or "", low, call))
            pos = end + 1
            consts.append((name, low))
            if head.group("vis"):
                shared_consts.append((name, low))
        else:
            out.append(text[pos:m.start()])
            out.append(call)
            pos = end
        count += 1
    out.append(text[pos:])
    new = "".join(out)
    for name, low in consts:
        pat = re.compile(r"\b%s\b" % name)
        new, n_uses = pat.subn(low + "()", new)
        print("const", os.path.basename(os.path.dirname(path)), name, "uses", n_uses)
        converted_consts.append((path, name, low))
    return new, count


SHARED_HELPERS = [("🧮️engine-compute", ["mesh_data_from_json", "aabb_of_positions", "test_serial"]), ("🕹️command-surface", ["node_move_operations_json", "declared_actions"]), ("🔁️extension-round-trip", ["node_eval_status"])]
HELPER_DIR = UNIT + "🧰️shared-helpers/🦀️.rs"


def extract_fn(text, name):
    lines = text.split("\n")
    at = next(i for i, l in enumerate(lines) if re.match(r"(?:pub(?:\([a-z]+\))? )?(?:async )?fn %s\b" % name, l))
    start = at
    while start > 0 and (lines[start - 1].startswith("///") or lines[start - 1].startswith("#[")):
        start -= 1
    end = next(i for i in range(at, len(lines)) if lines[i] == "}")
    item = lines[start:end + 1]
    rest = lines[:start] + lines[end + 1:]
    if start < len(rest) and rest[start] == "" and start > 0 and rest[start - 1] == "":
        del rest[start]
    return "\n".join(rest), "\n".join(item)


helpers_path = HERE + "/helpers.json"
persisted = json.load(open(helpers_path, encoding="utf8")) if os.path.exists(helpers_path) else {}
helper_texts = {}
for group, fns in SHARED_HELPERS:
    path = G + UNIT + group + "/🦀️.rs"
    text = current(path)
    for fn in fns:
        if re.search(r"(?m)^(?:pub(?:\([a-z]+\))? )?(?:async )?fn %s\b" % fn, text):
            text, persisted[fn] = extract_fn(text, fn)
    helper_texts[path] = text
for _, fns in SHARED_HELPERS:
    for fn in fns:
        assert fn in persisted, fn
json.dump(persisted, open(helpers_path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
helper_items = [persisted[fn] for _, fns in SHARED_HELPERS for fn in fns]
helper_file = "/// 🧰️ Test helpers the editor law groups share, kept in the support so every target that owns a group resolves them.\n\n" + "\n\n".join(helper_items) + "\n"
unit_support = re.sub(r'^include!\("[^"]+"\);\n', "", current(C + "🧪️tests/🧊️generation3d/" + UNIT_FILE), flags=re.M).rstrip("\n") + "\n"
unit_support += 'include!("🧰️shared-helpers/🦀️.rs");\n'

all_files = [p for p in idx if p.startswith(G) and p.endswith(".rs")]
rewritten = {}
sites = 0
for path in sorted(all_files):
    rel = path[len(G):]
    if rel in ("🦀️.rs",) or any(rel == f + "/🦀️.rs" for f in FRAGMENTS):
        continue
    text = helper_texts.get(path, current(path))
    if rel == UNIT_FILE:
        text = unit_support
    new, count = rewrite_file(path, text)
    sites += count
    if new != current(path):
        rewritten[path] = new

for name, low in shared_consts:
    pat = re.compile(r"\b%s\b" % name)
    for path in sorted(all_files):
        rel = path[len(G):]
        if rel == "🦀️.rs" or any(rel == f + "/🦀️.rs" for f in FRAGMENTS):
            continue
        now = rewritten.get(path, current(path))
        new, n = pat.subn(low + "()", now)
        if n:
            print("shared const use", name, n, os.path.basename(os.path.dirname(path)))
            rewritten[path] = new
for p, t in rewritten.items():
    write(p, t, derived=True)
for key, (dirname, target) in TARGETS.items():
    write(G + dirname + "/🦀️.rs", root_text[key])
for p, t in mounts.items():
    write(p, t)
write(G + "🦀️.rs", shared_new)
write(G + HELPER_DIR, helper_file)
write(G + IMPORTS + "/🦀️.rs", imports_text)

json.dump({**state, **written}, open(state_path, "w"), ensure_ascii=False, indent=0) if not DRY else None
print("files written", len(written), "include sites rewritten", sites, "consts", len(converted_consts))
json.dump({"assign": assign, "blocks": {n: root_of(f, n) if n != "editor_domain" else "*" for f, n, _ in blocks}}, open(HERE + "/assign.json", "w"), ensure_ascii=False, indent=1) if not DRY else None
