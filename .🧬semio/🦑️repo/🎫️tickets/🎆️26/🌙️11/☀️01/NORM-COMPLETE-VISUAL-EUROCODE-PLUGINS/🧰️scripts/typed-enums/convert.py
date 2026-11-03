#!/usr/bin/env python3
"""🧬 Typed-enum generator: turns one `String` field of a norm artifact into a schema enum in every twin, codec, leaf, editor list and fixture.

usage:
  python3 -I convert.py ARTIFACT HOST.field[,HOST2.field2] ENUM VARIANT... [options]
  python3 -I convert.py --spec specs/ARTIFACT.json [--only ENUM] [options]     (list of conversions; the specs of the runs are committed next to this file)

VARIANT is `Name|legacy1,legacy2|English label|Deutsches Label`: `Name` is the Rust variant (the wire token is the variant name, lower-cased first letter when the artifact's enums are camelCase), the legacy tokens are
the old `String` values (fixtures, assets, literals, python oracles) that map to it, the labels end up in the schema `$defs` options and the editor choice list.
A spec conversion is `{"hosts": ["Struct.field", …], "enum": …, "doc": …, "default": "Variant", "variants": [[name, [legacy…], en, de], …]}`; `default` takes bogus fixture values and the alternate value of an apply scenario
(`x-alt`, `changed-x`: the committed `before` snapshot then gets another variant so the mutation still changes something).
Catalogue mode: add `"catalogue": "<name>"` (JSON schema `format: "catalogue-id"` and `x-semio-catalogue`, plus `id()`, `from_id()` and `label(german)` on the Rust enum, string consumers `&x.field`/`x.field.as_str()` become `x.field.id()`).
`"wire": "camel"` (conversion or spec level) fixes the wire spelling when the artifact has no enum to copy it from (camel: lower-case first letter, so legacy tokens such as `building` stay valid and no fixture changes).
`"from_const": "NAME"` reads the variants once from an editor choice list; paste them into the spec afterwards, a second run would read the already rewritten list.

options:
  --root DIR      the `…/🪆️subsets/✳️any` directory to edit (default: the real artifact; point it at a scratch copy to prove a run)
  --apply         write the rewrites (without --apply or --check a run only lists the files it would change, like --dry)
  --check         edit nothing, list every site that is not converted yet, exit 1 when there is one (a second run must exit 0)
  --dry           edit nothing, list the files a run would change
  --doc TEXT      docstring of the enum (after the emoji)       --default NAME   variant for unmapped fixture values
  --only ENUM     run one conversion of a spec

Re-run safety: a value that already is a variant (wire token, DSL kebab token or variant name) is never rewritten, in fixtures, assets, oracles and leaf payloads alike; where the legacy map disagrees the run prints `lossy:` and keeps the value
(`--check` exits 1 on it). Only `String`-shaped legacy values reach the `default`. `from_const` refuses to run once the enum exists.

Idempotent and additive: every rule only rewrites text that still has the `String` shape, so a second run changes nothing. Not covered (the run prints them as `residual:`/`unmapped:`/`manual:`): `&str` parameters of helper functions,
Display uses, validations of unknown values that become unrepresentable, tests that feed invalid values. The `en*_manual.py` scripts next to this file are the hand edits of the runs done so far.
"""
import json, os, re, sys

SEMIO = os.environ.get("SEMIO_ROOT", "/home/deck/semio")
NORM = SEMIO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
EMOJI = {"en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️", "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️"}
SKIP_DIRS = {"target", "node_modules", ".git", "🗑️generated", "__pycache__"}
TEXT_EXT = (".rs", ".ts", ".json", ".graphql", ".proto", ".sql", ".py", ".semio", ".feature", ".md", ".toml")
FIXTURE = "🧫️fixtures"


def read_text(p):
    try:
        return open(p, encoding="utf-8", newline="").read()
    except (UnicodeDecodeError, OSError):
        return None


def artifact_root(short):
    return f"{NORM}/{EMOJI[short]}{short}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def split_words(name):
    return re.findall(r"[A-Z]+(?![a-z])|[A-Z]?[a-z]+|\d+", name.replace("-", "_").replace("_", " ")) or [name]


def snake(name):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", name).replace("-", "_").lower()


def camel(name):
    parts = name.split("_")
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:])


def pascal(name):
    return "".join(p[:1].upper() + p[1:] for p in name.replace("-", "_").split("_"))


def kebab(name):
    out = []
    for i, c in enumerate(name):
        if c in "_-":
            if out and out[-1] != "-":
                out.append("-")
            continue
        if c.isupper():
            prev = name[i - 1] if i else ""
            nxt = name[i + 1] if i + 1 < len(name) else ""
            boundary = prev.islower() or prev.isdigit() or (prev.isupper() and nxt.islower())
            if boundary and out and out[-1] != "-":
                out.append("-")
            out.append(c.lower())
        else:
            out.append(c)
    return "".join(out)


class Variant:
    def __init__(self, name, legacy, en, de):
        self.name, self.legacy, self.en, self.de = name, legacy, en, de

    @staticmethod
    def parse(text):
        parts = text.split("|")
        assert len(parts) == 4, f"variant needs Name|legacy,legacy|EN|DE: {text!r}"
        return Variant(parts[0], [x for x in parts[1].split(",") if x != ""] + ([""] if parts[1].endswith(",") or parts[1] == "<empty>" else []), parts[2], parts[3])


class Conversion:
    def __init__(self, artifact, hosts, enum, variants, doc="", default=None, remove=False, catalogue=None, wire=None):
        self.artifact, self.enum, self.variants, self.doc, self.default, self.remove, self.catalogue, self.wire = artifact, enum, variants, doc, default, remove, catalogue, wire
        self.hosts = [tuple(h.split(".")) for h in hosts]
        self.fields = sorted({f for _, f in self.hosts})
        assert len(self.fields) == 1 or remove or True
        self.legacy = {}
        for v in variants:
            for token in v.legacy + [v.name]:
                self.legacy[token] = v.name
        self.token_re = None

    def name_forms(self, field):
        return dict(s=field, c=camel(field), k=kebab(field))


class Ctx:
    def __init__(self, root, conv, dry=False, check=False):
        self.root, self.conv, self.dry, self.check = root, conv, dry, check
        self.files = []
        for base, dirs, names in os.walk(root):
            dirs[:] = sorted(d for d in dirs if d not in SKIP_DIRS)
            for n in sorted(names):
                self.files.append(os.path.join(base, n))
        self.changed, self.todo, self.log = {}, [], []
        self.snapshot = self.read_json("/🧬️schema/📸️snapshot/🔣️.json")
        self.artifact_json = self.read_json("/🧬️schema/🔣️.json")
        self.diff_json = self.read_json("/🧬️schema/🔺️diff/🔣️.json")
        self.schema_id = self.snapshot["$id"] if self.snapshot else ""
        self.model_crate = self.find_model_crate()

    def read_json(self, rel):
        p = self.root + rel
        return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else None

    def rel(self, p):
        return os.path.relpath(p, self.root)

    def find_model_crate(self):
        p = self.root + "/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml"
        if os.path.exists(p):
            m = re.search(r'^name\s*=\s*"([^"]+)"', open(p, encoding="utf-8").read(), re.M)
            if m:
                return m.group(1).replace("-", "_")
        return None

    def note(self, kind, path, text):
        self.todo.append(f"{kind}: {self.rel(path)}: {text}")


# ---------------------------------------------------------------- text scanning

def scan_strings(text, py=False):
    """Mask of string/comment characters so brace matching ignores them: returns a list of (start, end) string spans."""
    spans, i, n = [], 0, len(text)
    while i < n:
        c = text[i]
        if c == '"' or (py and c == "'"):
            j = i + 1
            while j < n and text[j] != c:
                j += 2 if text[j] == "\\" else 1
            spans.append((i, j + 1))
            i = j + 1
        elif c == "/" and text[i + 1:i + 2] == "/":
            j = text.find("\n", i)
            j = n if j < 0 else j
            spans.append((i, j))
            i = j
        elif py and c == "#":
            j = text.find("\n", i)
            j = n if j < 0 else j
            spans.append((i, j))
            i = j
        else:
            i += 1
    return spans


def brace_pairs(text, py=False, open_c="{", close_c="}"):
    """All matching bracket pairs outside strings: list of (open_index, close_index)."""
    masked = list(text)
    for a, b in scan_strings(text, py):
        for k in range(a, min(b, len(text))):
            if masked[k] not in "\n":
                masked[k] = " "
    stack, pairs = [], []
    for i, c in enumerate(masked):
        if c == open_c:
            stack.append(i)
        elif c == close_c and stack:
            pairs.append((stack.pop(), i))
    return sorted(pairs), "".join(masked)


def direct_text(text, pairs, a, b):
    """Text of the pair (a, b) with nested pairs blanked out."""
    inner = [(x, y) for x, y in pairs if a < x and y < b]
    top, last = [], -1
    for x, y in inner:
        if x > last:
            top.append((x, y))
            last = y
    out, pos = [" "], a + 1
    for x, y in top:
        out.append(text[pos:x])
        out.append(" " * (y - x + 1))
        pos = y + 1
    out.append(text[pos:b])
    return "".join(out)


def struct_body(text, header_re, start=0):
    """Index range (open+1, close) of the `{ … }` that follows the first match of header_re at or after start, or None."""
    m = re.compile(header_re).search(text, start)
    if not m:
        return None
    o = text.find("{", m.end() - 1 if text[m.end() - 1] == "{" else m.end())
    if o < 0:
        return None
    pairs, _ = brace_pairs(text)
    for a, b in pairs:
        if a == o:
            return a + 1, b
    return None


# ---------------------------------------------------------------- object-literal matching (JSON, python dicts, DSL groups)

KEY_JSON = re.compile(r"""["']([A-Za-z_][\w-]*)["']\s*:""")
KEY_DSL = re.compile(r"(?<![\w=-])([a-z][a-z0-9]*(?:-[a-z0-9]+)*)=")


