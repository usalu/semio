"""🧬️ Move `infix`-layout mutation leaves onto the taxonomy's canonical shape.

An infix leaf already sits directly under the mutation collection but spells its primary
`🦀️component.rs`; the `dsl::MutationLeaf` derive requires the taxonomy's `🦀️.rs`. So unlike the nested
layout this is a RENAME plus the derive, with no file merge — and every one of these leaves already
carries its `🔣️.json`, which is what the derive reads.
"""
import io, os, re, glob, json

def migrate(leaf):
    src = os.path.join(leaf, "🦀️component.rs")
    dst = os.path.join(leaf, "🦀️.rs")
    if not os.path.exists(src) or os.path.exists(dst):
        return False
    desc_path = os.path.join(leaf, "🔣️.json")
    if not os.path.exists(desc_path):
        return False
    variant = json.load(open(desc_path))["aggregateVariant"]
    text = io.open(src, encoding="utf-8").read()
    pat = re.compile(r"(#\[derive\(([^)]*)\)\]\s*\n(?:#\[[^\]]*\]\s*\n)*pub struct " + re.escape(variant) + r"\b)")
    m = pat.search(text)
    if m and "MutationLeaf" not in m.group(2):
        replaced = m.group(1).replace(
            f"#[derive({m.group(2)})]",
            f"#[derive({m.group(2)}, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]",
            1,
        )
        text = text[: m.start(1)] + replaced + text[m.end(1) :]
    io.open(dst, "w", encoding="utf-8").write(text)
    os.remove(src)
    return True

moved = 0
for collection in sorted(glob.glob("✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations")):
    for leaf in sorted(glob.glob(os.path.join(collection, "*"))):
        if not os.path.isdir(leaf) or os.path.basename(leaf) in ("💾️binary", "📝️text", "🧬️schema"):
            continue
        if migrate(leaf):
            moved += 1

# every `#[path]` that named the old primary now names the canonical one
touched = 0
for glue in glob.glob("✏️s/🔌️plugins/*/📦️packages/🦀️rust/📦️glue.rs") + glob.glob("✏️s/🔌️plugins/*/*/📦️packages/🦀️rust/📦️glue.rs"):
    base = os.path.dirname(glue)
    s = io.open(glue, encoding="utf-8").read()
    def repath(m):
        rel = m.group(1)
        return m.group(0) if os.path.exists(os.path.normpath(os.path.join(base, rel + "/🦀️component.rs"))) else f'#[path = "{rel}/🦀️.rs"]'
    new = re.sub(r'#\[path = "([^"]*🧬️mutations/[^"/]+)/🦀️component\.rs"\]', repath, s)
    if new != s:
        io.open(glue, "w", encoding="utf-8").write(new)
        touched += 1
print(f"renamed {moved} infix leaf/leaves; repointed {touched} glue file(s)")
