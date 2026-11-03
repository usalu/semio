"""🧩 Shared kernel of the schema-evolve tool: name forms, the in-memory edit plan (dry run prints the file plan, apply writes it atomically), text anchors and the artifact layout discovery."""
import difflib, json, os, pathlib, re, sys

SEMIO = os.environ.get("SEMIO_ROOT") or str(next(p for p in pathlib.Path(__file__).resolve().parents if (p / "✏️s" / "🔌️plugins" / "📕️norm").is_dir()))
NORM = SEMIO + "/✏️s/🔌️plugins/📕️norm"
ARTIFACTS = NORM + "/🗿️artifacts"
SCHEMA, SNAPSHOT, DIFF = "🧬️schema", "🧬️schema/📸️snapshot", "🧬️schema/🔺️diff"
MAX_PATH = 255
SKIP_DIRS = {"target", "node_modules", "__pycache__", "🗑️generated", ".git"}


class Unsupported(Exception):
    """🚧 The artifact layout lacks an anchor this layer needs; the dry run reports it, apply refuses."""


def camel(s):
    return re.sub(r"[_-]([a-z0-9])", lambda m: m.group(1).upper(), s)


def snake(s):
    return re.sub(r"(?<!^)([A-Z])", r"_\1", s.replace("-", "_")).lower()


def pascal(s):
    c = camel(s)
    return c[:1].upper() + c[1:]


def kebab(s):
    return snake(s).replace("_", "-")


def plural_of(word):
    return word if word.endswith("s") else (word[:-1] + "ies" if word.endswith("y") else word + "s")


def singular_of(word):
    return word[:-3] + "y" if word.endswith("ies") else (word[:-1] if word.endswith("s") else word)


PROTECT = set()


def protect_names(defs, root_props):
    """🪡 Identifiers that must never be renamed as part of a longer name: every property and definition name of the schema, in all spellings."""
    PROTECT.clear()
    names = set(root_props) | set(defs)
    for d in defs.values():
        names |= set(d.get("properties", {}))
    for n in names:
        PROTECT.update({n, snake(n), camel(snake(n))})


class Names:
    """🏷️ All spellings of one collection: plural camel (`layerWaters`), snake, Pascal and kebab, and the same for its row entity (`LayerWater`)."""

    def __init__(self, plural_camel, entity):
        self.plural_camel, self.entity = plural_camel, entity
        self.plural_snake, self.plural_pascal, self.plural_kebab = snake(plural_camel), pascal(plural_camel), kebab(plural_camel)
        self.snake, self.camel, self.kebab = snake(entity), camel(snake(entity)), kebab(entity)

    def forms(self):
        return [(self.plural_pascal, "P"), (self.plural_camel, "c"), (self.plural_snake, "s"), (self.plural_kebab, "k"), (self.entity, "E"), (self.camel, "e"), (self.snake, "t"), (self.kebab, "d")]

    def mapping(self, other):
        pairs = [(a, b) for (a, _), (b, _) in zip(self.forms(), other.forms())]
        return sorted(dict(pairs).items(), key=lambda kv: -len(kv[0]))

    def rename(self, text, other, evidence=None, unquoted="snake"):
        """🧶 Word-aligned rename of every spelling; spellings that coincide for a single-word name (`joints` is camel, snake and kebab) take the form the context shows: kebab next to `-`, snake in Rust paths, otherwise the quoted-name convention of the file (`evidence`, default the text)."""
        pairs = {}
        for (a, kind), (b, _) in zip(self.forms(), other.forms()):
            pairs.setdefault(a, {})[kind] = b
        keys = sorted(pairs, key=lambda k: -len(k))
        rx = re.compile("|".join(re.escape(k) for k in keys))
        quoted = evidence if evidence is not None else text
        snake_votes = len(re.findall(r'"[a-z][a-z0-9]*_[a-z0-9_]+"', quoted))
        camel_votes = len(re.findall(r'"[a-z][a-z0-9]*[A-Z][A-Za-z0-9]*"', quoted))

        def pick(m):
            src, forms = m.group(0), pairs[m.group(0)]
            before, after = text[m.start() - 1] if m.start() else " ", text[m.end()] if m.end() < len(text) else " "
            if src[0].isupper():
                ok = not (before.islower() or before == "_" or before.isdigit()) or bool(re.search(r"En\d{4}$|(?:Insert|Remove|Change|Set|Add|Delete|Patch|Move|Edit|Update)(?:d|ed|s)?$", text[: m.start()]))
            else:
                ident = re.match(r"[\w-]+" if "-" in src else r"\w+", text[m.start():]).group(0)
                ok = not (before.isalnum() or before == "_") and (ident == src or (not after.isdigit() and ident not in PROTECT and (after == "_" or after.isupper())))
            if not ok:
                return src
            options = set(forms.values())
            if len(options) == 1:
                return next(iter(options))
            if ("k" in forms or "d" in forms) and "-" in (before, after):
                return forms.get("k") or forms.get("d")
            if after.isupper() and ("e" in forms or "c" in forms):
                return forms.get("e") or forms.get("c")
            if after == "_" and ("t" in forms or "s" in forms):
                return forms.get("t") or forms.get("s")
            quote = before in "\"'" and after in "\"'" or evidence == '"aB"'
            plural = any(k in forms for k in "Pcsk")
            snake_form, camel_form = ("s" if plural else "t"), ("c" if plural else "e")
            if quote:
                first, second = (snake_form, camel_form) if snake_votes >= camel_votes and evidence != '"aB"' else (camel_form, snake_form)
                return forms.get(first) or forms.get(second) or next(iter(options))
            order = (camel_form, snake_form) if unquoted == "camel" else (snake_form, camel_form)
            return forms.get(order[0]) or forms.get(order[1]) or next(iter(options))
        return rx.sub(pick, text)