def json_value_sites(text, keys, siblings, py=False, min_overlap=3):
    """Yield (start, end, key, raw_value) of string values of any key in `keys` inside an object whose direct keys overlap `siblings`."""
    if not any(k in text for k in keys):
        return
    pairs, _ = brace_pairs(text, py)
    for a, b in pairs:
        d = direct_text(text, pairs, a, b)
        present = {m.group(1) for m in KEY_JSON.finditer(d)}
        hit = present & set(keys)
        if not hit or len(present & siblings) < min(min_overlap, len(siblings)):
            continue
        for k in hit:
            for m in re.finditer(r"""["']%s["']\s*:\s*(["'])((?:[^"'\\]|\\.)*)\1""" % re.escape(k), d):
                yield a + m.start(1), a + m.end(), k, m.group(2)


def dsl_value_sites(text, keys, siblings, min_overlap=3):
    """Yield (start, end, key, raw_value) of `key=value` pairs inside a `{…}` group whose keys overlap `siblings`."""
    if not any(k + "=" in text for k in keys):
        return
    pairs, _ = brace_pairs(text)
    for a, b in pairs:
        d = direct_text(text, pairs, a, b)
        present = {m.group(1) for m in KEY_DSL.finditer(d)}
        hit = present & set(keys)
        if not hit or len(present & siblings) < min(min_overlap, len(siblings)):
            continue
        for k in hit:
            for m in re.finditer(r'(?<![\w=-])%s=("(?:[^"\\]|\\.)*"|[^\s}\]]+)' % re.escape(k), d):
                yield a + m.start(1), a + m.end(1), k, m.group(1)


# ---------------------------------------------------------------- schema introspection

def host_props(ctx, host):
    """Property names (wire camelCase) of a host struct in the snapshot schema; the root struct has its properties at the top."""
    s = ctx.snapshot
    if s is None:
        return None
    if s.get("title") == host:
        return list(s["properties"])
    d = s.get("$defs", {}).get(host)
    return list(d["properties"]) if d and "properties" in d else None


def host_ref(ctx, host, camel_field):
    suffix = f"#/properties/{camel_field}" if ctx.snapshot.get("title") == host else f"#/$defs/{host}/properties/{camel_field}"
    return suffix


def patch_fields(ctx, host, camel_field):
    """{patch type: [wire patch field…]}: diff schema properties that reference the host property."""
    out = {}
    d = ctx.diff_json
    if not d:
        return out
    ref = host_ref(ctx, host, camel_field)

    def refs(node):
        if isinstance(node, dict):
            r = node.get("$ref")
            if isinstance(r, str) and r.endswith(ref):
                return True
            return any(refs(v) for v in node.values())
        if isinstance(node, list):
            return any(refs(v) for v in node)
        return False

    for name, df in [(d.get("title", "Diff"), d)] + list(d.get("$defs", {}).items()):
        for pname, pval in (df.get("properties") or {}).items():
            if refs(pval):
                out.setdefault(name, []).append(pname)
    for name in inline_patch_types(ctx, host):
        df = d.get("$defs", {}).get(name) or (d if d.get("title") == name else None)
        pval = ((df or {}).get("properties") or {}).get(camel_field)
        if pval is not None and name not in out and pval == INLINE_STRING_PATCH:
            out.setdefault(name, []).append(camel_field)
    return out


INLINE_STRING_PATCH = {"anyOf": [{"type": "string"}, {"type": "null"}]}


def inline_patch_types(ctx, host):
    """Patch types of `host` declared by the diff crate's `Patch of …::Host { set { … } }` macro (diff schemas that spell a patch field as plain `string | null` instead of a `$ref` of the snapshot property)."""
    t = read_text(ctx.root + "/🧬️schema/🔺️diff/🦀️.rs") or ""
    return re.findall(r"pub\s+(\w+)\s+of\s+[\w:]*::%s\s*\{\s*set\b" % re.escape(host), t)


def patch_props(ctx, ptype):
    d = ctx.diff_json
    if d.get("title") == ptype:
        return set(d["properties"])
    return set(d["$defs"][ptype]["properties"])


class Site:
    """One conversion target: host struct, field in all spellings, patch types and the sibling keys that identify its objects."""

    def __init__(self, ctx, host, field):
        self.host, self.field = host, field
        self.c, self.k = camel(field), kebab(field)
        props = host_props(ctx, host)
        self.props = set(props or [])
        self.siblings = self.props - {self.c, "id"}
        self.patches = patch_fields(ctx, host, self.c)
        self.patch_names = {n for ns in self.patches.values() for n in ns}
        self.patch_sets = {p: patch_props(ctx, p) for p in self.patches}


def collect_values(ctx, site):
    """Census: {value: count} of the string values of the field in every object literal of every text file, split by kind of file."""
    out = {}
    keys = {site.c} | site.patch_names
    for p in ctx.files:
        if not p.endswith((".json", ".py", ".semio", ".ts")):
            continue
        t = read_text(p)
        if t is None:
            continue
        kind = "fixture" if FIXTURE in p else "asset" if p.endswith(".semio") else "code"
        if p.endswith(".semio"):
            for a, b, k, v in dsl_value_sites(t, {site.k}, {kebab(x) for x in site.siblings}):
                out.setdefault((kind, v.strip('"')), 0)
                out[(kind, v.strip('"'))] += 1
            continue
        for a, b, k, v in json_value_sites(t, keys, site.siblings | site.patch_names, p.endswith(".py")):
            out.setdefault((kind, v), 0)
            out[(kind, v)] += 1
    return out


def rs_struct_fields(ctx, host):
    """Rust field names of the model struct (the wire names cannot be turned back into them: `fKN` is `f_k_n`)."""
    for rel in ("/🧬️schema/📸️snapshot/🦀️.rs", "/🧬️schema/🦀️.rs"):
        text = read_text(ctx.root + rel)
        if text:
            rng = struct_body(text, r"pub struct %s\b[^{;]*\{" % re.escape(host))
            if rng:
                return set(re.findall(r"\bpub\s+(\w+)\s*:", text[rng[0]:rng[1]]))
    return set()


# ---------------------------------------------------------------- conversion plan

class Plan:
    """Everything the rules need: names, wire tokens, sites per host, model crate, leaves."""

    def __init__(self, ctx, conv):
        self.ctx, self.conv = ctx, conv
        self.E = conv.enum
        self.e_snake = snake(conv.enum)
        self.e_upper = self.e_snake.upper()
        self.camel_wire = self.detect_camel_wire()
        self.sites = []
        for host, field in conv.hosts:
            facets = []
            for js in (ctx.snapshot, ctx.artifact_json):
                if js and (js.get("title") == host or host in js.get("$defs", {})):
                    facets.append(js)
            props = set()
            for js in facets:
                d = js if js.get("title") == host else js["$defs"][host]
                props |= set(d["properties"])
            site = Site.__new__(Site)
            site.host, site.field, site.c, site.k = host, field, camel(field), kebab(field)
            site.props = props
            site.siblings = props - {site.c, "id"}
            site.rs_fields = rs_struct_fields(ctx, host)
            site.patches = patch_fields(ctx, host, site.c) if ctx.snapshot and ctx.snapshot.get("title") == host or (ctx.snapshot and host in ctx.snapshot.get("$defs", {})) else {}
            site.patch_names = {n for ns in site.patches.values() for n in ns}
            site.patch_sets = {p: patch_props(ctx, p) for p in site.patches}
            self.sites.append(site)
        self.ambiguous = False
        for s in self.sites:
            owners = sum(1 for js in [ctx.snapshot] if js for d in list(js.get("$defs", {}).values()) + [js] if s.c in (d.get("properties") or {}))
            self.ambiguous = self.ambiguous or owners > len(conv.hosts)
        self.tokens = {v.name: self.wire(v.name) for v in conv.variants}
        self.leaves = self.find_leaves()

    def detect_camel_wire(self):
        if self.conv.wire:
            return self.conv.wire == "camel"
        s = self.ctx.snapshot
        toks = [t for k, d in (s or {}).get("$defs", {}).items() if "enum" in d and k != "AnnexChoice" for t in d["enum"]]
        return bool(toks) and all(t[:1].islower() for t in toks)

    def wire(self, name):
        return name[:1].lower() + name[1:] if self.camel_wire else name

    def qual(self, path):
        """The enum as written in a Rust file: bare inside the model snapshot file, fully qualified elsewhere."""
        if self.in_model(path) or not self.ctx.model_crate:
            return self.E
        return f"{self.ctx.model_crate}::{self.E}"

    def in_model(self, path):
        return self.ctx.rel(path) in ("🧬️schema/📸️snapshot/🦀️.rs",)

    def current_variant(self, token):
        """Variant a value already is (exact wire token, DSL kebab token or variant name); such a value is converted data and is never rewritten."""
        for v in self.conv.variants:
            if token in (self.tokens[v.name], v.name, kebab(v.name)):
                return v.name
        return None

    def map_value(self, token, alt_of=None):
        """Variant for a legacy token; None when unknown."""
        c = self.conv
        if token in c.legacy:
            return c.legacy[token]
        m = re.fullmatch(r"(.+?)[-_ ]alt", token) or re.fullmatch(r"changed-(.+)", token)
        if m and m.group(1) in c.legacy:
            return c.legacy[m.group(1)]
        return None

    def find_leaves(self):
        """{leaf dir: new field name}: leaf diffs that write the host property through a patch field with `payload.new_x`."""
        out = {}
        base = self.ctx.root + "/🧬️schema/🧬️mutations"
        if not os.path.isdir(base):
            return out
        for d in sorted(os.listdir(base)):
            p = f"{base}/{d}/🔺️diff/🦀️.rs"
            t = read_text(p) if os.path.exists(p) else None
            if not t:
                continue
            for site in self.sites:
                for ptype, names in site.patches.items():
                    if ptype not in t:
                        continue
                    for n in names:
                        m = re.search(r"\b%s\s*\{[^{}]*?\b%s:\s*Some\(payload\.(new_\w+?)(?:\.clone\(\))?\)" % (re.escape(ptype), re.escape(snake(n))), t)
                        if m:
                            out[f"{base}/{d}"] = m.group(1)
        return out


# ---------------------------------------------------------------- shared helpers for rules

LIT = r'"((?:[^"\\]|\\.)*)"'


def unescape(s):
    return s.replace('\\"', '"').replace("\\\\", "\\")


def dump_indent(o):
    return json.dumps(o, indent=2, ensure_ascii=False) + "\n"


def _c(v):
    return json.dumps(v, separators=(",", ":"), ensure_ascii=False)


