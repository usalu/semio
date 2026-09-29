#!/usr/bin/env python3
"""🧹️ U6 set B — remove the compiler-dead code the peer's OPC `xml_parts` migration left in stdio docx/xlsx (window 3, T2, ON TOP
of LB2 p5 `wp-lb2/lb2-p5-docx-xlsx-opc-reds.py`).

"Dead" is the compiler's verdict, never a guess: `analyze` reads the JSON diagnostics of two scratch checks (native lane, private
build-dir — `u6-scratch-check.sh`): the lib WITH `component-app-assembly` (every non-test use, editor/viewer included) and the
lib-test unit WITHOUT it (every test use that compiles). An item is removed only when BOTH units report it `dead_code`, or — for
feature-gated editor/viewer files the lib-test unit cannot see — when the lib reports it and no feature-gated `🧪️tests/` file names it.
`unused_imports` are applied from rustc's own machine-applicable suggestion. Rounds repeat (fixpoint) until neither unit reports a
docx/xlsx `dead_code`/`unused_imports` warning; the payload keeps every round in order, each edit computed on the tree the previous
rounds produced.

Every edit is span-keyed: the exact removed block (item + its docs/attributes, one adjoining blank line) must occur EXACTLY ONCE in
its file at that round, and must lie inside the recorded `//#region` (region guard). `--write` keeps byte backups under
`w3-backup/dead-docx-xlsx/`; `--revert` restores them.

Usage:
  u6-dead-docx-xlsx.py analyze <capture-prefix> --root <scratch> [--round N]   append one round from `<prefix>.lib.json/.tests.json`
  u6-dead-docx-xlsx.py count <capture-prefix>                                  `COUNT errors=… dead=…` of one round
  u6-dead-docx-xlsx.py [--dry-run | --write | --revert] [--root <repo root>] [--rounds N] [--backup <dir>]"""
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

ARGS = sys.argv[1:]
ROOT = Path(ARGS[ARGS.index("--root") + 1]) if "--root" in ARGS else Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent
PAYLOAD = HERE / "payload" / "dead-docx-xlsx.json"
BACKUP = Path(ARGS[ARGS.index("--backup") + 1]) if "--backup" in ARGS else HERE / "w3-backup" / "dead-docx-xlsx"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
CRATES = {"semio-s-artifact-stdio-docx": ART + "📜️docx/", "semio-s-artifact-stdio-xlsx": ART + "📕️xlsx/"}
GATED = ("/✏️editor/", "/👁️viewer/")
REGION = re.compile(r"^\s*//#region\s+(.*?)\s*$")
USE_HEAD = re.compile(r"^[ \t]*(pub(\([^)]*\))?[ \t]+)?use\b")
RUSTFMT_CONFIG = "/Users/ueli/Documents/semio/rustfmt.toml"
ENDREGION = re.compile(r"^\s*//#endregion\b")


