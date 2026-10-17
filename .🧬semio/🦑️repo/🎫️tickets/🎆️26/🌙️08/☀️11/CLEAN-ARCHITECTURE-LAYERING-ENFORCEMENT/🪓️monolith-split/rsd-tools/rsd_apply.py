"""🛠️ Rewrites outdated retained-step constructor calls in one Rust file to the current owner signatures.

StepBudget::new(fuel, deadline)          -> StepBudget::new(fuel, deadline, <GRANT>)
StepContext::new(6 args)                 -> StepContext::new(6 args, &mut <progress>)

usage: rsd_apply.py <file> <grant-const-name> [--write]
Prints the planned edits; with --write rewrites the file (the caller re-reads it right before).
"""
import re, sys

NAME = sys.argv[2]
WRITE = "--write" in sys.argv
PATH = sys.argv[1]
SRC = open(PATH, encoding="utf-8").read()
PROGRESS = "semio_framework_job::RetainedCloneProgress"
GRANT_TYPE = "semio_framework_job::RetainedCloneGrant"


def lex_mask(s):
    n = len(s)
    mask = bytearray(n)
    i = 0
    while i < n:
        c = s[i]
        if s.startswith("//", i):
            j = s.find("\n", i)
            j = n if j < 0 else j
            mask[i:j] = b"\x01" * (j - i)
            i = j
        elif s.startswith("/*", i):
            depth = 1
            j = i + 2
            while j < n and depth:
                if s.startswith("/*", j):
                    depth += 1
                    j += 2
                elif s.startswith("*/", j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            mask[i:j] = b"\x01" * (j - i)
            i = j
        elif c == "r" and re.match(r'r#*"', s[i:i + 40]) and (i == 0 or not (s[i - 1].isalnum() or s[i - 1] == "_")):
            m = re.match(r'r(#*)"', s[i:])
            hashes = m.group(1)
            end = s.find('"' + hashes, i + len(m.group(0)))
            end = n if end < 0 else end + 1 + len(hashes)
            mask[i:end] = b"\x02" * (end - i)
            i = end
        elif c == '"':
            j = i + 1
            while j < n and s[j] != '"':
                j += 2 if s[j] == "\\" else 1
            j = min(j + 1, n)
            mask[i:j] = b"\x02" * (j - i)
            i = j
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", s[i:i + 12])
            if m:
                j = i + len(m.group(0))
                mask[i:j] = b"\x02" * (j - i)
                i = j
            else:
                i += 1
        else:
            i += 1
    return mask


MASK = lex_mask(SRC)


def args_of(open_idx):
    depth = 0
    angle = 0
    starts = [open_idx + 1]
    j = open_idx
    n = len(SRC)
    while j < n:
        c = SRC[j]
        if MASK[j]:
            j += 1
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0:
                return starts, j
        elif c == "," and depth == 1:
            starts.append(j + 1)
        j += 1
    return None, None


def arg_spans(starts, close_idx):
    spans = []
    for k, st in enumerate(starts):
        en = (starts[k + 1] - 1) if k + 1 < len(starts) else close_idx
        t = SRC[st:en]
        if t.strip() == "" and k + 1 == len(starts):
            continue
        spans.append((st, en))
    return spans


def trim_end(en, st):
    while en > st and SRC[en - 1].isspace():
        en -= 1
    return en


def line_start(i):
    return SRC.rfind("\n", 0, i) + 1


def indent_at(i):
    ls = line_start(i)
    m = re.match(r"[ \t]*", SRC[ls:])
    return m.group(0)


edits = []
notes = []
LET_RX = re.compile(r"\blet\s+(?:mut\s+)?([A-Za-z_]\w*)\s*(?::[^=;]*)?=\s*$")
for m in re.finditer(r"\b(StepContext|StepBudget)::new\s*\(", SRC):
    if MASK[m.start()]:
        notes.append(f"skip-masked@{SRC.count(chr(10), 0, m.start()) + 1}")
        continue
    kind = m.group(1)
    open_idx = m.end() - 1
    starts, close = args_of(open_idx)
    spans = arg_spans(starts, close)
    want = 7 if kind == "StepContext" else 3
    have = len(spans)
    line = SRC.count("\n", 0, m.start()) + 1
    if have == want:
        continue
    if have != want - 1:
        notes.append(f"unexpected-arg-count {kind} {have} @{line}")
        continue
    last_st, last_en = spans[-1]
    p = trim_end(last_en, last_st)
    multi = "\n" in SRC[starts[0]:p]
    spaced = ", " in SRC[starts[0]:p] or multi
    sep = ", " if spaced else ","
    if kind == "StepBudget":
        ins = sep + NAME
        edits.append((p, ins, "budget", line))
        continue
    pre = SRC[:m.start()]
    stripped = pre.rstrip()
    ctx_start = m.start()
    qual = re.search(r"((?:[A-Za-z_]\w*::)+)$", pre)
    if qual:
        ctx_start = m.start() - len(qual.group(1))
        pre = SRC[:ctx_start]
        stripped = pre.rstrip()
    lm = LET_RX.search(stripped[-200:])
    if lm:
        var = lm.group(1) + "_retained"
        let_pos = len(stripped) - len(stripped[-200:]) + lm.start()
        ls = line_start(let_pos)
        before = SRC[ls:let_pos]
        decl = f"let mut {var} = {PROGRESS}::default();"
        if before.strip() == "":
            edits.append((let_pos, decl + "\n" + before, "decl", line))
        else:
            edits.append((let_pos, decl + " ", "decl", line))
        arg = f"&mut {var}"
    else:
        arg = f"&mut {PROGRESS}::default()"
        notes.append(f"inline-temp StepContext @{line}: {SRC[max(0, ctx_start - 30):ctx_start].strip()[-30:]!r}")
    if multi:
        edits.append((p, ",\n" + indent_at(last_st) + arg, "ctx", line))
    else:
        edits.append((p, sep + arg, "ctx", line))

if not edits:
    print("no edits", notes)
    sys.exit(0)

def find_const_insertion():
    lines = SRC.split("\n")
    pos = 0
    last = 0
    in_use = False
    first_item = None
    offs = []
    for ln in lines:
        offs.append(pos)
        pos += len(ln) + 1
    for k, ln in enumerate(lines):
        if re.match(r"(pub(\([^)]*\))?\s+)?use\s", ln) or in_use:
            in_use = not ln.rstrip().endswith(";")
            last = k + 1
            continue
        if ln.startswith("//!") or ln.strip() == "" or ln.startswith("#![") :
            continue
        first_item = k
        break
    if last:
        return offs[last] if last < len(offs) else len(SRC)
    k = 0
    while k < len(lines) and (lines[k].startswith("//!") or lines[k].strip() == ""):
        k += 1
    return offs[k] if k < len(offs) else 0

const_ins = None
if "--no-const" not in sys.argv and any(e[2] == "budget" for e in edits) and not re.search(rf"\bconst {NAME}\b", SRC):
    at = find_const_insertion()
    text = (f"\n/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.\n"
            f"const {NAME}: {GRANT_TYPE} = {GRANT_TYPE} {{ maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }};\n")
    edits.append((at, text, "const", 0))

out = SRC
for pos, ins, kind, line in sorted(edits, key=lambda e: (e[0], 0 if e[2] != "decl" else 1), reverse=True):
    out = out[:pos] + ins + out[pos:]
print(f"{PATH}: {sum(1 for e in edits if e[2]=='ctx')} ctx, {sum(1 for e in edits if e[2]=='budget')} budget, {sum(1 for e in edits if e[2]=='decl')} decl, const={'yes' if any(e[2]=='const' for e in edits) else 'no'}")
for n in notes:
    print("  note:", n)
if WRITE:
    cur = open(PATH, encoding="utf-8").read()
    assert cur == SRC, "file changed under the rewriter"
    open(PATH, "w", encoding="utf-8").write(out)
