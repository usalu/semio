#!/usr/bin/env python3
"""✂️ Shared helpers of the energy model artifact cut (lane P1, slice S2).

The artifact's crates name each other, the engine and the framework through crate-root aliases
(`extern crate <package> as <alias>;`); this module owns the alias registry, the anchored text surgery, the
path rewrites from the monolith's module tree to those aliases, crate-root alias blocks derived from what a crate's
closure names, and the manifests and workspace rows derived from the same facts.
"""
import json, os, re

REPO = "/Users/ueli/Documents/semio"
PLUGIN = f"{REPO}/✏️s/🔌️plugins/🔋️energy"
ART = f"{PLUGIN}/🗿️artifacts/🔋️model"
ANY_REL = "🏅️standards/🔖️1/🪆️subsets/✳️any"
ANY = f"{ART}/{ANY_REL}"
ENGINE = f"{PLUGIN}/🔨️modules/⚡️simulation/⚙️engine"
FW = f"{REPO}/🧰️framework"
DEEP = "standards::v1::subsets::any"

def rd(path): return open(path, encoding="utf-8").read()
def wr(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)

def take(text, start, end, include_end=True):
    """✂️ Removes the unique span `start … end` and returns (rest, span)."""
    assert text.count(start) == 1, f"anchor not unique ({text.count(start)}): {start[:90]!r}"
    i = text.index(start); j = text.index(end, i) + (len(end) if include_end else 0)
    return text[:i] + text[j:], text[i:j]

def take_item(text, start, end):
    """✂️ Like `take`, but the item's leading `///` docs and `#[…]` attributes travel with it."""
    assert text.count(start) == 1, f"anchor not unique ({text.count(start)}): {start[:90]!r}"
    i = text.index(start); j = text.index(end, i) + len(end)
    while True:
        k = text.rfind("\n", 0, i - 1) + 1
        line = text[k:i].strip()
        if i > 0 and (line.startswith("///") or line.startswith("#[")): i = k
        else: break
    return text[:i] + text[j:], text[i:j]

def replace_once(text, old, new):
    assert text.count(old) == 1, f"anchor not unique ({text.count(old)}): {old[:90]!r}"
    return text.replace(old, new)

ENGINE_MODULES = sorted(re.sub(r"^[^a-z_]+", "", d) for d in os.listdir(ENGINE) if os.path.isdir(os.path.join(ENGINE, d)))

# 🧭️ alias -> package for every crate an artifact crate root may alias
ALIASES = {m: "semio-s-plugin-energy-engine-" + m.replace("_", "-") for m in ENGINE_MODULES}
ALIASES.update({
    "sim_wire": "semio-s-plugin-energy-engine-sim-wire", "sim_retirement": "semio-s-plugin-energy-engine-sim-retirement",
    "sim_restore": "semio-s-plugin-energy-engine-sim-restore",
    "epw": "semio-s-plugin-energy-epw",
    "dsl": "semio-framework-os-kernel", "protocol": "semio-framework-os-kernel", "store": "semio-framework-os-kernel",
    "framework_schema": "semio-framework-schema",
    "snapshot": "semio-s-artifact-energy-model-snapshot", "diff": "semio-s-artifact-energy-model-diff",
    "diff_patch": "semio-s-artifact-energy-model-diff-patch", "diff_splice": "semio-s-artifact-energy-model-diff-splice",
    "schema": "semio-s-artifact-energy-model-schema", "inferences": "semio-s-artifact-energy-model-inferences",
    "mutations": "semio-s-artifact-energy-model-mutations", "fixtures": "semio-s-artifact-energy-model-mutation-fixtures",
    "io": "semio-s-artifact-energy-model-io", "import": "semio-s-artifact-energy-model-import", "export": "semio-s-artifact-energy-model-export",
    "session": "semio-s-artifact-energy-model-session", "examples": "semio-s-artifact-energy-model-examples",
    "editor_commands": "semio-s-artifact-energy-model-editor-commands", "editor_config": "semio-s-artifact-energy-model-editor-config",
    "editor_results": "semio-s-artifact-energy-model-editor-results", "editor_interaction": "semio-s-artifact-energy-model-editor-interaction",
    "editor_edit": "semio-s-artifact-energy-model-editor-edit", "editor_panels": "semio-s-artifact-energy-model-editor-panels",
    "editor": "semio-s-artifact-energy-model-editor", "viewer": "semio-s-artifact-energy-model-viewer",
})
FRAMEWORK_ALIAS_CRATES = {"dsl", "protocol", "store", "framework_schema"}
# 🧱️ alias the energy crates under, in crate-root order
ROOT_ALIAS_ORDER = ["dsl", "protocol", "store", "framework_schema"]

