"""🧭 Run context of one evolve command: plan, layout, spec and the reference sibling collections whose text the layers clone."""
import copy, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "field-adder"))
import schema_twins, jsonspan
from core import Layout, Names, Plan, SEMIO, Unsupported, camel, pascal, snake


def json_edit(plan, path, mutate, layout=None):
    """🧷 Edits a schema document through the plan, keeping the layout (indent or compact) the file has."""
    text = plan.read(path)
    data = json.loads(text)
    if mutate(data) is False:
        return False
    kind = layout or schema_twins.layout_of(text)
    return plan.write(path, schema_twins.dump_compact(data) if kind == "compact" else schema_twins.dump_indent(data))


def token_style(token):
    """🔤 `pascal` (FrameDerived), `camel` (frameDerived) or `other` (lower case, kebab, snake) spelling of an enum token."""
    if re.fullmatch(r"[A-Z][A-Za-z0-9]*", token):
        return "pascal"
    return "camel" if re.fullmatch(r"[a-z][a-z0-9]*[A-Z][A-Za-z0-9]*", token) else "other"


def conform_enum_tokens(ctx):
    """🔠 New enums take the token spelling the artifact uses for its existing enums (schema-sync compares enum spelling): PascalCase or camelCase when most existing tokens have it, otherwise the spec's tokens stay."""
    from collections import Counter
    votes = Counter(token_style(t) for v in ctx.defs.values() if v.get("type") == "string" and "enum" in v for t in v["enum"])
    style_ = votes.most_common(1)[0][0] if votes else "other"
    if style_ == "other" or not ctx.spec or not hasattr(ctx.spec, "collection"):
        return
    fields = []
    if ctx.spec.collection:
        fields += ctx.spec.collection.all_enums
    if getattr(ctx.spec, "leaf", None) is not None and ctx.spec.leaf.ty == "enum":
        fields.append(ctx.spec.leaf)

    def fix(token):
        if token_style(token) == style_:
            return token
        word = pascal(token) if re.search(r"[-_ ]", token) or token.islower() else token[0].upper() + token[1:]
        return word if style_ == "pascal" else word[0].lower() + word[1:]
    for f in fields:
        if not f.enum.get("values"):
            continue
        mapped = {v["token"]: fix(v["token"]) for v in f.values}
        f.set_values([{**v, "token": mapped[v["token"]]} for v in f.values])
        f.sample = mapped.get(f.sample, f.sample)


def style(text):
    return (", ", ": ") if re.search(r'"[\w$-]+": [\{"\[0-9tfn]', text) else (",", ":")


def dumps(value, text):
    return json.dumps(value, ensure_ascii=False, separators=style(text))


def parse(text):
    return jsonspan.parse(text)


def get(node, *path):
    for key in path:
        node = node.get(key).value
    return node


def append_member(text, obj, key, value):
    if obj.get(key):
        return text
    return jsonspan.insert_member(text, obj, obj.members[-1].key, key, dumps(value, text))


def append_item(text, arr, value):
    return jsonspan.insert_item(text, arr, len(arr.items) - 1, dumps(value, text))


def span_edit(plan, path, fn):
    """🪢 Edits a schema document by text spans (the layout of the file is kept): `fn(text)` returns the new text."""
    text = plan.read(path)
    new = fn(text)
    return plan.write(path, new) if new != text else False


def put_after(mapping, name, value, after=None):
    if name in mapping:
        return mapping
    out = {}
    done = False
    for key, item in mapping.items():
        out[key] = item
        if key == after:
            out[name] = value
            done = True
    if not done:
        out[name] = value
    return out


class Ctx:
    def __init__(self, short, spec, apply_=False):
        self.layout = Layout(short)
        self.short, self.upper, self.spec = short, self.layout.upper, spec
        self.plan = Plan(SEMIO)
        self.apply = apply_
        self.snapshot = self.layout.schema_json()
        self.defs = self.snapshot["$defs"]
        self.collections = self.layout.collections(self.snapshot)
        self.anchor = self.collections[-1][0]
        self.ref = self.choose(spec.reference, nested=False)
        self.unsupported = []
        self.nref = None
        if spec.collection and spec.collection.nested:
            try:
                self.nref = self.choose(spec.nested_reference, nested=True)
            except Unsupported as error:
                self.unsupported.append(f"mutations: nested rows ({spec.collection.nested[0].plural}) need a reference collection with nested rows and a diff row group: {error}")
        from core import protect_names
        protect_names(self.defs, self.snapshot["properties"])
        conform_enum_tokens(self)

    def nested_fields_of(self, entity):
        return [n for n, node in self.defs[entity]["properties"].items() if node.get("type") == "array" and node.get("items", {}).get("$ref", "").startswith("#/$defs/")]

    def has_diff_group(self, entity, plural):
        import l_schema
        if not hasattr(self, "_diff_defs"):
            self._diff_defs = json.loads(open(self.layout.p("🧬️schema/🔺️diff/🔣️.json"), encoding="utf8").read())["$defs"]
        rx = l_schema.family_regex(self, Names(plural, entity))
        return any(rx.match(k) for k in self._diff_defs)

    def choose(self, wanted, nested):
        own = self.spec.collection.plural if self.spec.collection else None
        names = [c for c in self.collections if c[0] not in ("placements", "cases", own)]
        if not wanted and not [c for c in names if self.has_diff_group(c[1], c[0])]:
            names = [c for c in self.collections if c[0] == "placements"]
        if wanted:
            hit = [c for c in self.collections if c[0] == wanted]
            if not hit:
                raise Unsupported(f"reference collection {wanted} not found in {self.short}")
            names = hit
        else:
            names = [c for c in names if self.has_diff_group(c[1], c[0])]
            if nested:
                names = [c for c in names if self.nested_fields_of(c[1])]
            else:
                flat = [c for c in names if not self.nested_fields_of(c[1])]
                if not flat:
                    flat = [c for c in self.collections if c[0] == "placements" and c[0] != own and self.has_diff_group(c[1], c[0])]
                names = flat or names
                names = [c for c in names if not any(self.defs[c[1]]["properties"][p].get("$ref", "").split("/")[-1] in self.enum_defs() for p in self.defs[c[1]]["properties"])] or names
        if not names:
            raise Unsupported("no reference sibling collection with a diff row group of the wanted shape in " + self.short)
        return Names(*names[-1])

    def enum_defs(self):
        return {k for k, v in self.defs.items() if v.get("type") == "string" and "enum" in v}

    def nested_ref_names(self):
        if not self.nref:
            if self.spec.collection and self.spec.collection.nested:
                raise Unsupported("no reference collection with nested rows")
            return None
        field = self.nested_fields_of(self.nref.entity)[0]
        return Names(field, self.defs[self.nref.entity]["properties"][field]["items"]["$ref"].rsplit("/", 1)[1])

    @property
    def new(self):
        return self.spec.collection.names

    def to_new(self, text):
        out = self.ref.rename(text, self.new)
        return out

    def layer(self, name):
        self.plan.layer = name

    def guard(self, fn, name):
        self.layer(name)
        try:
            fn(self)
        except Unsupported as error:
            self.unsupported.append(f"{name}: {error}")
