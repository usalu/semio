"""🧯️ FH1 codemod: every census `app-raise-form` site `…Fault::new(<origin>, …FaultCode::new("code"), <message>)` of the
given owners becomes `…app_fault("code")` (the message is dropped: the person reads the DECLARED text). Imports are
kept tidy: a bare `Fault::new` gains `app_fault` in the file's existing `use semio_framework_plugin::{…}` /
`use semio_framework::{…}` list (else the call is written crate-qualified), and `FaultCode`/`FaultOrigin` leave a use
list once nothing else names them. Prints every rewritten site with its dropped message (the en stub source).
usage: python3 fh1-raise.py <family A|H> <owner-substring…> [--dry-run]"""
import json, re, sys

O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
family = sys.argv[1]
dry = "--dry-run" in sys.argv
owners = [a for a in sys.argv[2:] if a != "--dry-run"]
census = json.load(open(f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census/family-{family}.json"))


def string_end(code, i):
    m = re.match(r'r(#*)"', code[i:i + 40])
    if m and (i == 0 or not re.match(r"\w", code[i - 1])):
        close = '"' + m.group(1)
        stop = code.find(close, i + len(m.group(0)))
        return stop + len(close)
    if code[i] != '"':
        return None
    j = i + 1
    while j < len(code):
        if code[j] == "\\":
            j += 2
            continue
        if code[j] == '"':
            return j + 1
        j += 1
    return None


def args_of(code, open_idx):
    depth, start, j, args = 0, open_idx + 1, open_idx + 1, []
    while j < len(code):
        c = code[j]
        if c == '"' or (c == "r" and re.match(r'r#*"', code[j:j + 40]) and not re.match(r"\w", code[j - 1])):
            end = string_end(code, j)
            if end:
                j = end
                continue
        if c == "'":
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'", code[j:j + 16])
            if m:
                j += len(m.group(0))
                continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                args.append(code[start:j])
                return [a.strip() for a in args if a.strip()], j
            depth -= 1
        elif c == "," and depth == 0:
            args.append(code[start:j])
            start = j + 1
        j += 1
    raise ValueError("unclosed call")


sites = {}
for v in census["violations"]:
    if v["rule"] == "app-raise-form" and v["detail"].startswith("Fault::new") and any(o in v["path"] for o in owners):
        sites.setdefault(v["path"], set()).add(v["line"])

for path, lines in sorted(sites.items()):
    text = open(O + path).read()
    line_starts = [0] + [m.end() for m in re.finditer(r"\n", text)]
    edits = []
    for line in sorted(lines):
        lo, hi = line_starts[line - 1], line_starts[line] if line < len(line_starts) else len(text)
        for m in re.finditer(r"(?<![\w:])((?:[A-Za-z_]\w*::)*)Fault::new\s*\(", text[lo:hi]):
            start = lo + m.start()
            args, close = args_of(text, lo + m.end() - 1)
            code_m = re.fullmatch(r'(?:[A-Za-z_]\w*::)*FaultCode::new\s*\(\s*"([^"]+)"\s*\)', args[1]) if len(args) == 3 else None
            if not code_m or not re.fullmatch(r"(?:[A-Za-z_]\w*::)*FaultOrigin::\w+", args[0]):
                print(f"SKIP {path}:{line} {text[start:close + 1][:200]}")
                continue
            edits.append((start, close + 1, m.group(1), code_m.group(1), args[2], line))
    bare = any(edit[2] == "" for edit in edits)
    has_import = re.search(r"^\s*(?:pub\s+)?use\s+[^;]*\bapp_fault\b[^;]*;", text, re.M) is not None
    use_list = re.search(r"^\s*use\s+(semio_framework_plugin|semio_framework)::\{([^}]*)\};", text, re.M)
    for start, end, prefix, code, message, line in sorted(edits, reverse=True):
        call = f'{prefix}app_fault("{code}")' if prefix or has_import or use_list else f'semio_framework_plugin::app_fault("{code}")'
        text = text[:start] + call + text[end:]
        print(f"{path.split('🔌️plugins/')[-1]}:{line} {code} <= {' '.join(message.split())[:220]}")
    if bare and not has_import and use_list:
        items = use_list.group(2)
        text = text[:use_list.start(2)] + ("app_fault, " + items if not items.startswith(" ") else " app_fault," + items) + text[use_list.end(2):]
    for name in ("FaultCode", "FaultOrigin"):
        body = re.sub(r"^\s*(?:pub\s+)?use\s+[^;]*;", "", text, flags=re.M)
        if re.search(rf"\b{name}\b", body):
            continue
        text = re.sub(rf"^(\s*use\s+[^;]*\{{[^}}]*?)\b{name}\b\s*,\s*", r"\1", text, flags=re.M)
        text = re.sub(rf"^(\s*use\s+[^;]*\{{[^}}]*?),\s*\b{name}\b(\s*\}})", r"\1\2", text, flags=re.M)
        text = re.sub(rf"^\s*use\s+(?:[A-Za-z_]\w*::)+{name};\n", "", text, flags=re.M)
    if not dry:
        open(O + path, "w").write(text)