def domain_alias(slug): return slug.replace("-", "_") + "_mutations"
def domain_package(slug): return "semio-s-artifact-energy-model-mut-" + slug

def load_domains():
    cfg = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🧬️mutation-domains.json"), encoding="utf-8"))
    return cfg

for _emoji, _slug, _nouns, _kinds in load_domains()["domains"]:
    ALIASES[domain_alias(_slug)] = domain_package(_slug)

# 📦️ framework packages by crate ident, with their path from the repository root
FRAMEWORK_PATHS = {
    "semio-framework": "🧰️framework/📦️packages/🦀️rust",
    "semio-framework-2d": "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust",
    "semio-framework-artifact-reference": "🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust",
    "semio-framework-async": "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
    "semio-framework-async-macros": "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
    "semio-framework-diagnostic": "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
    "semio-framework-dispatch-macros": "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust",
    "semio-framework-dsl": "🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust",
    "semio-framework-dsl-record": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
    "semio-framework-dsl-record-derive": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "semio-framework-geometry": "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust",
    "semio-framework-job": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
    "semio-framework-os-kernel": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
    "semio-framework-pack": "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust",
    "semio-framework-pack-json": "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
    "semio-framework-plugin": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
    "semio-framework-schema": "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust",
    "semio-framework-schema-composition": "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust",
    "semio-framework-schema-registry": "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",
    "semio-framework-schema-state": "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust",
    "semio-framework-tool-run": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
    "semio-framework-ui-contract": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
    "semio-framework-ui-locale": "🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust",
    "semio-framework-value": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
    "semio-framework-value-derive": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
}
# 🏷️ crates referenced under a renamed dependency key
RENAMED = {"pack": "semio-framework-pack"}
EXTERNAL = {"serde": '{ workspace = true }', "serde_json": '{ workspace = true }'}

