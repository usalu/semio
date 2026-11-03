"""🩺 Repairs of generator defects that earlier tool runs left in committed SQLite codecs: type arguments the old inline cloning appended to the generic types of other rows (`Omit<Row,OtherRow,"a"|"b">`, `Exact<Row,OtherRow>`) the doubled quote that a list edit left in an `Exact<..>` key list, and snake_case keys in the root snapshot type (the TypeScript model is camelCase). Every rule is a text rewrite of exactly that shape; a file that needs none is left alone."""
import re
from core import walk_files

SQLITE = "🚪️io/🪶️sqlite/📸️snapshot"
GENERIC = re.compile(r"\b(Omit|Pick)<([A-Za-z_][\w$]*)((?:,[A-Z][\w$]*)+),(?=\")")
EXACT = re.compile(r"\bExact<([A-Za-z_][\w$]*)((?:,[A-Z][\w$]*)+)>")
SNAKE_LINE = re.compile(r"^export type \w+SqliteSnapshot=[^\n]*$", re.M)
SNAKE = re.compile(r"(\"|(?<=[;{]))([a-z][a-z0-9]*(?:_[a-z0-9]+)+)(\"|(?=:))")
DOUBLED = re.compile(r"(\|\"\w+\")\"(?=>)")


ORDERED_LOOP = re.compile(r"of await ordered\((\w+)\.values\(\)")


def renamed_declarators(text):
    """🏷️ A read loop over `frame_member_basis.values()` whose rows are declared as `frameMemberBaseRows=await entities(..)` (the declarator was renamed with a singularised name and nothing else uses it): the unused `...Rows` declarator that best matches the loop's name (similarity 0.85 or more) takes the loop's name again. Returns the new text and the number of rewrites."""
    import difflib
    count = 0
    flat = lambda word: re.sub(r"rows$|[^a-z0-9]", "", word.lower())
    for name in dict.fromkeys(ORDERED_LOOP.findall(text)):
        if re.search(rf"(?<![\w$.]){name}\s*=\s*await\s+(?:entities|groups)\(", text):
            continue
        orphans = [d for d in re.findall(r"(?<![\w$.])(\w+Rows)\s*=\s*await\s+entities\(", text) if len(re.findall(rf"(?<![\w$.]){d}(?![\w$])", text)) == 1]
        best = max(orphans, key=lambda d: difflib.SequenceMatcher(None, flat(d), flat(name)).ratio(), default=None)
        if best and difflib.SequenceMatcher(None, flat(best), flat(name)).ratio() >= 0.85:
            text, n = re.subn(rf"(?<![\w$.]){best}(?![\w$])", name, text)
            count += n
    return text, count


def fix(text):
    """🔧 The repaired text and the number of rewrites."""
    count = 0
    text, count = renamed_declarators(text)

    def drop(m):
        nonlocal count
        count += 1
        return f"{m.group(1)}<{m.group(2)},"

    def exact(m):
        nonlocal count
        count += 1
        return f"Exact<{m.group(1)}>"
    text = GENERIC.sub(drop, text)
    text = EXACT.sub(exact, text)
    new, n = DOUBLED.subn(r"\1", text)

    def camel_line(m):
        nonlocal n
        line, k = SNAKE.subn(lambda s: s.group(1) + re.sub(r"_([a-z0-9])", lambda u: u.group(1).upper(), s.group(2)) + s.group(3), m.group(0))
        n += k
        return line
    new = SNAKE_LINE.sub(camel_line, new)
    return new, count + n


RULE = re.compile(r'SetFieldRule \{ path: "(\w+)\[\]\.(\w+)", kind: "([a-z0-9-]+)", selectors: &\[SelectorKey::Id\("(\w+)"\)\], value: "(\w+)"')


def selector_repairs(ctx):
    """🎯 Set-field vectors whose payload does not name the row (and the value) that differs between the committed before and after documents (the donor kind's id and value survived the clone): both are taken from the pair."""
    import json, os
    from l_mutations import MUT
    rules = "".join(ctx.plan.read(p) for p in walk_files(ctx.layout.p(MUT + "/🧭️edit-rules"), (".rs",)))
    root = ctx.layout.p("🧪️tests/🧫️fixtures/🧬️mutations")
    for lane, field, kind, key, value in RULE.findall(rules):
        for folder in (d for d in os.listdir(root) if re.fullmatch(r"\W*" + re.escape(kind), d)):
            base = os.path.join(root, folder, "✅apply")
            names = [os.path.join(base, "🔣️." + part + ".json") for part in ("mutation", "before", "after")]
            if not all(os.path.isfile(n) for n in names):
                continue
            mutation, before, after = (json.loads(ctx.plan.read(n)) for n in names)
            body = next(iter(mutation.values())) if len(mutation) == 1 and isinstance(next(iter(mutation.values())), dict) else mutation
            rows = before.get(lane) or []
            moved = [(b["id"], a[field]) for b, a in zip(rows, after.get(lane) or []) if b.get(field) != a.get(field)]
            if key not in body or len(moved) != 1:
                continue
            wanted = {key: moved[0][0], value: moved[0][1]}
            if all(body.get(k) == v for k, v in wanted.items()):
                continue
            body.update(wanted)
            ctx.plan.write(names[0], json.dumps(mutation, indent=2, ensure_ascii=False) + "\n")
            ctx.plan.note(f"repair: {kind} vector addresses {moved[0][0]!r} in {key} and sets {value} to the after value")


FLOATS_DECL = re.compile(r"^const ([A-Z][A-Z0-9_]*)_FLOATS\b", re.M)


def float_constants(text):
    """🔤 The Rust codec names the float-column constants without their `_FLOATS` suffix where only the suffixed constant is declared (`SUPPORT_COLLISION` for `SUPPORT_COLLISION_FLOATS`): the uses are renamed. Returns the new text and the number of rewrites."""
    count = 0
    for name in FLOATS_DECL.findall(text):
        if re.search(rf"^const {name}\b(?!_)", text, re.M):
            continue
        text, n = re.subn(rf"(?<![A-Za-z0-9_]){name}(?![A-Za-z0-9_])", f"{name}_FLOATS", text)
        count += n
    return text, count


def run(ctx):
    """🩹 Plans the repair of the artifact's SQLite TypeScript codec and of the set-field vectors' row ids."""
    selector_repairs(ctx)
    for path in walk_files(ctx.layout.p(SQLITE), (".ts", ".rs")):
        if "/🧪️tests/" in path or path.endswith(".d.ts"):
            continue
        text = ctx.plan.read(path)
        new, n = float_constants(text) if path.endswith(".rs") else fix(text)
        if n:
            ctx.plan.write(path, new)
            ctx.plan.note(f"repair: {n} rewrites in {ctx.plan.rel(path)}")
