"""🪆️ Python twin of `outline-registration.py`: the per-kind `for kind in KINDS:` registration loop becomes registrations under
the Scenario Outline base ids, a `spec_vector(kind)` factory becomes a handler that reads its row with `ctx.row()`, the
feature's `no-mutation-baseline-*` scenarios are registered exactly, and a `KINDS` tuple nothing else reads is deleted.

Usage: outline-registration-py.py [--write] <case-dir>..."""
import os, re, sys
write = "--write" in sys.argv

def outline_bases(feature):
    lines = feature.split("\n"); bases, plain = set(), set()
    for i, line in enumerate(lines):
        m = re.match(r"\s*@id-([a-z0-9-]+)\s*$", line)
        if not m: continue
        j = i + 1
        while j < len(lines) and lines[j].strip().startswith("@"): j += 1
        (bases if lines[j].strip().startswith("Scenario Outline:") else plain).add(m.group(1))
    return bases, plain

for case in [a for a in sys.argv[1:] if not a.startswith("--")]:
    path = os.path.join(case, "🐍️.py")
    if not os.path.exists(path): continue
    text = open(path, encoding="utf-8").read()
    loop = re.search(r'    built = Adapter\("python"\)\n    for kind in KINDS:\n        built = built(?P<calls>(?:\.oracle\("[a-z-]+-%s" % kind, [\w()]+\))+)\n    return built(?P<rest>[^\n]*)\n', text)
    if not loop:
        print("skip", case.split("/🧪️tests/")[-1]); continue
    bases, plain = outline_bases(open(os.path.join(case, "🥒️.feature"), encoding="utf-8").read())
    calls = []
    for verb, handler in re.findall(r'\.oracle\("([a-z-]+)-%s" % kind, ([\w()]+)\)', loop.group("calls")):
        assert verb in bases, (case, verb)
        handler = handler.replace("(kind)", "")
        calls.append(f'.oracle("{verb}", {handler})')
        if f"no-mutation-baseline-{verb}" in plain:
            calls.append(f'.oracle("no-mutation-baseline-{verb}", {handler})')
    rest = re.sub(r'\.oracle\("no-mutation-baseline-[a-z]+", \w+\)|\.oracle\("spec-vector-no-mutation", spec_vector\("no-mutation"\)\)', "", loop.group("rest"))
    text = text.replace(loop.group(0), f'    return Adapter("python"){"".join(calls)}{rest}\n', 1)
    factory = re.search(r'def spec_vector\(kind: str\):\n(?P<doc>    """[\s\S]*?"""\n)\n    def handler\(ctx: Context\) -> Outcome:\n(?P<body>(?:        [^\n]*\n|\n)*?)\n    return handler\n', text)
    if factory:
        body = "\n".join(line[4:] for line in factory.group("body").rstrip("\n").split("\n"))
        text = text.replace(factory.group(0), f'def spec_vector(ctx: Context) -> Outcome:\n{factory.group("doc")}    kind = ctx.row()\n{body}\n', 1)
    code = "\n".join(l for l in text.split("\n") if not l.lstrip().startswith("#"))
    if len(re.findall(r"\bKINDS\b", code)) == 1:
        text = re.sub(r"(?:#[^\n]*\n)*KINDS = \([\s\S]*?\n\)\n\n?", "", text, count=1)
    print("changed", case.split("/🧪️tests/")[-1])
    if write: open(path, "w", encoding="utf-8").write(text)