def lines_changed(old, new):
    return sum(1 for l in difflib.unified_diff(old.splitlines(), new.splitlines(), n=0, lineterm="") if l[:1] in "+-" and l[:3] not in ("+++", "---"))


class Plan:
    """📋 In-memory edit set: layers read through it (so later layers see earlier edits), the dry run prints one line per file, apply writes all files after checking that no file changed on disk meanwhile."""

    def __init__(self, root):
        self.root, self.files, self.notes, self.layer = root, {}, [], ""

    def rel(self, path):
        return str(path)[len(self.root) + 1:] if str(path).startswith(self.root) else str(path)

    def read(self, path):
        path = str(path)
        if path in self.files:
            return self.files[path][1]
        with open(path, encoding="utf8") as handle:
            return handle.read()

    def exists(self, path):
        return str(path) in self.files or os.path.exists(path)

    def write(self, path, text):
        path = str(path)
        if len(path[len(SEMIO) + 1:].encode()) > MAX_PATH:
            raise Unsupported("path exceeds 255 bytes: " + path[len(SEMIO) + 1:])
        old = self.files[path][0] if path in self.files else (open(path, encoding="utf8").read() if os.path.exists(path) else None)
        current = self.files[path][1] if path in self.files else old
        if current == text:
            return False
        layers = self.files[path][2] if path in self.files else ""
        if self.layer not in layers.split(","):
            layers = (layers + "," if layers else "") + self.layer
        self.files[path] = (old, text, layers)
        return True

    def edit(self, path, fn):
        text = self.read(path)
        new = fn(text)
        return self.write(path, new) if new is not None and new != text else False

    def note(self, text):
        self.notes.append(f"{self.layer}: {text}")

    def report(self):
        out = []
        for path, (old, new, layers) in sorted(self.files.items()):
            kind = "create" if old is None else "edit  "
            out.append(f"  {kind} {lines_changed(old or '', new):>5} lines  [{layers}]  {self.rel(path)}")
        return out

    def commit(self):
        for path, (old, new, _) in self.files.items():
            current = open(path, encoding="utf8").read() if os.path.exists(path) else None
            if current != old:
                raise RuntimeError(f"{path} changed on disk while the plan was built; run again")
        for path, (old, new, _) in self.files.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf8") as handle:
                handle.write(new)


def sub1(text, pattern, repl, flags=0, what=None):
    out, n = re.subn(pattern, repl, text, count=1, flags=flags)
    if n != 1:
        raise Unsupported(f"anchor not found: {what or pattern[:120]}")
    return out


def line_span(text, pattern, flags=re.M):
    m = re.search(pattern, text, flags)
    if not m:
        raise Unsupported(f"anchor not found: {pattern[:120]}")
    start = text.rfind("\n", 0, m.start()) + 1
    end = text.find("\n", m.end() - 1 if text[m.end() - 1] == "\n" else m.end())
    return start, (len(text) if end < 0 else end + 1)