def dump_compact(d):
    rows = []
    for k, v in d.items():
        if k in ("$defs", "definitions") and isinstance(v, dict):
            body = ",\n".join(f"    {_c(dk)}:{_c(dv)}" for dk, dv in v.items())
            rows.append(f"  {_c(k)}:{{\n{body}\n  }}")
        else:
            rows.append(f"  {_c(k)}:{_c(v)}")
    return "{\n" + ",\n".join(rows) + "\n}\n"


def dump_spaced(d):
    rows = []
    for k, v in d.items():
        if k in ("$defs", "definitions") and isinstance(v, dict):
            body = ",\n".join(f"  {json.dumps(dk, ensure_ascii=False)}: {json.dumps(dv, ensure_ascii=False)}" for dk, dv in v.items())
            rows.append(f" {json.dumps(k)}: {{\n{body}\n }}")
        else:
            rows.append(f" {json.dumps(k, ensure_ascii=False)}: {json.dumps(v, ensure_ascii=False)}")
    return "{\n" + ",\n".join(rows) + "\n}\n"


def dump_hybrid(o):
    """One-space indent; the root, its `properties`/`$defs` containers, every definition and every definition's `properties` are expanded, everything else is inline."""
    def inl(v):
        return json.dumps(v, ensure_ascii=False)

    def body(d, depth, container_of):
        pad = " " * depth
        rows = []
        for k, v in d.items():
            if isinstance(v, dict) and container_of(k, v):
                rows.append(f"{pad}{inl(k)}: {{\n{container_rows(k, v, depth + 1)}\n{pad}}}")
            else:
                rows.append(f"{pad}{inl(k)}: {inl(v)}")
        return ",\n".join(rows)

    def container_rows(name, v, depth):
        pad = " " * depth
        rows = []
        for k, c in v.items():
            if name == "$defs" and isinstance(c, dict):
                rows.append(f"{pad}{inl(k)}: {{\n{body(c, depth + 1, lambda kk, vv: kk == 'properties')}\n{pad}}}")
            else:
                rows.append(f"{pad}{inl(k)}: {inl(c)}")
        return ",\n".join(rows)

    return "{\n" + body(o, 1, lambda k, v: k in ("properties", "$defs")) + "\n}\n"


def dump_indent_ascii(o):
    return json.dumps(o, indent=2, ensure_ascii=True) + "\n"


def json_style(t, obj):
    for style in (dump_indent, dump_indent_ascii, dump_compact, dump_spaced, dump_hybrid):
        if t == style(obj):
            return style
    return None


def enum_def(plan, template):
    d = {"title": plan.E, "type": "string", "enum": [plan.tokens[v.name] for v in plan.conv.variants]}
    d["x-semio-ui"] = {"options": {plan.tokens[v.name]: {"en": v.en, "de": v.de} for v in plan.conv.variants}}
    if template and "x-semio-formats" in template:
        d["x-semio-formats"] = template["x-semio-formats"]
    if plan.conv.catalogue:
        d["format"] = "catalogue-id"
        d["x-semio-catalogue"] = plan.conv.catalogue
    return d


def template_def(obj):
    for k, d in obj.get("$defs", {}).items():
        if "enum" in d and k != "AnnexChoice" and "x-semio-formats" in d:
            return d
    return None


def add_import_names(t, module_re, names):
    """Add named imports to the first `import { … } from "<module>"` whose module path matches module_re; returns text."""
    m = re.search(r"import \{([^}]*)\} from (\"%s\");" % module_re, t)
    if not m:
        return t, False
    have = [x.strip() for x in m.group(1).split(",") if x.strip()]
    add = [n for n in names if n not in have]
    if not add:
        return t, True
    body = " " + ", ".join(add + have) + " "
    return t[:m.start(1)] + body + t[m.end(1):], True


def ensure_after_imports(t, line):
    """Insert a line after the last top-of-file `import …;` / `use …;` block line."""
    ms = list(re.finditer(r"^(?:import|use) [^\n]*;\n", t, re.M))
    if not ms:
        return line + "\n" + t
    pos = ms[-1].end()
    return t[:pos] + line + "\n" + t[pos:]


# ---------------------------------------------------------------- twin rules (schema json, ts, graphql, proto)

SNAPSHOT_FACETS = ("🧬️schema/📸️snapshot", "🧬️schema")


def facet_of(ctx, p):
    r = ctx.rel(p)
    for f in SNAPSHOT_FACETS:
        if os.path.dirname(r) == f:
            return f
    return None


def rule_diff_json(ctx, plan, p, t):
    obj = json.loads(t)
    style = json_style(t, obj)
    changed = False
    for site in plan.sites:
        ref = ctx.schema_id + host_ref(ctx, site.host, site.c)
        for ptype, names in site.patches.items():
            d = obj if obj.get("title") == ptype else obj.get("$defs", {}).get(ptype)
            for n in names:
                if d and d.get("properties", {}).get(n) == INLINE_STRING_PATCH:
                    d["properties"][n] = {"anyOf": [{"$ref": ref}, {"type": "null"}]}
                    changed = True
    if not changed:
        return t
    if style is None:
        ctx.note("manual", p, "diff json style is neither indent nor compact; edit by hand")
        return t
    return style(obj)


def rule_schema_json(ctx, plan, p, t):
    obj = json.loads(t)
    style = json_style(t, obj)
    changed, hosts_used = False, []
    for site in plan.sites:
        d = obj if obj.get("title") == site.host else obj.get("$defs", {}).get(site.host)
        if d is None or site.c not in d.get("properties", {}):
            continue
        hosts_used.append(site.host)
        prop = d["properties"][site.c]
        if "$ref" in prop or prop.get("type") != "string":
            if "$ref" not in prop:
                ctx.note("manual", p, f"{site.host}.{site.c} is not a plain string property")
            continue
        new = {"$ref": f"#/$defs/{plan.E}"}
        for k, v in prop.items():
            if k == "type":
                continue
            if k == "x-semio-ui" and isinstance(v, dict) and v.get("widget") == "text":
                v = {a: b for a, b in v.items() if a != "widget"}
            new[k] = v
        d["properties"][site.c] = new
        changed = True
    if hosts_used and plan.E not in obj.setdefault("$defs", {}):
        defs, new_defs = obj["$defs"], {}
        anchor = hosts_used[0] if hosts_used[0] in defs else None
        definition = enum_def(plan, template_def(obj))
        if anchor is None:
            new_defs[plan.E] = definition
        for k, v in defs.items():
            if k == anchor:
                new_defs[plan.E] = definition
            new_defs[k] = v
        obj["$defs"] = new_defs
        changed = True
    if not changed:
        return t
    if style is None:
        return text_patch_schema(ctx, plan, p, t, obj, hosts_used)
    return style(obj)


def match_brace(t, i):
    """Index just past the `}`/`]` that closes the bracket at t[i], skipping strings."""
    depth, j, n = 0, i, len(t)
    while j < n:
        c = t[j]
        if c == '"':
            j += 1
            while t[j] != '"':
                j += 2 if t[j] == "\\" else 1
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return j + 1
        j += 1
    raise ValueError("unbalanced")


def text_patch_schema(ctx, plan, p, t, obj, hosts_used):
    """Layout-preserving edit for schema files whose hand-made layout is none of the known dump styles: swap `type: string` of the property for the `$ref` and insert the enum definition before the first host definition."""
    out = t
    for site in plan.sites:
        if site.host not in hosts_used:
            continue
        if obj.get("title") == site.host:
            m = re.search(r'\n\s*"properties"\s*:\s*\{', out)
        else:
            h = re.search(r'\n(\s*)"%s"\s*:\s*\{' % re.escape(site.host), out[out.index('"$defs"'):])
            base = out.index('"$defs"') + h.end() if h else None
            m = re.search(r'\n\s*"properties"\s*:\s*\{', out[base:]) if base else None
            if m:
                m = type("M", (), {"end": lambda s, b=base, mm=m: b + mm.end()})()
        if not m:
            ctx.note("manual", p, f"cannot locate {site.host} properties for a text edit")
            return t
        a = m.end() - 1
        b = match_brace(out, a)
        prop = re.search(r'"%s"\s*:\s*\{' % re.escape(site.c), out[a:b])
        if not prop:
            ctx.note("manual", p, f"cannot locate {site.host}.{site.c} for a text edit")
            return t
        s0 = a + prop.end() - 1
        e0 = match_brace(out, s0)
        body = out[s0:e0]
        nb = re.sub(r'"type"\s*:\s*"string"', '"$ref": "#/$defs/%s"' % plan.E, body, count=1)
        nb = re.sub(r',\s*"widget"\s*:\s*"text"|"widget"\s*:\s*"text"\s*,\s*', "", nb)
        out = out[:s0] + nb + out[e0:]
    if '"%s"' % plan.E not in out[out.index('"$defs"'):]:
        defs = out.index('"$defs"')
        first = next((s.host for s in plan.sites if re.search(r'\n[ \t]*"%s"\s*:\s*\{' % re.escape(s.host), out[defs:])), None)
        template = template_def(obj)
        text = json.dumps(enum_def(plan, template), ensure_ascii=False)
        if first:
            h = re.search(r'\n([ \t]*)"%s"\s*:\s*\{' % re.escape(first), out[defs:])
            pos = defs + h.start() + 1
            out = out[:pos] + h.group(1) + '"%s": %s,\n' % (plan.E, text) + out[pos:]
        else:
            h = re.search(r'"\$defs"\s*:\s*\{\n([ \t]*)', out)
            pos = h.end() - len(h.group(1))
            out = out[:pos] + h.group(1) + '"%s": %s,\n' % (plan.E, text) + out[pos:]
    json.loads(out)
    return out


def ts_enum_lines(plan):
    toks = " | ".join(json.dumps(plan.tokens[v.name]) for v in plan.conv.variants)
    lits = ", ".join(json.dumps(plan.tokens[v.name]) for v in plan.conv.variants)
    return f"export type {plan.E} = {toks};\n", f"export const parse{plan.E}: NormWireReader<{plan.E}> = normWireLiteral({lits});\n"


def ts_interface_field(t, iface, names, newty):
    """Replace `name: string[ | null]` (or `name?:`) inside `export interface iface {…}`."""
    rng = struct_body(t, r"export interface %s\b[^{]*\{" % re.escape(iface))
    if not rng:
        return t
    a, b = rng
    body = t[a:b]
    for n in names:
        body = re.sub(r"(\b%s\??:\s*)string\b" % re.escape(n), r"\g<1>" + newty, body)
    return t[:a] + body + t[b:]