def strip(src):
    """🧹️ Blanks comments and string/char literals, keeping every offset."""
    out, i, n = [], 0, len(src)
    blank = lambda t: re.sub(r"[^\n]", " ", t)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j; out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if src.startswith("/*", j): d, j = d + 1, j + 2
                elif src.startswith("*/", j): d, j = d - 1, j + 2
                else: j += 1
            out.append(blank(src[i:j])); i = j
        elif c == "r" and re.match(r'r#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'r(#*)"', src[i:]); close = '"' + m.group(1); j = src.find(close, i + len(m.group(0))); j = n if j < 0 else j + len(close)
            out.append(blank(src[i:j])); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == "\\" else 1
            j += 1; out.append(blank(src[i:j])); i = j
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]):
            m = re.match(r"'(\\.|[^\\'])'", src[i:]); out.append(" " * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(c); i += 1
    return "".join(out)

def rewrite_code(text, rules):
    """🔁️ Applies `(regex, replacement)` rules to code only (comments and literals untouched)."""
    for pattern, repl in rules:
        s = strip(text); edits = []
        for m in re.finditer(pattern, s):
            edits.append((m.start(), m.end(), m.expand(repl)))
        for a, b, r in reversed(edits):
            text = text[:a] + r + text[b:]
    return text

MOD_TOKEN = re.compile(r"(?P<attrs>(?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?(?<![\w])mod\s+(?P<name>\w+)\s*(?P<end>[;{])|(?P<brace>[{}])")

def closure_files(root_file):
    """📚️ Every `.rs` file mounted from a crate root, following nested mounts with rustc's rule for `#[path]` inside inline
    module blocks (relative to the file directory plus each enclosing inline module's directory component).
    https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute"""
    seen, stack = [], [os.path.normpath(root_file)]
    while stack:
        f = stack.pop()
        if f in seen or not os.path.exists(f): continue
        seen.append(f)
        raw = rd(f); s = strip(raw)
        dirs, marks, depth = [os.path.dirname(f)], [], 0
        for m in MOD_TOKEN.finditer(s):
            if m.group("brace") == "{": depth += 1; continue
            if m.group("brace") == "}":
                if marks and marks[-1] == depth: marks.pop(); dirs.pop()
                depth -= 1; continue
            path = re.search(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', raw[m.start("attrs"):m.end("attrs")])
            path = path.group(1) if path else None
            if m.group("end") == "{":
                depth += 1; marks.append(depth); dirs.append(os.path.normpath(os.path.join(dirs[-1], path if path is not None else m.group("name"))))
            elif path is not None:
                if path != ".": stack.append(os.path.normpath(os.path.join(dirs[-1], path)))
            else:
                for cand in (os.path.join(dirs[-1], m.group("name") + ".rs"), os.path.join(dirs[-1], m.group("name"), "mod.rs")):
                    if os.path.exists(cand): stack.append(os.path.normpath(cand)); break
    return seen

def aliases_used(files):
    """🔎️ Aliases a crate's files name: `crate::<alias>::` paths and bare `<alias>::` roots of framework aliases."""
    used = set()
    for f in files:
        s = strip(rd(f))
        for a in re.findall(r"\bcrate::([a-z_][a-z0-9_]*)::", s):
            if a in ALIASES: used.add(a)
        for a in re.findall(r"(?<![\w:])([a-z_][a-z0-9_]*)::", s):
            if a in FRAMEWORK_ALIAS_CRATES or a == "pack": used.add(a)
    return used

def crates_named(files):
    """📦️ Framework/stdio crates named directly (`semio_framework_x::`, `semio_s_artifact_stdio_x::`) plus derive implications."""
    named = set()
    for f in files:
        s = strip(rd(f))
        for ident in re.findall(r"(?<![\w:])(semio_(?:framework|s_artifact_stdio)[a-z0-9_]*)::", s):
            named.add(ident.replace("_", "-"))
        for ident in re.findall(r"(?<![\w:])(serde_json|serde)::", s):
            named.add(ident)
        if re.search(r"\bSerialize\b|\bDeserialize\b", s): named.add("serde")
        if "canonical_json" in s: named.add("semio-framework-pack-json")
        if "ToValueDerive" in s or "FromValueDerive" in s: named.update({"semio-framework-value", "semio-framework-value-derive"})
        if "dsl_record_derive" in s: named.add("semio-framework-dsl-record")
    return named

def alias_block(used, own_alias=None):
    """🧱️ The crate-root alias lines for the aliases a crate uses."""
    lines = []
    order = [a for a in ROOT_ALIAS_ORDER if a in used] + sorted(a for a in used if a not in ROOT_ALIAS_ORDER and a != "pack" and a != own_alias)
    for a in order:
        lines.append(f"extern crate {ALIASES[a].replace('-', '_')} as {a};")
    return "\n".join(lines)

def manifest(owner_dir, package, description, deps, dev_deps=(), dev_features=None, tests=()):
    """📦️ `<owner>/📦️packages/🦀️rust/Cargo.toml` of one artifact crate (workspace = the artifact workspace)."""
    pkg_dir = os.path.join(owner_dir, "📦️packages", "🦀️rust")
    ws = os.path.relpath(ART, pkg_dir)
    def row(dep, dev=False):
        feat = (dev_features or {}).get(dep) if dev else None
        key = next((k for k, v in RENAMED.items() if v == dep), None)
        name = key or dep
        extra = f', features = {json.dumps(feat)}' if feat else ""
        return f"{name} = {{ workspace = true{extra} }}"
    lines = ["[package]", f'workspace = "{ws}"', f'name = "{package}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true",
             f'description = "{description}"', "", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"']
    for name, path in tests:
        lines += ["", "[[test]]", f'name = "{name}"', f'path = "{path}"']
    lines += ["", "[dependencies]", *sorted(row(d) for d in deps)]
    if dev_deps: lines += ["", "[dev-dependencies]", *sorted(row(d, True) for d in dev_deps)]
    wr(os.path.join(pkg_dir, "Cargo.toml"), "\n".join(lines) + "\n")
    return os.path.join(pkg_dir, "Cargo.toml")

def deps_for(files, own_package, local=()):
    """🧮️ Packages a crate's files need: aliased crates plus directly named framework/stdio crates; `local` names the
    crate root's own modules, whose `crate::<name>::` paths are not aliases."""
    used = aliases_used(files) - set(local)
    deps = {ALIASES[a] if a in ALIASES else RENAMED[a] for a in used}
    deps |= crates_named(files)
    deps.discard(own_package)
    return used, deps

def ensure_workspace_rows(packages):
    """🧾️ Adds the artifact workspace `[workspace.dependencies.<pkg>]` rows a new crate inherits."""
    ws_p = f"{ART}/Cargo.toml"; w = rd(ws_p); added = []
    for pkg in sorted(packages):
        key = next((k for k, v in RENAMED.items() if v == pkg), pkg)
        if f"[workspace.dependencies.{key}]" in w: continue
        if pkg in FRAMEWORK_PATHS:
            row = f'[workspace.dependencies.{key}]\npath = "{os.path.relpath(os.path.join(REPO, FRAMEWORK_PATHS[pkg]), ART)}"\n' + (f'package = "{pkg}"\n' if key != pkg else "")
        elif pkg.startswith("semio-s-artifact-energy-model"):
            continue
        else:
            raise SystemExit(f"no workspace row source for {pkg}")
        anchor = "[workspace.dependencies.serde]\n"
        w = w.replace(anchor, row + "\n" + anchor, 1); added.append(pkg)
    wr(ws_p, w)
    return added

def add_energy_workspace_row(package, owner_dir):
    ws_p = f"{ART}/Cargo.toml"; w = rd(ws_p)
    if f"[workspace.dependencies.{package}]" in w: return
    row = f'[workspace.dependencies.{package}]\npath = "{os.path.relpath(os.path.join(owner_dir, "📦️packages", "🦀️rust"), ART)}"\n'
    anchor = "[workspace.dependencies.serde]\n"
    wr(ws_p, w.replace(anchor, row + "\n" + anchor, 1))

def add_workspace_member(owner_dir):
    ws_p = f"{ART}/Cargo.toml"; w = rd(ws_p)
    member = os.path.relpath(os.path.join(owner_dir, "📦️packages", "🦀️rust"), ART)
    m = re.search(r'members = \[([^\]]*)\]', w)
    items = [x.strip().strip('"') for x in m.group(1).split(",") if x.strip()]
    if member in items: return
    items = sorted(set(items + [member]))
    w = w[:m.start()] + "members = [\n" + "".join(f'  "{x}",\n' for x in items) + "]" + w[m.end():]
    wr(ws_p, w)

def engine_index():
    """🗂️ Every public top-level engine item name -> the alias of the engine crate that owns it."""
    roots = {m: f"{ENGINE}/{d}/🦀️.rs" for d in os.listdir(ENGINE) if os.path.isdir(os.path.join(ENGINE, d)) for m in [re.sub(r"^[^a-z_]+", "", d)]}
    roots.update({"sim_wire": f"{ENGINE}/🧪️sim/📡️wire/🦀️.rs", "sim_retirement": f"{ENGINE}/🧪️sim/🪜️retirement/🦀️.rs", "sim_restore": f"{ENGINE}/🧪️sim/♻️restore/🦀️.rs"})
    index = {}
    for alias, root in sorted(roots.items()):
        for name in re.findall(r"(?m)^pub (?:const fn|async fn|fn|struct|enum|const|static|type|trait) ([A-Za-z_][A-Za-z0-9_]*)", rd(root)):
            index.setdefault(name, alias)
    return index
