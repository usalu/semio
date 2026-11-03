"""Writes the showcase DSL asset of en1991, en1993, en1994, en1998 and en1999 from the committed showcase fixture: a loose draft (JSON keys kebab-cased, plain cells) is normalised by the asset-laws `patch.py` against the Rust model (field order, headers, unit suffixes, enum spelling), so the asset is the canonical print (python3 -I dsl_assets.py [--check])."""
import json, pathlib, re, sys, tempfile
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent.parent / "🛠️asset-laws"))
HERE = pathlib.Path(__file__).resolve().parent
src = (HERE.parent.parent / "🛠️asset-laws" / "patch.py").read_text(encoding="utf-8")
ns = {"__name__": "patch_lib", "__file__": str(HERE.parent.parent / "🛠️asset-laws" / "patch.py")}
ARGS = sys.argv[1:]
exec(compile(src.split("\nres=0\n")[0].replace('APPLY="--apply" in sys.argv', "APPLY=False").replace("ONLY=[a for a in sys.argv[1:] if not a.startswith(\"--\")]", "ONLY=[]"), "patch.py", "exec"), ns)
R, SUB = ns["R"], ns["SUB"]
CODES = {
    "🏋️en1991": ("🌉️bridge-showcase", "1991"),
    "🔩️en1993": ("🏗️steel-showcase", "1993"),
    "🧩️en1994": ("🌉️composite-showcase", "1994"),
    "🫨️en1998": ("🏢️seismic-showcase", "1998"),
    "🪶️en1999": ("🏠️aluminium-showcase", "1999"),
}


def kebab(key):
    return re.sub(r"(?<=[a-zA-Z0-9])(?=[A-Z])", "-", key).lower()


def cell(v):
    if v is None:
        return "_"
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return ns["rust_display"](v)
    if isinstance(v, list):
        return "[ " + "".join(cell(x) + " " for x in v) + "]"
    if isinstance(v, dict):
        return "{" + " ".join(f"{kebab(k)}={cell(x)}" for k, x in v.items()) + "}"
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_.\-]*", v) and v not in ("true", "false", "_"):
        return v
    return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'


def item_props(schema, spec):
    while "$ref" in spec:
        spec = schema["$defs"][spec["$ref"].rsplit("/", 1)[1]]
    return spec["properties"]


def snake(key):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", key).lower()


def synced(structs, enums, art):
    """Inserts the fields only the schema JSON declares (the Rust twin is regenerated later) at their schema position, so the draft matches the schema-first target."""
    schema = ns["schema_of"](art)
    defs = schema.get("$defs", {})

    def deref(s):
        while "$ref" in s:
            s = defs[s["$ref"].rsplit("/", 1)[1]]
        return s

    def field(name, spec):
        spec = deref(next((s for s in spec["anyOf"] if s.get("type") != "null"), spec)) if "anyOf" in spec else spec
        dsl = spec.get("x-semio-dsl", {})
        attrs = [f'#[dsl(unit = "{dsl["unit"]}")]'] if "unit" in dsl else ["#[dsl(table)]"] if dsl.get("table") else []
        kind = spec.get("type")
        if kind == "array":
            item = spec["items"]["$ref"].rsplit("/", 1)[1]
            assert item in structs, (art, name, item)
            rtype = f"Vec<{item}>"
        else:
            rtype = {"number": "f64", "integer": "u32", "boolean": "bool", "string": "String"}[kind]
            assert "enum" not in spec, (art, name)
        return ns["Field"](snake(name), rtype, attrs)

    def sync(model, props, where):
        known = {ns["norm"](f.name) for f in model}
        for index, (name, spec) in enumerate(props.items()):
            if ns["norm"](name) not in known:
                model.insert(index, field(name, spec))
                print(f"  schema-only field {where}.{name} (Rust twin lacks it)")

    top = ns["getstruct"](structs, schema["title"])
    sync(top, schema["properties"], "snapshot")
    for name, spec in schema["properties"].items():
        if spec.get("type") == "array":
            item = deref(spec["items"])
            f = next(g for g in top if ns["norm"](g.name) == ns["norm"](name))
            _, x, _ = ns["tag"](f, structs, enums)
            if "properties" in item and x in structs:
                sync(ns["getstruct"](structs, x), item["properties"], name)


PARSE_RUST = ns["parse_rust"]


def parse_synced(base):
    structs, enums = PARSE_RUST(base)
    art = next((a for a in CODES if base == f"{R}/{a}/{SUB}"), None)
    if art:
        synced(structs, enums, art)
    return structs, enums


ns["parse_rust"] = parse_synced


def draft(doc, art, structs, enums, magic):
    top = ns["getstruct"](structs, ns["schema_of"](art)["title"])
    by = {ns["norm"](f.name): f for f in top}
    scalars = []
    tables = []
    for k, v in doc.items():
        f = by[ns["norm"](k)]
        t, x, _ = ns["tag"](f, structs, enums)
        if t == "TABLE":
            item = ns["schema_of"](art)
            item = item["properties"][k]["items"]
            item = item_props(ns["schema_of"](art), item)
            fields = list(item)
            lines = [f"{kebab(k)} [" + " ".join(f"{kebab(n)}:TEXT" for n in fields) + "] {"]
            lines += ["  " + " ".join(cell(r[n]) for n in fields) for r in v]
            lines.append("}")
            tables.append("\n".join(lines))
        else:
            text = ns["convert"](cell(v), f, structs, enums, [], k)
            if t.startswith("?") and isinstance(v, str):
                text = text.lower()
            if t == "REC":
                text = text[1:-1]
            scalars.append(f"{kebab(k)}=" + text)
    return magic + "\n" + " ".join(scalars) + "\n" + "\n".join(tables) + "\n"


def build(art, slug, n):
    base = f"{R}/{art}/{SUB}"
    doc = json.loads(pathlib.Path(f"{base}/🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json").read_text(encoding="utf-8"))
    structs, enums = parse_synced(base)
    for p, fl in PARSE_RUST(ns["SHARED"])[0].items():
        structs.setdefault(p, []).extend(fl)
    out = pathlib.Path(f"{base}/🖼️assets/{slug}/{slug}/🗣️.dsl.semio")
    with tempfile.TemporaryDirectory() as tmp:
        probe = pathlib.Path(tmp) / "🖼️assets" / slug / "draft.dsl.semio"
        probe.parent.mkdir(parents=True)
        probe.write_text(draft(doc, art, structs, enums, f"semio norm.en{n}.dsl v1"), encoding="utf-8")
        _, new, changes, warn = ns["patch"](art, str(probe))
        again = pathlib.Path(tmp) / "🖼️assets" / slug / "again.dsl.semio"
        again.write_text(new, encoding="utf-8")
        _, newer, _, _ = ns["patch"](art, str(again))
    problems = [c for c in changes if "ADD" in c or "DROP" in c] + warn
    if problems or new != newer:
        print(art, "DRAFT PROBLEMS", problems[:20], "unstable" if new != newer else "")
        return True
    old = out.read_text(encoding="utf-8") if out.exists() else None
    if old != new and "--check" not in sys.argv:
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(new, encoding="utf-8")
    print(("unchanged " if old == new else "would write " if "--check" in sys.argv else "wrote ") + str(out.relative_to(ns["R"])), len(new))
    return old != new


if __name__ == "__main__":
    only = [a for a in sys.argv[1:] if not a.startswith("--")]
    failed = [build(a, s, n) for a, (s, n) in CODES.items() if not only or any(o in a for o in only)]
    sys.exit(1 if any(failed) and "--check" in sys.argv else 0)