def ts_parser_field(t, parser, names, expr):
    """Inside the one-line `export const parseX … = normWireObject<…>({ … })` swap `normWireString` of the named fields for `expr`."""
    m = re.search(r"^export const %s\b[^\n]*$" % re.escape(parser), t, re.M)
    if not m:
        return t
    line = m.group(0)
    for n in names:
        line = re.sub(r"(\b%s:\s*(?:normWire\w+\()*)normWireString" % re.escape(n), r"\g<1>" + expr.replace("\\", "\\\\"), line)
    return t[:m.start()] + line + t[m.end():]


def rule_snapshot_ts(ctx, plan, p, t):
    out = t
    used = False
    for site in plan.sites:
        if not re.search(r"export interface %s\b" % site.host, out):
            continue
        before = out
        out = ts_interface_field(out, site.host, [site.c], plan.E)
        out = ts_parser_field(out, "parse" + site.host, [site.c], f"normWireRef(() => parse{plan.E})")
        used = used or out != before or bool(re.search(r"\b%s\??:\s*%s\b" % (site.c, plan.E), out))
    if used and not re.search(r"export type %s\b" % plan.E, out):
        ty, pa = ts_enum_lines(plan)
        first = next(s.host for s in plan.sites if re.search(r"export interface %s\b" % s.host, out))
        out = re.sub(r"^(export interface %s\b)" % first, lambda m: ty + m.group(1), out, count=1, flags=re.M)
        out = re.sub(r"^(export const parse%s\b)" % first, lambda m: pa + m.group(1), out, count=1, flags=re.M)
        if "normWireLiteral" not in out.split("export ", 1)[0]:
            out, ok = add_import_names(out, r"[^\"]*norm-js/registry/contract", ["normWireLiteral"])
            if not ok:
                ctx.note("manual", p, "no contract import to add normWireLiteral to")
    return out


def rule_diff_ts(ctx, plan, p, t):
    out, used = t, False
    for site in plan.sites:
        for ptype, names in site.patches.items():
            if not re.search(r"export interface %s\b" % ptype, out):
                continue
            before = out
            out = ts_interface_field(out, ptype, names, plan.E)
            out = ts_parser_field(out, "parse" + ptype, names, f"normWireRef(() => parse{plan.E})")
            used = used or out != before
    if used:
        out, ok = add_import_names(out, r"[^\"]*snapshot[^\"]*", [f"type {plan.E}", f"parse{plan.E}"])
        if not ok:
            ctx.note("manual", p, "no snapshot import to add the enum to")
    return out


def has_native_enums(t):
    return re.search(r"^\s*enum \w+", t, re.M) is not None


def rule_graphql(ctx, plan, p, t):
    if not has_native_enums(t):
        return t
    out, used = t, False
    for site in plan.sites:
        m = re.search(r"\btype %s\s*\{" % site.host, out)
        if not m:
            continue
        o = m.end() - 1
        pairs, _ = brace_pairs(out)
        close = next((b for a, b in pairs if a == o), None)
        body = out[o:close]
        new = re.sub(r"(\b%s:\s*)String(!?)" % re.escape(site.c), r"\g<1>" + plan.E + r"\2", body)
        if new != body:
            out = out[:o] + new + out[close:]
            used = True
        elif re.search(r"\b%s:\s*%s\b" % (site.c, plan.E), body):
            used = True
    if used and not re.search(r"^enum %s\b" % plan.E, out, re.M):
        first = next(s.host for s in plan.sites if re.search(r"\btype %s\s*\{" % s.host, out))
        decl = f"enum {plan.E} {{ " + " ".join(v.name for v in plan.conv.variants) + " }\n"
        ms = list(re.finditer(r"^enum \w+[^\n]*\n", out, re.M))
        if ms:
            out = out[:ms[-1].end()] + decl + out[ms[-1].end():]
        else:
            out = re.sub(r"^(type %s\b)" % first, lambda m: decl + "\n" + m.group(1), out, count=1, flags=re.M)
    return out


def proto_prefix(plan, t):
    first = snake(plan.E).split("_")[0].upper()
    taken = set(re.findall(r"\b([A-Z][A-Z0-9_]*)\s*=\s*\d+\s*;", t))
    names = [f"{first}_{snake(v.name).upper()}" for v in plan.conv.variants]
    if any(n in taken for n in names):
        first = plan.e_upper
    return first


def rule_proto(ctx, plan, p, t):
    if not re.search(r"^enum \w+", t, re.M):
        return t
    out, used = t, False
    for site in plan.sites:
        m = re.search(r"\bmessage %s\s*\{" % site.host, out)
        if not m:
            continue
        o = m.end() - 1
        pairs, _ = brace_pairs(out)
        close = next((b for a, b in pairs if a == o), None)
        body = out[o:close]
        new = re.sub(r"\bstring(\s+%s\s*=)" % re.escape(site.field), plan.E + r"\1", body)
        if new != body:
            out = out[:o] + new + out[close:]
            used = True
        elif re.search(r"\b%s\s+%s\s*=" % (plan.E, site.field), body):
            used = True
    if used and not re.search(r"^enum %s\b" % plan.E, out, re.M):
        block_style = re.search(r"^enum \w+ \{\n  [A-Z0-9_]+ = \d+;\n", out, re.M) is not None
        pre = plan.e_upper if block_style else proto_prefix(plan, out)
        if block_style:
            decl = f"enum {plan.E} {{\n" + "".join(f"  {pre}_{snake(v.name).upper()} = {i};\n" for i, v in enumerate(plan.conv.variants)) + "}\n"
        else:
            vals = " ".join(f"{pre}_{snake(v.name).upper()} = {i};" for i, v in enumerate(plan.conv.variants))
            decl = f"enum {plan.E} {{ {vals} }}\n"
        ms = list(re.finditer(r"^enum \w+ \{\n(?:  [^\n]*\n)*\}\n", out, re.M)) if block_style else list(re.finditer(r"^enum \w+[^\n]*\n", out, re.M))
        if ms:
            out = out[:ms[-1].end()] + decl + out[ms[-1].end():]
        else:
            out = re.sub(r"^(message )", lambda m: decl + m.group(1), out, count=1, flags=re.M)
    return out


# ---------------------------------------------------------------- rust rules (model, diff, leaves, literals, comparisons)

PILOT_ATTRS = ("#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]\n"
               "#[canonical_json(owner = semio_framework_pack_json)]\n"
               "#[cfg_attr(any(test, feature = \"test-support\"), derive(serde::Serialize, serde::Deserialize))]\n")


def enum_template(t):
    for m in re.finditer(r"((?:[ \t]*#\[[^\n]*\]\n)+)pub enum (\w+)\s*\{", t):
        if "CanonicalJsonTree" in m.group(1) and m.group(2) != "AnnexChoice":
            return m.group(1)
    return None


def rs_enum_text(plan, t):
    attrs = enum_template(t) or PILOT_ATTRS
    if "DslScalar" not in attrs:
        attrs = attrs.replace("value_derive::ToValue", "semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue", 1)
    doc = plan.conv.doc or f"🔠 {' '.join(w.lower() for w in split_words(plan.E))}."
    variants = "".join(f"    {v.name},\n" for v in plan.conv.variants)
    return f"/// {doc}\n{attrs}pub enum {plan.E} {{\n{variants}}}\n\n" + (rs_catalogue_impl(plan) if plan.conv.catalogue else "")


def rs_catalogue_impl(plan):
    """Catalogue mode: validated id lookup and EN/DE labels of the closed catalogue behind the enum."""
    vs = plan.conv.variants
    def ident(v):
        return v.legacy[0] if v.legacy else v.name
    arms_id = "".join(f'            Self::{v.name} => {json.dumps(ident(v))},\n' for v in vs)
    arms_from = "".join(f'            {json.dumps(ident(v))} => Some(Self::{v.name}),\n' for v in vs)
    arms_label = "".join(f'            Self::{v.name} => if german {{ {json.dumps(v.de, ensure_ascii=False)} }} else {{ {json.dumps(v.en, ensure_ascii=False)} }},\n' for v in vs)
    return (f"impl {plan.E} {{\n    /// 🆔 The catalogue id (`{plan.conv.catalogue}`) the tables and the editor list use.\n    pub const fn id(self) -> &'static str {{\n        match self {{\n{arms_id}        }}\n    }}\n"
            f"    /// 🔎 The catalogue entry of an id, `None` for an id outside the catalogue.\n    pub fn from_id(id: &str) -> Option<Self> {{\n        match id {{\n{arms_from}            _ => None,\n        }}\n    }}\n"
            f"    /// 🏷️ The English or German label of the catalogue entry.\n    pub const fn label(self, german: bool) -> &'static str {{\n        match self {{\n{arms_label}        }}\n    }}\n}}\n\n")


def attr_block_start(t, idx):
    """Start index of the doc/attribute lines directly above the item that begins at idx."""
    lines = t[:idx].split("\n")
    n = len(lines) - 1
    k = n
    while k > 0 and re.match(r"\s*(///|#\[|#!\[)", lines[k - 1]):
        k -= 1
    return sum(len(x) + 1 for x in lines[:k])


def model_enum_defined(ctx, plan):
    pat = re.compile(r"pub enum %s\b" % plan.E)
    for p in ctx.files:
        if p.endswith(".rs") and FIXTURE not in p:
            t = read_text(p)
            if t and pat.search(t):
                return True
    return False