def lex_end(text: str, start: int, stop_at_semicolon_only: bool) -> int:
    """🧮️ Index just past the item that begins at `start` (comments, strings, raw strings, chars vs lifetimes skipped)."""
    depth, i, n, opened = 0, start, len(text), False
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            i = text.find("\n", i)
            i = n if i < 0 else i
            continue
        if text.startswith("/*", i):
            nest, i = 1, i + 2
            while i < n and nest:
                if text.startswith("/*", i):
                    nest, i = nest + 1, i + 2
                elif text.startswith("*/", i):
                    nest, i = nest - 1, i + 2
                else:
                    i += 1
            continue
        raw = re.match(r"b?r(#*)\"", text[i : i + 40])
        if raw and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            close = '"' + raw.group(1)
            i = text.find(close, i + raw.end()) + len(close)
            continue
        if c == '"' or (c == "b" and text.startswith('b"', i) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_"))):
            i += 2 if c == "b" else 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            continue
        if c == "'":
            if text.startswith("\\", i + 1):
                i = text.find("'", i + 2) + 1
                continue
            if i + 2 < n and text[i + 2] == "'":
                i += 3
                continue
            i += 1
            continue
        if c in "([{":
            if c == "{" and depth == 0 and not stop_at_semicolon_only:
                opened = True
            depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0 and c == "}" and opened:
                return i + 1
        elif c == ";" and depth == 0:
            return i + 1
        i += 1
    raise ValueError(f"unterminated item at {start}")


def item_lines(lines: list, line: int) -> tuple:
    """📏️ (first, last) 0-based inclusive line range of the item whose identifier sits on `line`, docs and attributes included."""
    first = line
    while first > 0:
        s = lines[first - 1].strip()
        if (s.startswith("///") or s.startswith("#[") or (s.startswith("//") and not s.startswith("//#"))) and s:
            first -= 1
            continue
        if s.endswith(")]") and not s.startswith("#["):
            probe = first - 1
            while probe > 0 and not lines[probe].strip().startswith("#["):
                probe -= 1
            first = probe
            continue
        break
    head = line
    while head > first and not re.search(r"\b(fn|const|static|struct|enum|type|impl|trait|mod|union)\b", lines[head]):
        head -= 1
    text = "".join(lines)
    offset = sum(len(l) for l in lines[:head])
    kind = re.search(r"\b(fn|const|static|struct|enum|type|impl|trait|mod|union)\b", lines[head]).group(1)
    end = lex_end(text, offset, kind in ("const", "static", "type"))
    last = text.count("\n", 0, end - 1)
    if text[end:].split("\n", 1)[0].strip():
        raise ValueError(f"item end shares its line with more code at line {last + 1}")
    return first, last


def region_of(lines: list, index: int) -> str:
    """🧭️ Innermost `//#region` name enclosing line `index` (empty string: none)."""
    stack = []
    for i, l in enumerate(lines[:index]):
        if REGION.match(l):
            stack.append(REGION.match(l).group(1))
        elif ENDREGION.match(l) and stack:
            stack.pop()
    return stack[-1] if stack else ""


def blank(lines: list, i: int) -> bool:
    return 0 <= i < len(lines) and lines[i].strip() == ""


def closes(lines: list, i: int) -> bool:
    return 0 <= i < len(lines) and (lines[i].strip().startswith("}") or ENDREGION.match(lines[i]) is not None)


def opens(lines: list, i: int) -> bool:
    return i < 0 or (i < len(lines) and (lines[i].rstrip().endswith("{") or REGION.match(lines[i]) is not None))


def with_blank(lines: list, first: int, last: int) -> tuple:
    """🧽️ Widens a removed range by one adjoining blank line so no double blank / block-edge blank is left behind."""
    if blank(lines, last + 1) and (blank(lines, first - 1) or opens(lines, first - 1)):
        return first, last + 1
    if blank(lines, first - 1) and (closes(lines, last + 1) or last + 1 >= len(lines)):
        return first - 1, last
    return first, last


def unique_edit(text: str, old: str, new: str, lines: list, first: int) -> tuple:
    """🔑️ Extends `old`/`new` upward by whole lines until `old` occurs exactly once."""
    while text.count(old) != 1:
        first -= 1
        if first < 0:
            raise ValueError("block never becomes unique")
        old, new = lines[first] + old, lines[first] + new
    return old, new, first


def tidy(file: str, text: str, edits: list) -> str:
    """🧹️ Removes what the removals emptied: `impl … {}` blocks and `//#region`s holding nothing but blank lines."""
    changed = True
    while changed:
        changed = False
        cur = text.splitlines(keepends=True)
        for i, l in enumerate(cur):
            j = i + 1
            while j < len(cur) and cur[j].strip() == "":
                j += 1
            empty_impl = re.match(r"^\s*impl\b.*\{\s*$", l) and j < len(cur) and cur[j].strip() == "}" and len(cur[j]) - len(cur[j].lstrip()) == len(l) - len(l.lstrip())
            empty_region = REGION.match(l) and j < len(cur) and ENDREGION.match(cur[j])
            if not (empty_impl or empty_region):
                continue
            first, last = (item_lines(cur, i)[0], j) if empty_impl else (i, j)
            first, last = with_blank(cur, first, last)
            old = "".join(cur[first : last + 1])
            old_u, new_u, at = unique_edit(text, old, "", cur, first)
            edits.append({"file": file, "region": region_of(cur, at), "old": old_u, "new": new_u, "what": "tidy " + l.strip()[:100]})
            del cur[first : last + 1]
            text = "".join(cur)
            changed = True
            break
    return text


def diagnostics(path: Path) -> list:
    out = []
    for raw in path.read_text().splitlines():
        try:
            record = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if record.get("reason") == "compiler-message" and any(f"#{name}@" in record.get("package_id", "") for name in CRATES):
            out.append(record["message"])
    return out


ITEM_MESSAGE = re.compile(r"^(function|functions|method|methods|associated function|associated functions|associated items|associated constant|associated constants|constant|constants|static|enum|struct|type alias|trait|union|multiple) ")


def span_name(span: dict) -> str:
    text = span["text"][0]
    return text["text"][text["highlight_start"] - 1 : text["highlight_end"] - 1]


def count(prefix: str) -> int:
    """🔢️ `COUNT errors=<hard errors> dead=<removable items + imports + unresolved imports>` of one round (both units)."""
    errors = 0
    for unit in ("lib", "tests"):
        for m in diagnostics(Path(f"{prefix}.{unit}.json")):
            if m.get("level") == "error" and (m.get("code") or {}).get("code") != "E0432" and not re.match(r"(aborting due to|could not compile)", m["message"]):
                errors += 1
                print(f"  ERROR [{unit}] {m['rendered'].strip().splitlines()[0][:300]}")
    unresolved = sum(len(spans) for spans in unresolved_imports(prefix).values())
    lib, tests, _, _, kept, dead = classify(prefix) if errors == 0 else ({}, {}, {}, {}, [], -1)
    print(f"COUNT errors={errors} dead={dead + unresolved if errors == 0 else -1} kept={len(kept)} unresolved={unresolved} lib-warnings={len(lib)} test-warnings={len(tests)}")
    return 0


def norm(file_name: str) -> str:
    return os.path.normpath(file_name)


def dead_keys(messages: list) -> dict:
    keys = {}
    for m in messages:
        code = (m.get("code") or {}).get("code")
        if m.get("level") != "warning" or code not in ("dead_code", "unused_imports"):
            continue
        for span in m["spans"]:
            if span["is_primary"] and any(c in norm(span["file_name"]) for c in CRATES.values()):
                keys[(code, norm(span["file_name"]), span["line_start"], span["column_start"])] = (m, span)
    return keys


def gated_test_names(root: Path) -> str:
    corpus = []
    for base in CRATES.values():
        for dirpath, _, files in os.walk(root / base):
            if "🧪️tests" in dirpath and any(g.strip("/") in dirpath for g in GATED):
                corpus += [(Path(dirpath) / f).read_text() for f in files if f.endswith(".rs")]
    return "\n".join(corpus)


def classify(prefix: str) -> tuple:
    """⚖️ Splits one round's diagnostics into removable items (file → identifier lines), removable imports and kept warnings."""
    lib, tests = dead_keys(diagnostics(Path(prefix + ".lib.json"))), dead_keys(diagnostics(Path(prefix + ".tests.json")))
    gated_corpus = gated_test_names(ROOT)
    removals, imports, kept, removable = {}, {}, [], 0
    for key, (message, span) in lib.items():
        code, file, line, column = key
        is_gated = any(g in file for g in GATED)
        if code == "unused_imports":
            imports.setdefault(file, set()).add((span["byte_start"], span["byte_end"], key in tests or is_gated))
            removable += 1
            continue
        if not ITEM_MESSAGE.match(message["message"]) or re.search(r"\b(field|fields|variant|variants)\b", message["message"]):
            kept.append(("not an item (field/variant)", file, line, message["message"]))
            continue
        names = [span_name(span) for span in message["spans"]]
        name = names[0]
        if not is_gated and key not in tests:
            kept.append(("used by lib tests", file, line, message["message"]))
            continue
        if any(re.search(rf"\b{re.escape(n)}\b", gated_corpus) for n in names):
            kept.append(("named in a feature-gated test", file, line, message["message"]))
            continue
        removable += 1
        for span in message["spans"]:
            removals.setdefault(norm(span["file_name"]), set()).add(span["line_start"] - 1)
        if re.match(r"(enum|struct|type alias|trait|union) `", message["message"]):
            owner = norm(message["spans"][0]["file_name"])
            for index, text_line in enumerate((ROOT / owner).read_text().splitlines()):
                if re.match(rf"^\s*impl(<[^>]*>)?\s+([^{{]*\bfor\s+)?{re.escape(name)}\b[^{{]*\{{", text_line):
                    removals[owner].add(index)
    return lib, tests, removals, imports, kept, removable


def use_statement(text: str, start: int, end: int) -> tuple:
    """🧭️ `[start, end)` of the whole `use` statement (possibly multi-line) that contains the span `[start, end)`."""
    head = text.rfind("\n", 0, start) + 1
    while not USE_HEAD.match(text[head : text.find("\n", head)]):
        head = text.rfind("\n", 0, head - 1) + 1
    tail = text.find("\n", text.find(";", end))
    return head, len(text) if tail < 0 else tail + 1


def drop_use_names(statement: str, spans: list) -> str:
    """✂️ Removes each `(start, end, separators_included)` span from one `use` statement, then its separator and any group the
    removal emptied or left with one member (rustfmt's own normal form); an emptied statement becomes empty text."""
    for start, end, separated in sorted(set(spans), reverse=True):
        if separated:
            statement = statement[:start] + statement[end:]
            continue
        after = re.match(r"\s*,\s*", statement[end:])
        before = re.search(r",\s*$", statement[:start])
        if after:
            statement = statement[:start] + statement[end + after.end() :]
        elif before:
            statement = statement[: before.start()] + statement[end:]
        else:
            statement = statement[:start] + statement[end:]
    previous = None
    while previous != statement:
        previous = statement
        statement = re.sub(r"(?:\b[\w:]+)?::\{\s*\}\s*,?\s*", "", statement)
        statement = re.sub(r",(\s*)\}", r"\1}", statement)
        statement = re.sub(r"::\{\s*([\w]+(?:\s+as\s+\w+)?|\*)\s*,?\s*\}", r"::\1", statement)
    return "" if re.fullmatch(r"\s*(pub(\([^)]*\))?\s+)?use\s*(\{\s*\})?\s*;?\s*", statement) else statement


def rustfmt_statement(statement: str) -> str:
    """🎨️ Formats one `use` statement exactly as rustfmt formats it in place: nested modules reproduce its indentation."""
    if not statement:
        return statement
    depth = (len(statement) - len(statement.lstrip(" "))) // 4
    source = "".join(f"mod u6_{level} {{\n" for level in range(depth)) + statement.strip() + "\n" + "}\n" * depth
    formatted = subprocess.run(["rustfmt", "--edition", "2021", "--config-path", RUSTFMT_CONFIG, "--emit", "stdout"], input=source, capture_output=True, text=True, check=True).stdout
    lines = formatted.splitlines(keepends=True)
    return "".join(lines[depth : len(lines) - depth])


def unresolved_imports(prefix: str) -> dict:
    """🔗️ file → E0432 name spans (`byte_start`, `byte_end`) of both units: imports of items an earlier round removed."""
    spans = {}
    for unit in ("lib", "tests"):
        for m in diagnostics(Path(f"{prefix}.{unit}.json")):
            if m.get("level") == "error" and (m.get("code") or {}).get("code") == "E0432":
                for span in m["spans"]:
                    if span["is_primary"] and any(c in norm(span["file_name"]) for c in CRATES.values()):
                        spans.setdefault(norm(span["file_name"]), set()).add((span["byte_start"], span["byte_end"]))
    return spans


def analyze(prefix: str, round_no: int) -> int:
    lib, tests, removals, imports, kept, _ = classify(prefix)
    unresolved = unresolved_imports(prefix)
    edits = []
    for file in sorted(set(removals) | set(imports) | set(unresolved)):
        path = ROOT / file
        original = path.read_text()
        lines = original.splitlines(keepends=True)
        line_offset = [0]
        for l in lines:
            line_offset.append(line_offset[-1] + len(l))
        items = []
        for line in sorted(removals.get(file, ())):
            first, last = item_lines(lines, line)
            if not any(a <= first and last <= b for a, b in items):
                items = [r for r in items if not (first <= r[0] and r[1] <= last)] + [(first, last)]
        raw = original.encode()
        ops = [("item", line_offset[a], a, b) for a, b in items]
        for byte_start, byte_end, dead in imports.get(file, ()):
            start_char, end_char = len(raw[:byte_start].decode()), len(raw[:byte_end].decode())
            if not any(line_offset[a] <= start_char < line_offset[b + 1] for a, b in items):
                ops.append(("import", start_char, start_char, end_char, False, dead))
        for byte_start, byte_end in unresolved.get(file, ()):
            start_char, end_char = len(raw[:byte_start].decode()), len(raw[:byte_end].decode())
            if not any(line_offset[a] <= start_char < line_offset[b + 1] for a, b in items):
                ops.append(("import", start_char, start_char, end_char, False, True))
        statements = {}
        for op in [op for op in ops if op[0] == "import"]:
            statements.setdefault(use_statement(original, op[2], op[3]), []).append(op)
        ops = [op for op in ops if op[0] != "import"] + [("use", head, head, tail, spans) for (head, tail), spans in statements.items()]
        text = original
        for op in sorted(ops, key=lambda o: o[1], reverse=True):
            cur = text.splitlines(keepends=True)
            if op[0] == "item":
                first, last = with_blank(cur, op[2], op[3])
                old = "".join(cur[first : last + 1])
                old_u, new_u, at = unique_edit(text, old, "", cur, first)
                edits.append({"file": file, "region": region_of(cur, at), "old": old_u, "new": new_u, "what": cur[op[2]].strip()[:120] if cur[op[2]].strip() else cur[op[2] + 1].strip()[:120]})
                del cur[first : last + 1]
                text = "".join(cur)
                continue
            head, tail, spans = op[2], op[3], op[4]
            old = text[head:tail]
            new = drop_use_names(old, [(span[2] - head, span[3] - head, span[4]) for span in spans])
            test_only = sorted({text[span[2] : span[3]] for span in spans if not span[5]})
            if test_only:
                indent = old[: len(old) - len(old.lstrip(" "))]
                use_head = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([\w:]+?)(?:::\{|;)", old)
                use_path = use_head.group(1) if "{" in old else use_head.group(1).rsplit("::", 1)[0]
                names = test_only[0] if len(test_only) == 1 else "{" + ", ".join(test_only) + "}"
                new = (new if new.strip() else indent) + ("" if not new.strip() else indent) + f"#[cfg(test)]\n{indent}use {use_path}::{names};\n"
            new = rustfmt_statement(new)
            old_u, new_u, at = unique_edit(text, old, new, cur, text.count("\n", 0, head))
            edits.append({"file": file, "region": region_of(cur, at), "old": old_u, "new": new_u, "what": "import " + ", ".join(sorted(text[span[2] : span[3]].strip(" ,\n") for span in spans))[:100]})
            text = text[:head] + new + text[tail:]
        text = tidy(file, text, edits)
        path.write_text(text)
    payload = json.loads(PAYLOAD.read_text()) if PAYLOAD.exists() else {"rounds": []}
    payload["rounds"] = payload["rounds"][: round_no - 1] + [{"round": round_no, "capture": prefix, "edits": edits, "kept": kept}]
    PAYLOAD.parent.mkdir(parents=True, exist_ok=True)
    PAYLOAD.write_text(json.dumps(payload, ensure_ascii=False, indent=1) + "\n")
    if not edits:
        print(f"round {round_no}: no edit derivable from {prefix}")
        return 1
    print(f"round {round_no}: lib dead/unused {len(lib)}, lib-test {len(tests)}, unresolved imports {sum(len(v) for v in unresolved.values())}, edits {len(edits)} "
          f"({sum(1 for e in edits if not e['what'].startswith('import'))} items, {sum(1 for e in edits if e['what'].startswith('import'))} imports), kept {len(kept)}")
    for reason, file, line, message in kept:
        print(f"  KEPT {reason}: {file.rsplit('🪆️subsets/', 1)[-1]}:{line} {message}")
    return 0


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def apply(write: bool, rounds: int) -> int:
    payload = json.loads(PAYLOAD.read_text())
    states, problems, count = {}, 0, 0
    for r in payload["rounds"][: rounds or None]:
        for edit in r["edits"]:
            rel = edit["file"]
            text = states.get(rel)
            if text is None:
                text = (ROOT / rel).read_text()
            found = text.count(edit["old"])
            if found != 1:
                print(f"PROBLEM round {r['round']} {rel.rsplit('/🪆️subsets/', 1)[-1]}: block found {found}x: {edit['what']}")
                problems += 1
                continue
            at = text.index(edit["old"])
            region = region_of(text.splitlines(keepends=True), text.count("\n", 0, at))
            if region != edit["region"]:
                print(f"PROBLEM round {r['round']} {rel}: region guard {region!r} != {edit['region']!r}: {edit['what']}")
                problems += 1
                continue
            states[rel] = text[:at] + edit["new"] + text[at + len(edit["old"]) :]
            count += 1
    print(f"{'write' if write and not problems else 'dry-run'}: {len(payload['rounds'][: rounds or None])} rounds, {count} edits, {len(states)} files, {problems} problems")
    if write and not problems:
        BACKUP.mkdir(parents=True, exist_ok=True)
        manifest = {}
        for rel, text in states.items():
            (BACKUP / key(rel)).write_bytes((ROOT / rel).read_bytes())
            manifest[key(rel)] = rel
            (ROOT / rel).write_text(text)
        (BACKUP / "manifest.json").write_text(json.dumps({"root": str(ROOT), "files": manifest}, ensure_ascii=False, indent=1))
    return 1 if problems else 0


def revert() -> int:
    manifest = json.loads((BACKUP / "manifest.json").read_text())
    for k, rel in manifest["files"].items():
        (Path(manifest["root"]) / rel).write_bytes((BACKUP / k).read_bytes())
        print("restored", rel)
    return 0


if __name__ == "__main__":
    if ARGS and ARGS[0] == "count":
        sys.exit(count(ARGS[1]))
    if ARGS and ARGS[0] == "analyze":
        sys.exit(analyze(ARGS[1], int(ARGS[ARGS.index("--round") + 1]) if "--round" in ARGS else 1))
    if "--revert" in ARGS:
        sys.exit(revert())
    sys.exit(apply("--write" in ARGS, int(ARGS[ARGS.index("--rounds") + 1]) if "--rounds" in ARGS else 0))
