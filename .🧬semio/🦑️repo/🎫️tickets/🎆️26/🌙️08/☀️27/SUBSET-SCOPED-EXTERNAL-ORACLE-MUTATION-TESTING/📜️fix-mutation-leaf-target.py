"""🪪️ Put `dsl::MutationLeaf` on the type that implements `MutationKind`.

The bound the compiler enforces is `MutationKind: Self: MutationLeaf`, so the derive belongs on whatever
type carries the `impl MutationKind`. The migration attached it to the struct named by the descriptor's
`aggregateVariant`, which is usually the same type — but many leaves declare a `…Payload` struct AND a
separate two-phase `pub enum …Mutation { Apply(Payload), Restore(Diff) }` that implements the kind, and
there the derive landed on the payload while the bound stayed unsatisfied on the enum.
"""
import io, re, glob

KIND = re.compile(r"impl\s+(?:[a-z_]+::)*MutationKind\s*<[^>]*>\s+for\s+([A-Za-z0-9_]+)")

fixed = 0
for leaf in glob.glob("✏️s/🔌️plugins/**/🧬️mutations/*/🦀️.rs", recursive=True):
    text = io.open(leaf, encoding="utf-8").read()
    kinds = KIND.findall(text)
    if not kinds:
        continue
    target = kinds[0]
    decl = re.search(
        r"(#\[derive\(([^)]*)\)\]\s*\n(?:#\[[^\]]*\]\s*\n)*pub (?:struct|enum) " + re.escape(target) + r"\b)",
        text,
    )
    if decl is None or "MutationLeaf" in decl.group(2):
        continue
    replaced = decl.group(1).replace(
        f"#[derive({decl.group(2)})]",
        f"#[derive({decl.group(2)}, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]",
        1,
    )
    io.open(leaf, "w", encoding="utf-8").write(text[: decl.start(1)] + replaced + text[decl.end(1) :])
    fixed += 1
print(f"attached the derive to the MutationKind type in {fixed} leaf/leaves")