def rule_model_rs(ctx, plan, p, t):
    out, used = t, False
    q = plan.qual(p)
    for site in plan.sites:
        rng = struct_body(out, r"pub struct %s\b[^{;]*\{" % site.host)
        if not rng:
            continue
        a, b = rng
        body = out[a:b]
        new = re.sub(r"(\bpub\s+%s\s*:\s*)(Option<)?String(>)?(\s*,)" % re.escape(site.field),
                     lambda m: m.group(1) + (f"Option<{q}>" if m.group(2) else q) + m.group(4), body)
        if new != body:
            out = out[:a] + new + out[b:]
            used = True
        elif re.search(r"\bpub\s+%s\s*:\s*(?:Option<)?(?:[\w:]*::)?%s\b" % (re.escape(site.field), plan.E), body):
            used = True
    if used and plan.in_model(p) and not model_enum_defined(ctx, plan) and not re.search(r"pub enum %s\b" % plan.E, out):
        first = next(s.host for s in plan.sites if re.search(r"pub struct %s\b" % s.host, out))
        m = re.search(r"pub struct %s\b" % first, out)
        pos = attr_block_start(out, m.start())
        out = out[:pos] + rs_enum_text(plan, out) + out[pos:]
    if plan.conv.catalogue and plan.in_model(p) and re.search(r"pub enum %s\b" % plan.E, out) and not re.search(r"impl %s \{" % plan.E, out):
        m = re.search(r"pub enum %s\b[^{]*\{.*?\n\}\n\n?" % plan.E, out, re.S)
        out = out[:m.end()] + (rs_catalogue_impl(plan) if m.group(0).endswith("\n\n") else "\n" + rs_catalogue_impl(plan)) + out[m.end():]
    return out


def rule_diff_rs(ctx, plan, p, t):
    out = t
    q = f"{ctx.model_crate}::{plan.E}"
    for site in plan.sites:
        for ptype, names in site.patches.items():
            for n in names:
                sn = snake(n)
                rng = struct_body(out, r"pub struct %s\b[^{;]*\{" % re.escape(ptype))
                if rng:
                    a, b = rng
                    body = re.sub(r"(\bpub\s+%s\s*:\s*Option<)String(>)" % re.escape(sn), r"\g<1>" + q + r"\2", out[a:b])
                    out = out[:a] + body + out[b:]
                out = re.sub(r"(\b%s\s*=>\s*[\w\.]+\s*:\s*)String\b" % re.escape(sn), r"\g<1>" + q, out)
        pat, pos = re.compile(r"\bfor\s+[\w:]*\b%s\s*\{" % site.host), 0
        while True:
            m = pat.search(out, pos)
            if not m:
                break
            pairs, _ = brace_pairs(out)
            o = m.end() - 1
            close = next((b for a, b in pairs if a == o), None)
            pos = m.end()
            if close is None:
                continue
            body = re.sub(r"(\b%s\s*:\s*)String\b" % re.escape(site.field), r"\g<1>" + q, out[o:close])
            out = out[:o] + body + out[close:]
    return out


def leaf_info(plan, p):
    """(leaf dir, new field) when path lies in a converted leaf, else None."""
    for d, nf in plan.leaves.items():
        if p.startswith(d + "/"):
            return d, nf
    return None


def rule_leaf(ctx, plan, p, t):
    info = leaf_info(plan, p)
    if not info:
        return t
    d, nf = info
    r = p[len(d) + 1:]
    cn = camel(nf)
    out = t
    if r == "🦀️.rs":
        out = re.sub(r"(\bpub\s+%s\s*:\s*)String(\s*[,}])" % re.escape(nf), r"\g<1>" + plan.E + r"\2", out)
        if re.search(r"\bpub\s+%s\s*:\s*%s\b" % (nf, plan.E), out) and not re.search(r"use [\w:]*::%s;|use [\w:]*::\{[^}]*\b%s\b" % (plan.E, plan.E), out):
            line = f"use {ctx.model_crate}::{plan.E};\n"
            m = re.search(r"^use %s::" % re.escape(ctx.model_crate), out, re.M)
            out = out[:m.start()] + line + out[m.start():] if m else ensure_after_imports(out, line.rstrip("\n"))
    elif r in ("🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs"):
        out = re.sub(r"(payload\.%s)\.clone\(\)" % re.escape(nf), r"\1", out)
        for site in plan.sites:
            out = re.sub(r"(\.%s)\.clone\(\)" % re.escape(site.field), r"\1", out)
    elif r == "🧬️schema/🟦️.ts":
        out = re.sub(r"(\b%s:\s*)string\b" % re.escape(cn), r"\g<1>" + plan.E, out)
        out = re.sub(r"(\b%s:\s*normWireRequired\()normWireString\)" % re.escape(cn), r"\g<1>parse%s)" % plan.E, out)
        if re.search(r"\b%s:\s*%s\b" % (cn, plan.E), out) and f"parse{plan.E}" in out and not re.search(r"import \{[^}]*parse%s\b" % plan.E, out):
            out = ensure_after_imports(out, f'import {{ type {plan.E}, parse{plan.E} }} from "../../../📸️snapshot/🟦️.ts";')
    elif r == "🧬️schema/🔣️.json":
        obj = json.loads(out)
        style = json_style(out, obj)
        prop = obj.get("properties", {}).get(cn)
        if prop and "$ref" not in prop and prop.get("type") == "string":
            if style is None:
                ctx.note("manual", p, "json style")
                return out
            new = {"$ref": f"{ctx.schema_id}#/$defs/{plan.E}"}
            for k, v in prop.items():
                if k == "type":
                    continue
                if k == "x-semio-ui" and v.get("widget") == "text":
                    v = {a: b for a, b in v.items() if a != "widget"}
                new[k] = v
            obj["properties"][cn] = new
            out = style(obj)
    return out


# ---------------------------------------------------------------- sqlite codecs

def sql_tables(sql):
    """{table: (start, end)} of every CREATE TABLE block."""
    out = {}
    for m in re.finditer(r"CREATE TABLE (?:IF NOT EXISTS )?(\w+)\s*\(", sql):
        o = m.end() - 1
        depth, i = 0, o
        while i < len(sql):
            if sql[i] == "(":
                depth += 1
            elif sql[i] == ")":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        out[m.group(1)] = (o, i)
    return out


def host_table(ctx, site):
    p = ctx.root + "/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"
    sql = read_text(p) if os.path.exists(p) else None
    if not sql:
        return None, None
    best = None
    sib = {snake(x) for x in site.siblings}
    for name, (a, b) in sql_tables(sql).items():
        cols = set(re.findall(r"(?:^|[,(\n])\s*(\w+)\s+(?:TEXT|REAL|INTEGER|BLOB)", sql[a:b]))
        if site.field not in cols:
            continue
        score = len(cols & sib) / max(1, len(sib))
        if best is None or score > best[0]:
            best = (score, name)
    return (best[1], sql) if best and best[0] >= 0.5 else (None, sql)


def rule_sqlite_sql(ctx, plan, p, t):
    out = t
    toks = ",".join(f"'{plan.tokens[v.name]}'" for v in plan.conv.variants)
    for site in plan.sites:
        table, _ = host_table(ctx, site)
        if not table:
            continue
        a, b = sql_tables(out)[table]
        block = out[a:b]
        if re.search(r"CHECK\(\s*%s\s+IN" % re.escape(site.field), block):
            continue
        new = re.sub(r"(\b%s\s+TEXT\s+NOT NULL)(?!\s+CHECK)" % re.escape(site.field), r"\1 CHECK(%s IN (%s))" % (site.field, toks), block, count=1)
        out = out[:a] + new + out[b:]
    return out


def sqlite_helper_names(t, plan):
    w = None
    for cand in (plan.e_snake, plan.e_snake + "_text"):
        if re.search(r"\bfn %s\(" % cand, t):
            w = cand
    return w or plan.e_snake + "_text"


def rule_sqlite_rs(ctx, plan, p, t):
    out, used = t, False
    w = sqlite_helper_names(out, plan)
    rd = "read_" + plan.e_snake
    for site in plan.sites:
        table, _ = host_table(ctx, site)
        if table:
            pos = 0
            while True:
                m = re.compile(r'"%s"' % re.escape(table)).search(out, pos)
                if not m:
                    break
                end = out.find(";", m.start())
                seg = out[m.start():end]
                new = re.sub(r"Cell::Text\(&(\w+)\.%s\)" % re.escape(site.field), lambda x: f"Cell::Text({w}({x.group(1)}.{site.field}))", seg)
                if new != seg:
                    out = out[:m.start()] + new + out[end:]
                    used = True
                pos = m.start() + 1
        pat, pos = re.compile(r"\b%s\s*\{" % site.host), 0
        while True:
            m = pat.search(out, pos)
            if not m:
                break
            pos = m.end()
            if out[max(0, m.start() - 8):m.start()].endswith(("struct ", "impl ")):
                continue
            pairs, _ = brace_pairs(out)
            o = m.end() - 1
            close = next((b for a, b in pairs if a == o), None)
            if close is None:
                continue
            seg = out[o:close]
            new = re.sub(r"(\b%s\s*:\s*)((?:\w+\.)?text\([^()]*\)\?)" % re.escape(site.field), lambda x: f"{x.group(1)}{rd}({x.group(2)})?", seg)
            if new != seg:
                out = out[:o] + new + out[close:]
                used = True
    for site in plan.sites:
        sib = sorted(site.rs_fields - {site.field, "id"}) or [snake(x) for x in site.siblings]
        stmts = list(re.finditer(r"[^;{}]*Cell::Text\(&(\w+)\.%s\)[^;]*;" % re.escape(site.field), out))
        for m in reversed(stmts):
            var = m.group(1)
            if sum(1 for s in sib if re.search(r"\b%s\.%s\b" % (var, re.escape(s)), m.group(0))) >= min(2, len(sib)):
                seg = re.sub(r"Cell::Text\(&%s\.%s\)" % (var, re.escape(site.field)), f"Cell::Text({w}({var}.{site.field}))", m.group(0))
                out = out[:m.start()] + seg + out[m.end():]
                used = True
        pat, pos = re.compile(r"\b%s\s*\{" % site.host), 0
        while True:
            m = pat.search(out, pos)
            if not m:
                break
            pos = m.end()
            if out[max(0, m.start() - 8):m.start()].endswith(("struct ", "impl ")):
                continue
            pairs, _ = brace_pairs(out)
            o = m.end() - 1
            close = next((b for a, b in pairs if a == o), None)
            if close is None:
                continue
            seg = out[o:close]
            new = re.sub(r"(\b%s\s*:\s*)reconstruct_text\(\s*\w+\s*,\s*((?:\w+\.)?text\([^()]*\)\?)\s*\)\?" % re.escape(site.field), lambda x: f"{x.group(1)}{rd}({x.group(2)})?", seg)
            if new != seg:
                out = out[:o] + new + out[close:]
                used = True
        for m in reversed(list(re.finditer(r"for text in \[([^\]]*)\]", out))):
            body = m.group(1)
            if not re.search(r"&(\w+)\.%s\b" % re.escape(site.field), body):
                continue
            def elem(x):
                v, f = x.group(1), x.group(2)
                if f == site.field:
                    return f"{w}({v}.{f})"
                return f"{v}.{f}.as_str()" if re.fullmatch(r"id|\w+_id|label_en|label_de|name", f) else x.group(0)
            nb = re.sub(r"&(\w+)\.(\w+)\b(?!\.)", elem, body)
            out = out[:m.start(1)] + nb + out[m.end(1):]
            used = True
    if used:
        if not re.search(r"\bfn %s\(" % rd, out):
            art = ctx.conv.artifact.upper()
            arms_w = "".join(f'{plan.E}::{v.name}=>"{plan.tokens[v.name]}",' for v in plan.conv.variants)
            arms_r = "".join(f'"{plan.tokens[v.name]}"=>Ok({plan.E}::{v.name}),' for v in plan.conv.variants)
            helpers = (f"fn {w}(a:{plan.E})->&'static str{{match a{{{arms_w}}}}}\n"
                       f"fn {rd}(a:&str)->Result<{plan.E}, ValueError>{{match a{{{arms_r}_=>Err(invalid(\"{art} {plan.e_snake.replace('_', ' ')} differs\"))}}}}\n")
            m = re.search(r"^fn ", out, re.M)
            out = out[:m.start()] + helpers + out[m.start():]
        for site in plan.sites:
            m = re.search(r"use crate::\{([^}]*)\};", out)
            if m and site.host in [x.strip() for x in m.group(1).split(",")]:
                if plan.E not in [x.strip() for x in m.group(1).split(",")]:
                    out = out[:m.end(1)] + f", {plan.E}" + out[m.end(1):]
                break
    return out


