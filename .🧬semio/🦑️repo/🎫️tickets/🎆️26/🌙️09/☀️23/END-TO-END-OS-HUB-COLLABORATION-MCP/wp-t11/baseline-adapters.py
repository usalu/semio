"""⏸️ Drops `no-mutation` from a Rust adapter's KINDS and registers the two baseline scenario ids
(`no-mutation-baseline-mutate`/`-inverse`) on the same handlers the per-kind loop uses."""
import re, sys
for path in sys.argv[1:]:
    text = open(path, encoding="utf-8").read()
    before = text
    text = re.sub(r'\n\s*"no-mutation",(?=\n)', "", text, count=1) if re.search(r'\n\s*"no-mutation",\n', text) else text.replace('"no-mutation", ', "", 1)
    lines = text.split("\n")
    out, pending = [], []
    for line in lines:
        m = re.match(r'^(\s*)built = built\.(oracle|subject)\(&format!\("mutate-\{kind\}"\), ([\w:]+)\)(?:\.\2\(&format!\("inverse-\{kind\}"\), ([\w:]+)\))?;$', line)
        out.append(line)
        if m:
            indent = m.group(1)[:-4]
            role, mutate, inverse = m.group(2), m.group(3), m.group(4)
            call = f'{indent}built = built.{role}("no-mutation-baseline-mutate", {mutate})' + (f'.{role}("no-mutation-baseline-inverse", {inverse})' if inverse else "") + ";"
            pending.append((indent + "}", call))
            continue
        if pending and line == pending[-1][0]:
            out.append(pending.pop()[1])
    text = "\n".join(out)
    open(path, "w", encoding="utf-8").write(text)
    print(path.split("/🧪️tests/")[-1], "no-mutation left:", text.count('"no-mutation"'), "baseline regs:", text.count("no-mutation-baseline"), "unmatched:", len(pending))
