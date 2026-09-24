"""🪆️ Rewrites a Rust case adapter's per-kind registration loop (`for kind in KINDS { built.<role>(&<id of kind, verb>, H) }`)
into registrations under the Scenario Outline base ids (`built.<role>("<verb>", H)`), which the hosts now resolve for every
expanded row, plus the exact `no-mutation-baseline-<verb>` ids the feature declares. A `KINDS` list that nothing else reads
is deleted. Adapters whose handlers depend on `kind` or whose verbs are not outline ids are reported and left untouched.

Usage: outline-registration.py [--write] <case-dir>..."""
import os, re, sys

write = "--write" in sys.argv
ID = r'&(?:format!\("(?P<v1>[a-z-]+)-\{kind\}"\)|[\w:]*scenario_id\(kind, "(?P<v2>[a-z-]+)"\))'
CALL = re.compile(r'\.(?P<role>oracle|subject)\(' + ID + r', (?P<handler>[^()]*(?:\([^()]*\))?)\)')

def outline_bases(feature):
    lines = feature.split("\n")
    bases, plain = set(), set()
    for i, line in enumerate(lines):
        m = re.match(r"\s*@id-([a-z0-9-]+)\s*$", line)
        if not m: continue
        j = i + 1
        while j < len(lines) and lines[j].strip().startswith("@"): j += 1
        (bases if lines[j].strip().startswith("Scenario Outline:") else plain).add(m.group(1))
    return bases, plain

def rewrite(text, bases, plain):
    out, problems = text, []
    for loop in re.finditer(r"(?P<attr>[ \t]*#\[cfg\(feature = \"sut\"\)\]\n)?(?P<indent>[ \t]*)for &?kind in (?P<src>[\w:]+|subject::SUBJECT_KINDS)(?:\.iter\(\))? \{\n(?P<body>(?:.*\n)*?)(?P=indent)\}\n", text):
        body = loop.group("body")
        if "kind" in re.sub(ID, "", body).replace("KINDS", ""):
            problems.append("handler depends on kind"); continue
        calls = list(CALL.finditer(body))
        if not calls:
            problems.append("no registration calls in loop"); continue
        verbs = {c.group("v1") or c.group("v2") for c in calls}
        if not verbs <= bases:
            problems.append(f"verbs {sorted(verbs - bases)} are not outline ids"); continue
        def call_sub(c):
            verb = c.group("v1") or c.group("v2")
            role, handler = c.group("role"), c.group("handler")
            extra = f'.{role}("no-mutation-baseline-{verb}", {handler})' if f"no-mutation-baseline-{verb}" in plain else ""
            return f'.{role}("{verb}", {handler}){extra}'
        new_body = CALL.sub(call_sub, body)
        new_body = "\n".join(line[4:] if line.startswith(loop.group("indent") + "    ") else line for line in new_body.split("\n"))
        replacement = (loop.group("attr") or "") + new_body
        if loop.group("attr") and new_body.count("built = ") > 1:
            replacement = loop.group("attr") + loop.group("indent") + "{\n" + "\n".join("    " + l if l.strip() else l for l in new_body.rstrip("\n").split("\n")) + "\n" + loop.group("indent") + "}\n"
        out = out.replace(loop.group(0), replacement, 1)
    return out, problems

def drop_unused_kinds(text):
    uses = len(re.findall(r"\bKINDS\b", text))
    definition = re.search(r"(?:[ \t]*///[^\n]*\n|[ \t]*#\[[^\n]*\n)*[ \t]*(?:pub )?const KINDS: [^=]+= [^;]*;\n", text)
    if definition and uses == 1:
        text = text.replace(definition.group(0), "", 1)
    text = re.sub(r"\n//#region 🔖️Kinds\n\s*//#endregion 🔖️Kinds\n", "\n", text)
    imported = re.search(r"use [^;]*\bKINDS\b[^;]*;", text)
    if imported and len(re.findall(r"\bKINDS\b", text)) == 1:
        fixed = re.sub(r",\s*KINDS\b|\bKINDS,\s*", "", imported.group(0))
        text = text.replace(imported.group(0), fixed, 1)
    return text

for case in [a for a in sys.argv[1:] if not a.startswith("--")]:
    rs, feature = os.path.join(case, "🦀️.rs"), os.path.join(case, "🥒️.feature")
    if not (os.path.exists(rs) and os.path.exists(feature)): continue
    text = open(rs, encoding="utf-8").read()
    if not re.search(r"for &?kind in", text): continue
    bases, plain = outline_bases(open(feature, encoding="utf-8").read())
    new, problems = rewrite(text, bases, plain)
    if new != text:
        new = drop_unused_kinds(new)
    status = "changed" if new != text else "unchanged"
    print(f"{status:9} {'; '.join(problems) or '-':50.50} {case.split('🗿️artifacts/')[-1].split('/🧪️tests/')[-1]}")
    if write and new != text:
        open(rs, "w", encoding="utf-8").write(new)