def rule_sqlite_ts(ctx, plan, p, t):
    out, used = t, False
    const = plan.e_upper + "S"
    for site in plan.sites:
        table, _ = host_table(ctx, site)
        sib = [site.c] + sorted(site.siblings)
        for m in reversed(list(re.finditer(r"export type (\w+)=\{([^}]*)\}", out))):
            keys = set(re.findall(r"(\w+):", m.group(2)))
            if site.c in keys and len(keys & site.siblings) >= min(3, len(site.siblings)):
                body = re.sub(r"(\b%s:)string\b" % re.escape(site.c), r"\g<1>" + plan.E, m.group(2))
                if body != m.group(2):
                    out = out[:m.start(2)] + body + out[m.end(2):]
                    used = True
        if table:
            pos = 0
            while True:
                m = re.compile(r'write\(out,"%s",\[' % re.escape(table)).search(out, pos)
                if not m:
                    break
                a = m.end() - 1
                close = out.find("]", a)
                seg = out[a:close]
                new = re.sub(r"(?<!choice\()(?<![\w.])(\w+)\.%s\b" % re.escape(site.c), lambda x: f"choice({x.group(1)}.{site.c},{const})", seg)
                if new != seg:
                    out = out[:a] + new + out[close:]
                    used = True
                pos = m.end()
        pairs, _ = brace_pairs(out)
        edits = []
        for a, b in pairs:
            d = direct_text(out, pairs, a, b)
            keys = set(re.findall(r"(?<![\w\"'.])(\w+):(?!:)", d))
            if site.c in keys and len(keys & site.siblings) >= min(3, len(site.siblings)):
                for x in re.finditer(r"(?<![\w\"'.])%s:(artifactSqliteText\([^()]*\))" % re.escape(site.c), d):
                    edits.append((a + x.start(1), a + x.end(1), x.group(1)))
        for s, e, inner in sorted(edits, reverse=True):
            out = out[:s] + f"choice({inner},{const})" + out[e:]
            used = True
    if used:
        if not re.search(r"\bconst %s\b" % const, out):
            arr = ",".join(json.dumps(plan.tokens[v.name]) for v in plan.conv.variants)
            decl = f"const {const}:readonly {plan.E}[]=[{arr}];\n"
            m = re.search(r"^const \w+:readonly ", out, re.M) or re.search(r"^(?:async )?function ", out, re.M)
            out = out[:m.start()] + decl + out[m.start():]
        if not re.search(r"\bfunction choice<", out):
            helper = 'function choice<T extends string>(v:unknown,values:readonly T[]):T{if(typeof v!=="string"||!values.includes(v as T))throw Error("owned enum differs");return v as T}\n'
            m = re.search(r"^(?:async )?function ", out, re.M)
            out = out[:m.start()] + helper + out[m.start():]
        m = re.search(r"import\s+type\s*\{([^}]*)\}\s*from\s*[\"'][^\"']*🧬️schema/📸️snapshot/🟦️\.ts[\"']", out)
        if m and plan.E not in [x.strip() for x in m.group(1).split(",")]:
            out = out[:m.end(1)] + f",{plan.E}" + out[m.end(1):]
    return out


# ---------------------------------------------------------------- rust usage: literals, comparisons, match arms, residuals

def vpath(plan, p, name):
    return f"{plan.qual(p)}::{name}"


def lit_variant(ctx, plan, p, raw, where):
    v = plan.map_value(unescape(raw))
    if v is None:
        v = plan.map_value(unescape(raw).lower())
    if v is None:
        if not plan.ambiguous:
            ctx.note("unmapped", p, f"{where}: literal {raw!r} has no variant")
        return None
    return vpath(plan, p, v)


def expr_end(d, start):
    depth, i, n = 0, start, len(d)
    while i < n:
        c = d[i]
        if c == '"':
            i += 1
            while i < n and d[i] != '"':
                i += 2 if d[i] == "\\" else 1
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                return i
            depth -= 1
        elif c == "," and depth == 0:
            return i
        i += 1
    return n


STR_FORMS = [r'%s\.(?:into|to_string|to_owned)\(\)' % LIT, r'String::from\(%s\)' % LIT]


def rule_rs_usage(ctx, plan, p, t):
    out = t
    if any(s in ctx.rel(p) for s in ("🧬️schema/🧬️mutations/",)) and leaf_info(plan, p):
        pass
    for site in plan.sites:
        f = re.escape(site.field)
        edits = []
        pairs, _ = brace_pairs(out)
        byopen = {a: b for a, b in pairs}
        host_pat = r"(?<![\w])(?:(?:\w+::)*%s|Self)\s*\{" % re.escape(site.host) if not plan.ambiguous else r"(?<![\w])(?:\w+::)*%s\s*\{" % re.escape(site.host)
        for m in re.finditer(host_pat, out):
            if re.search(r"(struct|impl|for|trait|enum|use)\s+(?:[\w:]+::)?$", out[:m.start()][-60:]):
                continue
            o = m.end() - 1
            close = byopen.get(o)
            if close is None:
                continue
            d = direct_text(out, pairs, o, close)
            for fm in re.finditer(r"(?<![\w.])%s\s*:\s*" % f, d):
                s = o + fm.end()
                e = o + expr_end(d, fm.end())
                ex = out[s:e]
                new = ex
                for form in STR_FORMS:
                    new = re.sub(form, lambda x: lit_variant(ctx, plan, p, x.group(1), f"{site.host}.{site.field} literal") or x.group(0), new)
                if new != ex:
                    edits.append((s, e, new))
                elif '"' in ex and plan.E not in ex:
                    ctx.note("residual", p, f"{site.host}.{site.field} value not a plain literal: {ex.strip()[:100]}")
        for s, e, new in sorted(edits, reverse=True):
            out = out[:s] + new + out[e:]
        acc = r"(\.%s)(?:\.as_str\(\))?" % f
        out = re.sub(acc + r"(\s*)(==|!=)(\s*)" + LIT, lambda x: f"{x.group(1)}{x.group(2)}{x.group(3)}{x.group(4)}{lit_variant(ctx, plan, p, x.group(5), 'comparison') or chr(34) + x.group(5) + chr(34)}", out)
        out = re.sub(r"(\.%s\s*=\s*)%s\.(?:into|to_string|to_owned)\(\)(?=\s*[;,}])" % (f, LIT), lambda x: x.group(1) + (lit_variant(ctx, plan, p, x.group(2), "assignment") or chr(34) + x.group(2) + chr(34) + ".into()"), out)
        out = re.sub(LIT + r"(\s*)(==|!=)(\s*)([\w\.]*\.%s)(?:\.as_str\(\))?" % f, lambda x: f"{x.group(5)}{x.group(2)}{x.group(3)}{x.group(4)}{lit_variant(ctx, plan, p, x.group(1), 'comparison') or chr(34) + x.group(1) + chr(34)}", out)
        out = re.sub(r"(\.%s)\.eq_ignore_ascii_case\(%s\)" % (f, LIT), lambda x: f"{x.group(1)} == {lit_variant(ctx, plan, p, x.group(2), 'eq_ignore_ascii_case') or chr(34) + x.group(2) + chr(34)}", out)
        out = re.sub(r"(?P<head>matches!\(\s*[^,()]*\.%s)(?:\.as_str\(\))?(?P<sep>\s*,\s*)(?P<lits>%s(?:\s*\|\s*%s)*)(?P<tail>\s*\))" % (f, LIT, LIT),
                     lambda x: x.group("head") + x.group("sep") + re.sub(LIT, lambda y: lit_variant(ctx, plan, p, y.group(1), "matches!") or y.group(0), x.group("lits")) + x.group("tail"), out)
        if plan.conv.catalogue and not leaf_info(plan, p):
            out = re.sub(r"(\.%s)\.as_str\(\)" % f, r"\1.id()", out)
            out = re.sub(r"&((?:\w+\.|\w+\[\w+\]\.)*\w+\.%s)\b(?![\w(.])" % f, r"\1.id()", out)
        pairs, _ = brace_pairs(out)
        byopen = {a: b for a, b in pairs}
        edits = []
        for m in re.finditer(r"\bmatch\s+[^{};\n]*?\.%s((?:\.to_lowercase\(\)|\.to_ascii_lowercase\(\)|\.as_str\(\))*)\s*\{" % f, out):
            o = m.end() - 1
            close = byopen.get(o)
            if close is None:
                continue
            body = out[o:close]
            arms = re.findall(r"(?<![\w])%s(?=\s*(?:\||=>|if\b))" % LIT, body)
            if plan.ambiguous and any(plan.map_value(unescape(a)) is None for a in arms):
                continue
            nb = re.sub(r"(?<![\w])%s(?=\s*(?:\||=>|if\b))" % LIT, lambda x: lit_variant(ctx, plan, p, x.group(1), "match arm") or x.group(0), body)
            nb = re.sub(r"((?:\w+::)+\w+)(\s*\|\s*)\1(?![\w:])", r"\1", nb)
            head = out[m.start():o]
            nh = head.replace(m.group(1), "", 1) if m.group(1) else head
            if nb != body or nh != head:
                edits.append((m.start(), close, nh + nb))
        for s, e, new in sorted(edits, reverse=True):
            out = out[:s] + new + out[e:]
    return out