def balanced_end(text, start):
    """🧵 The end (exclusive, after the line break) of the statement that begins at `start`: the first line end at bracket depth 0 that closes with `,` `;` or a closing bracket."""
    depth, i, quote = 0, start, False
    while i < len(text):
        ch = text[i]
        if quote:
            if ch == "\\":
                i += 1
            elif ch == '"':
                quote = False
        elif ch == '"':
            quote = True
        elif ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        elif ch == "\n" and depth <= 0:
            return i + 1
        i += 1
    return len(text)


def unit_at(text, pattern, flags=re.M):
    """🔎 The multi-line statement starting at the first line that matches `pattern` (line start .. balanced end)."""
    start, _ = line_span(text, pattern, flags)
    return start, balanced_end(text, start)


def item_at(text, pattern, flags=re.M):
    """🧲 A top-level Rust item (docstrings, attributes and the item up to its closing `}` at the same indent) containing the first match of `pattern`."""
    m = re.search(pattern, text, flags)
    if not m:
        raise Unsupported(f"item not found: {pattern[:120]}")
    start = text.rfind("\n", 0, m.start()) + 1
    indent = re.match(r"[ ]*", text[start:]).group(0)
    while True:
        prev = text.rfind("\n", 0, start - 1) + 1
        line = text[prev:start]
        if start > 0 and (line.startswith(indent + "///") or line.startswith(indent + "#[") or line.startswith(indent + "#![")):
            start = prev
        else:
            break
    first_end = text.find("\n", m.start())
    if text[m.start():first_end].rstrip().endswith(";"):
        return start, first_end + 1
    close = re.compile(r"^%s\}[^\n]*\n" % indent, re.M).search(text, m.end())
    if not close:
        raise Unsupported("item end not found: " + pattern[:80])
    return start, close.end()


def clone_after(text, span, names_from, names_to, guard=None):
    """🔨 Duplicates the text span with the collection names swapped, right after the span (idempotent through the guard string)."""
    chunk = text[span[0]:span[1]]
    new = names_from.rename(chunk, names_to)
    if guard and guard in text:
        return text
    if new in text:
        return text
    return text[:span[1]] + new + text[span[1]:]


def walk_files(root, suffixes=None, skip=()):
    for dp, ds, fs in os.walk(root):
        ds[:] = sorted(d for d in ds if d not in SKIP_DIRS and d not in skip)
        for f in sorted(fs):
            if suffixes is None or f.endswith(suffixes):
                yield os.path.join(dp, f)


class Layout:
    """🗺️ Where one artifact keeps its layers, found by reading the tree (nothing is hard-coded per artifact)."""

    def __init__(self, short):
        self.short = short
        matches = sorted(d for d in os.listdir(ARTIFACTS) if d.endswith(short))
        if not matches:
            raise Unsupported(f"artifact {short} not found under {ARTIFACTS}")
        self.dir = ARTIFACTS + "/" + matches[0]
        found = [p for p in walk_files(self.dir, ("🔣️.json",)) if p.endswith("/" + SNAPSHOT + "/🔣️.json")]
        if not found:
            raise Unsupported("no snapshot schema below " + self.dir)
        self.any = found[0][: -len("/" + SNAPSHOT + "/🔣️.json")]
        self.prefix, self.upper = short, short[0].upper() + short[1:]
        self.model_crate = "semio_s_artifact_norm_%s_model" % short
        self.snapshot_name = self.upper + "Snapshot"

    def p(self, rel):
        return self.any + "/" + rel

    def schema_json(self, rel=SNAPSHOT):
        return json.loads(open(self.p(rel + "/🔣️.json"), encoding="utf8").read())

    def collections(self, snapshot=None):
        """🔧 Root properties that are arrays of `$defs` records, in schema order: [(camel, entity)]."""
        s = snapshot or self.schema_json()
        out = []
        for name, node in s["properties"].items():
            items = node.get("items", {}).get("$ref", "") if node.get("type") == "array" else ""
            if items.startswith("#/$defs/"):
                out.append((name, items.rsplit("/", 1)[1]))
        return out

    def find(self, rel_glob_suffix):
        return [p for p in walk_files(self.any) if p.endswith(rel_glob_suffix)]