def residual_lines(ctx, plan, p, t):
    rel = ctx.rel(p)
    if "🧬️schema/📸️snapshot/" in rel or "🚪️io/🪶️sqlite" in rel or "🧬️schema/🔺️diff/" in rel or "field-meta" in rel or "edit-rules" in rel:
        return
    if leaf_info(plan, p):
        return
    for i, line in enumerate(t.split("\n"), 1):
        if line.lstrip().startswith("//"):
            continue
        for site in plan.sites:
            f = re.escape(site.field)
            if not re.search(r"\.%s\b(?!\s*\()" % f, line) or re.search(r"\b%s\b" % plan.E, line):
                continue
            if re.search(r"&[\w\.\[\]()]*\.%s\b|\.%s\.(?:as_str|to_lowercase|clone|eq|len|is_empty|starts_with|contains)|\"[^\"]*\{[^}]*\}[^\"]*\".*\.%s\b|match [^{]*\.%s\b|\.%s\)" % (f, f, f, f, f), line):
                ctx.note("residual", p, f"{i}: {line.strip()[:170]}")


# ---------------------------------------------------------------- values in fixtures, assets, python oracles

def wire_for(plan, variant):
    return plan.tokens[variant]


def value_keysets(plan):
    """[(key names, siblings)]: the host property key with its host siblings, every patch property with its patch siblings."""
    out = []
    for s in plan.sites:
        out.append(({s.c}, s.siblings))
        for ptype, names in s.patches.items():
            for n in names:
                out.append(({n}, s.patch_sets[ptype] - {n, "id"}))
    return out


def rule_values(ctx, plan, p, t, scenario=None):
    out = t
    edits = []
    for keys, sib in value_keysets(plan):
        if p.endswith(".semio"):
            sites = dsl_value_sites(out, {kebab(next(iter(keys)))}, {kebab(x) for x in sib})
        else:
            sites = json_value_sites(out, keys, sib, p.endswith(".py"))
        for s, e, key, raw in sites:
            raw_v = raw.strip('"') if p.endswith(".semio") else raw
            v = plan.map_value(raw_v)
            have = plan.current_variant(raw_v)
            if have is not None:
                if v is not None and v != have:
                    ctx.note("lossy", p, f"{key}={raw_v!r} is already variant {have}; the legacy map says {v}: kept")
                continue
            if v is None:
                v = plan.conv.default
                if v is None:
                    ctx.note("unmapped", p, f"{key}={raw_v!r} has no variant (give a legacy token or --default)")
                    continue
                ctx.log.append(f"defaulted {key}={raw_v!r} -> {v} in {ctx.rel(p)}")
            if p.endswith(".semio"):
                new = kebab(v)
            else:
                new = json.dumps(wire_for(plan, v), ensure_ascii=False)[1:-1]
            cur = raw_v
            if cur != new:
                if p.endswith(".semio"):
                    edits.append((s, e, new if re.fullmatch(r"[\w-]+", new) else json.dumps(new)))
                else:
                    edits.append((s, e, out[s] + new + out[s]))
    seen = set()
    for s, e, new in sorted(edits, reverse=True):
        if (s, e) in seen:
            continue
        seen.add((s, e))
        out = out[:s] + new + out[e:]
    return out


def rule_py(ctx, plan, p, t):
    out = t
    for s in plan.sites:
        names = {s.c} | s.patch_names
        for n in names:
            acc = r"""(\[\s*["']%s["']\s*\]|\.get\(\s*["']%s["'][^)]*\))""" % (re.escape(n), re.escape(n))
            def repl(x):
                v = plan.map_value(x.group(4))
                return f"{x.group(1)}{x.group(2)}{x.group(3)}{plan.tokens[v]}{x.group(3)}" if v else x.group(0)
            out = re.sub(acc + r"""(\s*(?:==|!=)\s*)(["'])([^"']*)\3""", lambda x: (lambda v: f"{x.group(1)}{x.group(2)}{x.group(3)}{plan.tokens[v]}{x.group(3)}" if v else x.group(0))(plan.map_value(x.group(4))), out)
            out = re.sub(r"""(\.get\(\s*["']%s["']\s*,\s*)(["'])([^"']*)\2(\s*\))""" % re.escape(n), lambda x: (lambda v: f"{x.group(1)}{x.group(2)}{plan.tokens[v]}{x.group(2)}{x.group(4)}" if v else x.group(0))(plan.map_value(x.group(3))), out)
        for i, line in enumerate(out.split("\n"), 1):
            if any(re.search(r"""["']%s["']""" % re.escape(n), line) for n in names) and not any(tok in line for tok in plan.tokens.values()):
                ctx.note("residual", p, f"{i}: {line.strip()[:150]}")
    return out


# ---------------------------------------------------------------- editor field meta (rust table)

def path_host(ctx, path):
    """Host struct name addressed by a field-meta path like `members[].longitudinal[].position` (the type that owns the last segment)."""
    s = ctx.snapshot
    segs = path.replace("[]", "").split(".")
    node, name = s, s.get("title")
    for seg in segs[:-1]:
        prop = (node.get("properties") or {}).get(seg)
        if prop is None:
            return None
        prop = prop.get("items", prop)
        ref = prop.get("$ref") if isinstance(prop, dict) else None
        if not ref:
            return None
        name = ref.rsplit("/", 1)[-1]
        node = s["$defs"].get(name)
        if node is None:
            return None
    return name


def rule_field_meta(ctx, plan, p, t):
    out = rule_field_meta_choice(ctx, plan, p, t)
    return rule_field_meta_m(ctx, plan, p, out)


def choices_converged(plan, text):
    """True when an editor choice list already offers exactly the variants, each under its wire token, kebab token, name or a legacy token: an owner's hand-edited list (values, labels, order) is never regenerated."""
    values = re.findall(r'choice\("([^"]*)"|value:\s*"([^"]*)"', text)
    values = [a or b for a, b in values]
    forms = [{plan.tokens[v.name], kebab(v.name), v.name, *v.legacy} for v in plan.conv.variants]
    return len(values) == len(forms) and all(any(x in f for x in [val]) for val, f in zip(values, forms))


def rule_field_meta_choice(ctx, plan, p, t):
    """The `("path", FieldSpec::choice("en", "de", CONST))` style of the table: rewrites the choice block of CONST."""
    out = t
    block = "".join(f'    choice("{kebab(v.name)}", {json.dumps(v.en, ensure_ascii=False)}, {json.dumps(v.de, ensure_ascii=False)}),\n' for v in plan.conv.variants)
    for site in plan.sites:
        for m in list(re.finditer(r'\("([^"]*)",\s*FieldSpec::choice\(("[^"]*"),\s*("[^"]*"),\s*(\w+)\)\)', out)):
            path, const = m.group(1), m.group(4)
            if path.rsplit(".", 1)[-1] != site.c or path_host(ctx, path) != site.host:
                continue
            decl = f"const {const}: &[NormFieldChoice] = &[\n{block}];\n"
            cm = re.search(r"const %s: &\[NormFieldChoice\] = &\[\n.*?\n\];\n" % re.escape(const), out, re.S)
            if cm and not choices_converged(plan, cm.group(0)):
                out = out[:cm.start()] + decl + out[cm.end():]
    return out


def rule_field_meta_m(ctx, plan, p, t):
    out = t
    block = "".join(f'    choice("{kebab(v.name)}", {json.dumps(v.en, ensure_ascii=False)}, {json.dumps(v.de, ensure_ascii=False)}),\n' for v in plan.conv.variants)
    for site in plan.sites:
        for m in list(re.finditer(r'\("([^"]*)",\s*m\(("[^"]*"),\s*("[^"]*"),\s*(None|Some\(UnitId::\w+\)),\s*(None|Some\((\w+)\))\)\)', out)):
            path = m.group(1)
            if path.rsplit(".", 1)[-1] != site.c or path_host(ctx, path) != site.host:
                continue
            const = m.group(6) or plan.e_upper
            if not m.group(6):
                row = m.group(0)
                new_row = row[:row.rindex("None")] + f"Some({const})" + row[row.rindex("None") + 4:]
                out = out.replace(row, new_row, 1)
            decl = f"const {const}: &[NormFieldChoice] = &[\n{block}];\n"
            cm = re.search(r"const %s: &\[NormFieldChoice\] = &\[\n.*?\n\];\n" % re.escape(const), out, re.S)
            if cm:
                if not choices_converged(plan, cm.group(0)):
                    out = out[:cm.start()] + decl + out[cm.end():]
            else:
                at = re.search(r"^const fn m\(", out, re.M)
                out = out[:at.start()] + decl + "\n" + out[at.start():]
    return out


# ---------------------------------------------------------------- apply scenarios: the old `-alt` value becomes a real second variant

BEFORE_SUFFIXES = ("✅apply/🔣️.before.json", "✅apply/🔣️.before.json")


def is_before(p):
    return p.endswith(BEFORE_SUFFIXES)


def after_of(p):
    return p[:-len("🔣️.before.json")] + "🔣️.after.json" if p.endswith("🔣️.before.json") else p.replace("⬅️before", "➡️after")


def scenario_before(ctx, plan, p, t):
    """before.json of a `✅apply` scenario: keeps the old value unless it maps to the same variant as the after value (then another variant)."""
    after_p = after_of(p)
    after = read_text(after_p)
    if after is None:
        return t
    def values(text):
        out = []
        for keys, sib in [value_keysets(plan)[0]]:
            for s, e, key, raw in sorted(json_value_sites(text, keys, sib), key=lambda x: x[0]):
                out.append((s, e, raw))
        return out
    b, a = values(t), values(after)
    if len(b) != len(a):
        return t
    names = [v.name for v in plan.conv.variants]
    edits = []
    for (s, e, braw), (_, _, araw) in zip(b, a):
        if braw == araw or plan.map_value(araw) is None and plan.conv.default is None:
            continue
        if plan.current_variant(braw) is not None and plan.current_variant(araw) is not None:
            continue
        bv = plan.map_value(braw) or plan.conv.default
        av = plan.map_value(araw) or plan.conv.default
        if bv == av and braw != araw:
            alt = next(n for n in names if n != av)
            edits.append((s, e, t[s] + plan.tokens[alt] + t[s]))
    for s, e, new in sorted(edits, reverse=True):
        t = t[:s] + new + t[e:]
    if edits:
        ctx.log.append(f"scenario before flipped {len(edits)} value(s): {ctx.rel(p)}")
    return t


# ---------------------------------------------------------------- orchestration

TWIN_JSON = ("🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔣️.json")


def apply_rules(ctx, plan, p):
    rel = ctx.rel(p)
    name = os.path.basename(p)
    t = read_text(p)
    if t is None or not name.endswith(TEXT_EXT):
        return None
    out = t
    fixture = FIXTURE in p
    in_schema = rel.startswith("🧬️schema/")
    if not fixture:
        if rel in TWIN_JSON:
            out = rule_schema_json(ctx, plan, p, out)
        elif rel == "🧬️schema/🔺️diff/🔣️.json":
            out = rule_diff_json(ctx, plan, p, out)
        elif name == "🟦️.ts" and rel in ("🧬️schema/🟦️.ts", "🧬️schema/📸️snapshot/🟦️.ts"):
            out = rule_snapshot_ts(ctx, plan, p, out)
        elif name == "🟦️.ts" and rel == "🧬️schema/🔺️diff/🟦️.ts":
            out = rule_diff_ts(ctx, plan, p, out)
        elif name == "🔗️.graphql" and os.path.dirname(rel) in SNAPSHOT_FACETS:
            out = rule_graphql(ctx, plan, p, out)
        elif name == "🛰️.proto" and os.path.dirname(rel) in SNAPSHOT_FACETS:
            out = rule_proto(ctx, plan, p, out)
        elif rel == "🧬️schema/🔺️diff/🦀️.rs":
            out = rule_diff_rs(ctx, plan, p, out)
        elif rel == "🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql":
            out = rule_sqlite_sql(ctx, plan, p, out)
        elif rel == "🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs":
            out = rule_sqlite_rs(ctx, plan, p, out)
        elif rel == "🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts":
            out = rule_sqlite_ts(ctx, plan, p, out)
        elif rel.endswith("🏷️field-meta/🦀️.rs"):
            out = rule_field_meta(ctx, plan, p, out)
        if leaf_info(plan, p):
            out = rule_leaf(ctx, plan, p, out)
        if name == "🦀️.rs":
            if rel in ("🧬️schema/🦀️.rs", "🧬️schema/📸️snapshot/🦀️.rs"):
                out = rule_model_rs(ctx, plan, p, out)
            out = rule_rs_usage(ctx, plan, p, out)
            residual_lines(ctx, plan, p, out)
        if name.endswith(".py"):
            out = rule_py(ctx, plan, p, out)
            out = rule_values(ctx, plan, p, out)
        if name.endswith(".semio"):
            out = rule_values(ctx, plan, p, out)
        if name.endswith(".json") and not in_schema and any(x in rel for x in ("🔮️oracles", "📚️examples", "🖼️assets")):
            out = rule_values(ctx, plan, p, out)
    else:
        if name.endswith(".json"):
            if is_before(p):
                out = scenario_before(ctx, plan, p, out)
            out = rule_values(ctx, plan, p, out)
            for d, nf in plan.leaves.items():
                if ("/" + os.path.basename(d) + "/") in p:
                    out = leaf_payload_values(ctx, plan, p, out, nf)
    return out


def leaf_payload_values(ctx, plan, p, t, nf):
    """`newX` of a leaf payload fixture."""
    cn = camel(nf)
    edits = []
    for m in re.finditer(r'"%s"\s*:\s*"((?:[^"\\]|\\.)*)"' % re.escape(cn), t):
        have = plan.current_variant(m.group(1))
        if have is not None:
            if plan.map_value(m.group(1)) not in (None, have):
                ctx.note("lossy", p, f"{cn}={m.group(1)!r} is already variant {have}; the legacy map says {plan.map_value(m.group(1))}: kept")
            continue
        v = plan.map_value(m.group(1)) or plan.conv.default
        if v is None:
            ctx.note("unmapped", p, f"{cn}={m.group(1)!r}")
            continue
        if plan.tokens[v] != m.group(1):
            edits.append((m.start(1), m.end(1), plan.tokens[v]))
    for s, e, new in sorted(edits, reverse=True):
        t = t[:s] + new + t[e:]
    return t


def run(root, conv, dry=False, check=False, verbose=True):
    ctx = Ctx(root, conv, dry, check)
    plan = Plan(ctx, conv)
    for v in conv.variants:
        conv.legacy.setdefault(plan.tokens[v.name], v.name)
    model = read_text(root + "/🧬️schema/📸️snapshot/🦀️.rs") or ""
    m = re.search(r"pub enum %s \{\n(.*?)\n\}" % re.escape(conv.enum), model, re.S)
    if m:
        have = re.findall(r"^    ([A-Z]\w*),$", m.group(1), re.M)
        want = [v.name for v in conv.variants]
        if have != want:
            ctx.note("lossy", root + "/🧬️schema/📸️snapshot/🦀️.rs", f"spec drift: the model enum {conv.enum} has {have}, the spec lists {want}; a value of a missing variant would be defaulted: fix the spec first")
    changed = []
    before_first = sorted(ctx.files, key=lambda x: not is_before(x))
    for p in before_first:
        if os.path.islink(p):
            continue
        new = apply_rules(ctx, plan, p)
        old = read_text(p)
        if new is not None and new != old:
            changed.append(p)
            if not dry and not check:
                cur = read_text(p)
                if cur != old:
                    new = apply_rules(ctx, plan, p)
                with open(p, "w", encoding="utf-8", newline="") as f:
                    f.write(new)
    return ctx, plan, changed


def parse_args(argv):
    opts = {"only": None, "root": None, "check": False, "dry": False, "apply": False, "doc": "", "default": None, "remove": False, "spec": None}
    pos = []
    i = 0
    while i < len(argv):
        a = argv[i]
        if a in ("--root", "--doc", "--default", "--spec", "--only"):
            opts[a[2:]] = argv[i + 1]
            i += 2
            continue
        if a in ("--check", "--dry", "--remove", "--apply"):
            opts[a[2:]] = True
        elif a == "--leaf-keep":
            pass
        else:
            pos.append(a)
        i += 1
    return opts, pos


def conversions_from(opts, pos):
    if opts["spec"]:
        spec = json.load(open(opts["spec"], encoding="utf-8"))
        for c in spec["conversions"]:
            art = c.get("artifact", spec.get("artifact"))
            variants = [Variant(v[0], v[1], v[2], v[3]) for v in c.get("variants", [])]
            if c.get("from_const"):
                if "enum %s " % c["enum"] in (read_text((opts["root"] or artifact_root(art)) + "/🧬️schema/📸️snapshot/🦀️.rs") or ""):
                    raise SystemExit(f"{c['enum']} is already converted: `from_const` is a first-run input (a rerun would read the rewritten list and default every fixture value); paste the variants into the spec and drop `from_const`")
                variants += variants_from_const(artifact_root(art) if not opts["root"] else opts["root"], c["from_const"], c.get("name_prefix", "C"), c.get("legacy_empty"))
            yield Conversion(art, c["hosts"], c["enum"], variants, c.get("doc", ""), c.get("default"), catalogue=c.get("catalogue"), wire=c.get("wire", spec.get("wire")))
        return
    art, hosts, enum = pos[0], pos[1].split(","), pos[2]
    yield Conversion(art, hosts, enum, [Variant.parse(v) for v in pos[3:]], opts["doc"], opts["default"])


def variants_from_const(root, const, prefix, empty_to=None):
    """Catalogue mode: the variants are the rows of an editor choice list (`choice("id", "EN", "DE")`); the id is the legacy token, the variant name its identifier spelling."""
    text = read_text(root + "/✏️editor/🏷️field-meta/🦀️.rs")
    m = re.search(r"const %s: &\[NormFieldChoice\] = &\[\n(.*?)\n\];" % re.escape(const), text, re.S)
    assert m, f"no choice list {const}"
    out = []
    for r in re.finditer(r'choice\("([^"]*)",\s*"((?:[^"\\]|\\.)*)",\s*"((?:[^"\\]|\\.)*)"\)', m.group(1)):
        ident = re.sub(r"[^A-Za-z0-9]", "", r.group(1))
        name = ident[:1].upper() + ident[1:] if ident[:1].isalpha() else prefix + ident
        out.append(Variant(name, [r.group(1)], r.group(2).encode().decode("unicode_escape") if "\\" in r.group(2) else r.group(2), r.group(3)))
    return out


def main(argv):
    opts, pos = parse_args(argv)
    opts["dry"] = opts["dry"] or not (opts["apply"] or opts["check"])
    rc = 0
    for conv in conversions_from(opts, pos):
        if opts["only"] and conv.enum != opts["only"]:
            continue
        root = opts["root"] or artifact_root(conv.artifact)
        ctx, plan, changed = run(root, conv, opts["dry"], opts["check"])
        mode = "check" if opts["check"] else "dry" if opts["dry"] else "apply"
        print(f"== {conv.artifact} {conv.enum} [{mode}] leaves={len(plan.leaves)} files-changed={len(changed)}")
        for p in changed[:400 if (opts['dry'] or opts['check']) else 40]:
            print("   ~", ctx.rel(p))
        for kind in ("manual", "unmapped", "lossy", "residual"):
            rows = [x for x in ctx.todo if x.startswith(kind + ":")]
            for r in sorted(set(rows))[:200]:
                print("  ", r)
            if rows:
                print(f"   ({len(rows)} {kind})")
        for l in ctx.log[:15]:
            print("   log:", l)
        if len(ctx.log) > 15:
            print(f"   log: ... {len(ctx.log)} entries")
        if opts["check"] and (changed or any(x.startswith(("manual", "unmapped", "lossy")) for x in ctx.todo)):
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
