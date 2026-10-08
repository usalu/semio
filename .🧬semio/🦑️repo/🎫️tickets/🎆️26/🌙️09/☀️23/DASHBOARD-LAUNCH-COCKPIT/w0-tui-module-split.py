#!/usr/bin/env python3
"""✂️ Splits the TUI monolith `🖱️ui/⌨️tui/🦀️.rs` into one `<emoji>️<name>/🦀️.rs` file per module.

Ticket input of 26/09/23/DASHBOARD-LAUNCH-COCKPIT, slice w0 (structural, zero behaviour change).

Sub-commands (run from the repository root):
  snapshot   copy the live monolith into the ticket's generated folder (the copy every later step cuts from)
  plan       lex the snapshot, print every module span and every line the de-indent cannot invert
  write      write the module files, prove `inline(files) == snapshot` byte for byte, re-check the live
             monolith still equals the snapshot, then replace it once with the manifest
  verify     re-inline the live tree and compare it with the snapshot (bytes, then tokens); an optional
             second argument names another pre-split copy of the monolith to compare against
  inline     print the re-inlined monolith built from the live tree to stdout
"""
import hashlib
import os
import sys

ROOT = os.getcwd()
TUI = os.path.join(ROOT, "🧰️framework", "🔨️modules", "🖱️ui", "⌨️tui")
MONOLITH = os.path.join(TUI, "🦀️.rs")
TICKET = os.path.join(ROOT, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️23", "DASHBOARD-LAUNCH-COCKPIT")
GENERATED = os.path.join(TICKET, "🗑️generated", "tui-split")
SNAPSHOT = os.path.join(GENERATED, "monolith.before.rs")
INDENT = "    "
SOURCE_FILE = "🦀️.rs"

# module name -> (folder emoji without selector, region label of the old monolith, blank line after the region marker)
MODULES = [
    ("geometry", "📐", "Geometry", False),
    ("theme", "🎨", "Theme", False),
    ("text", "📝", "Text", False),
    ("cell", "🔲", "Cell", False),
    ("ansi", "🔡", "Ansi", False),
    ("vt", "📟", "Vt", False),
    ("event", "📡", "Event", False),
    ("scene", "🎬", "Scene", False),
    ("layout", "📏", "Layout", False),
    ("widget", "🪀", "Widget", True),
    ("chrome", "🖥", "Chrome", True),
    ("engine", "⚙", "Engine", False),
    ("backend", "🔌", "Backend", False),
    ("pty", "🚇", "Pty", False),
    ("host", "🏃", "WasmHost", False),
]
SELECTOR = "️"
# `include!` resolves against the including file, so the three backend test includes gain one `../`.
INCLUDE_REWRITES = {"backend": [('include!("🧪️tests/', 'include!("../🧪️tests/')]}


def folder(name):
    for module, emoji, _, _ in MODULES:
        if module == name:
            return emoji + SELECTOR + name
    raise KeyError(name)


def sha(data):
    return hashlib.sha256(data.encode("utf-8") if isinstance(data, str) else data).hexdigest()


def read(path):
    with open(path, encoding="utf-8", newline="") as handle:
        return handle.read()


def write_new(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "x", encoding="utf-8", newline="") as handle:
        handle.write(text)


# region Lexer
def is_ident(ch):
    return ch.isalnum() or ch == "_"


def lex(text):
    """🔬 Yields `(kind, start, end)` for every Rust token, comment and whitespace run of `text`."""
    i, n = 0, len(text)
    while i < n:
        ch = text[i]
        if ch in " \t\r\n":
            j = i
            while j < n and text[j] in " \t\r\n":
                j += 1
            yield ("ws", i, j)
            i = j
            continue
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            body = text[i:j]
            doc = (body.startswith("///") and not body.startswith("////")) or body.startswith("//!")
            yield ("doc" if doc else "comment", i, j)
            i = j
            continue
        if text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            body = text[i:j]
            doc = (body.startswith("/**") and not body.startswith("/***") and body != "/**/") or body.startswith("/*!")
            yield ("doc" if doc else "comment", i, j)
            i = j
            continue
        if is_ident(ch):
            j = i
            while j < n and is_ident(text[j]):
                j += 1
            word = text[i:j]
            if word in ("r", "br", "cr") and j < n and text[j] in '#"':
                k = j
                while k < n and text[k] == "#":
                    k += 1
                if k < n and text[k] == '"':
                    close = '"' + "#" * (k - j)
                    end = text.find(close, k + 1)
                    if end < 0:
                        raise ValueError(f"unterminated raw string at {i}")
                    yield ("string", i, end + len(close))
                    i = end + len(close)
                    continue
            if word in ("b", "c") and j < n and text[j] == '"':
                end = scan_string(text, j)
                yield ("string", i, end)
                i = end
                continue
            if word == "b" and j < n and text[j] == "'":
                end = scan_char(text, j)
                if end is None:
                    raise ValueError(f"bad byte literal at {i}")
                yield ("char", i, end)
                i = end
                continue
            yield ("word", i, j)
            i = j
            continue
        if ch == '"':
            end = scan_string(text, i)
            yield ("string", i, end)
            i = end
            continue
        if ch == "'":
            end = scan_char(text, i)
            if end is None:
                j = i + 1
                while j < n and is_ident(text[j]):
                    j += 1
                yield ("lifetime", i, j)
                i = j
            else:
                yield ("char", i, end)
                i = end
            continue
        yield ("punct", i, i + 1)
        i += 1


def scan_string(text, quote):
    j, n = quote + 1, len(text)
    while j < n:
        if text[j] == "\\":
            j += 2
        elif text[j] == '"':
            return j + 1
        else:
            j += 1
    raise ValueError(f"unterminated string at {quote}")


def scan_char(text, quote):
    n = len(text)
    if quote + 1 >= n:
        return None
    if text[quote + 1] == "\\":
        j = quote + 3
        while j < n and text[j] != "'":
            j += 1
        return j + 1
    if quote + 2 < n and text[quote + 2] == "'" and text[quote + 1] != "'":
        return quote + 3
    return None


def line_states(text):
    """🧭 For every line of `text`: the token kind that spans its first byte (`None` when a line starts fresh)."""
    starts = [0]
    for index, ch in enumerate(text):
        if ch == "\n":
            starts.append(index + 1)
    states = [None] * len(starts)
    cursor = 0
    for kind, start, end in lex(text):
        if kind == "ws":
            continue
        while cursor < len(starts) and starts[cursor] <= start:
            cursor += 1
        probe = cursor
        while probe < len(starts) and starts[probe] < end:
            states[probe] = kind
            probe += 1
    return states


def tokens(text):
    """🪙 The compiler-visible token texts: whitespace and plain comments dropped, doc comments kept."""
    return [text[start:end] for kind, start, end in lex(text) if kind not in ("ws", "comment")]
# endregion Lexer


# region Spans
def module_spans(text):
    """🗺 `name -> (open_line, close_line)` (0-based) for every top-level `pub mod name {` of the monolith."""
    lines = text.split("\n")
    states = line_states(text)
    offsets, total = [], 0
    for line in lines:
        offsets.append(total)
        total += len(line) + 1
    depth_at_line = [0] * len(lines)
    depth, cursor = 0, 0
    for kind, start, end in lex(text):
        while cursor < len(lines) and offsets[cursor] <= start:
            depth_at_line[cursor] = depth
            cursor += 1
        if kind == "punct":
            if text[start] == "{":
                depth += 1
            elif text[start] == "}":
                depth -= 1
    while cursor < len(lines):
        depth_at_line[cursor] = depth
        cursor += 1
    if depth != 0:
        raise ValueError(f"unbalanced braces: {depth}")
    spans = {}
    for index, line in enumerate(lines):
        if states[index] is None and depth_at_line[index] == 0 and line.startswith("pub mod ") and line.endswith(" {"):
            name = line[len("pub mod "):-2]
            close = index + 1
            while not (depth_at_line[close] == 1 and lines[close] == "}"):
                close += 1
            spans[name] = (index, close)
    return lines, states, spans


def dedent(lines, states, open_line, close_line):
    """⬅️ The module body one level shallower; returns the text plus the lines the inverse cannot restore."""
    out, anomalies = [], []
    for index in range(open_line + 1, close_line):
        line = lines[index]
        if states[index] == "string":
            out.append(line)
            anomalies.append((index + 1, "starts-inside-string", line))
        elif line == "":
            out.append(line)
        elif line.startswith(INDENT):
            rest = line[len(INDENT):]
            if rest == "":
                anomalies.append((index + 1, "whitespace-only", line))
            out.append(rest)
        else:
            out.append(line)
            anomalies.append((index + 1, "under-indented", line))
    return "\n".join(out) + "\n", anomalies


def indent(body):
    """➡️ Exact inverse of `dedent` for bodies without anomalies."""
    if not body.endswith("\n"):
        raise ValueError("module file must end with a newline")
    lines = body[:-1].split("\n")
    states = line_states(body[:-1])
    out = []
    for index, line in enumerate(lines):
        out.append(line if line == "" or states[index] == "string" else INDENT + line)
    return "\n".join(out) + "\n"
# endregion Spans


# region Manifest
def manifest(lines, spans):
    """📜 The root file after the cut: everything outside the module blocks, each block as one `#[path]` item."""
    out, index = [], 0
    by_open = {open_line: name for name, (open_line, _) in spans.items()}
    close_of = {name: close for name, (_, close) in spans.items()}
    while index < len(lines):
        line = lines[index]
        if index in by_open:
            name = by_open[index]
            out.append(f'#[path = "{folder(name)}/{SOURCE_FILE}"]')
            out.append(f"pub mod {name};")
            index = close_of[name] + 1
            continue
        if line.startswith("// #region ") or line.startswith("// #endregion "):
            index += 1
            if line.startswith("// #region ") and index < len(lines) and lines[index] == "" and index + 1 in by_open:
                index += 1
            continue
        out.append(line)
        index += 1
    return "\n".join(out)


def region_label(name):
    for module, _, label, _ in MODULES:
        if module == name:
            return label
    raise KeyError(name)


def inline(manifest_text, load):
    """🧵 Rebuilds the monolith: every `#[path] pub mod name;` of the manifest becomes `pub mod name { … }` again."""
    lines = manifest_text.split("\n")
    out, index = [], 0
    names = {module: (emoji, label, blank) for module, emoji, label, blank in MODULES}
    while index < len(lines):
        line = lines[index]
        hit = None
        for name in names:
            if line == f'#[path = "{folder(name)}/{SOURCE_FILE}"]' and index + 1 < len(lines) and lines[index + 1] == f"pub mod {name};":
                hit = name
                break
        if hit is None:
            if line == "#[cfg(test)]" and lines[index + 1].startswith('#[path = "🧪') and lines[index + 2] == "mod tests;":
                out.extend(["// #region ???Tests", line, lines[index + 1], lines[index + 2], "// #endregion ???Tests"])
                index += 3
                continue
            out.append(line)
            index += 1
            continue
        _, label, blank = names[hit]
        body = load(hit)
        for monolith_form, split_form in INCLUDE_REWRITES.get(hit, []):
            body = body.replace(split_form, monolith_form)
        lead = []
        while out and out[-1] != "" and (out[-1].startswith("///") or out[-1].startswith("#[")):
            lead.insert(0, out.pop())
        out.append(f"// #region ???{label}")
        if blank:
            out.append("")
        out.extend(lead)
        out.append(f"pub mod {hit} {{")
        out.extend(indent(body)[:-1].split("\n"))
        out.append("}")
        out.append(f"// #endregion ???{label}")
        index += 2
    return "\n".join(out)
# endregion Manifest


# region Commands
def build(snapshot_text):
    lines, states, spans = module_spans(snapshot_text)
    expected = [name for name, _, _, _ in MODULES]
    if list(spans) != expected:
        raise SystemExit(f"module roster drifted: {list(spans)} != {expected}")
    files, anomalies = {}, {}
    for name, (open_line, close_line) in spans.items():
        body, bad = dedent(lines, states, open_line, close_line)
        for monolith_form, split_form in INCLUDE_REWRITES.get(name, []):
            if body.count(monolith_form) == 0 or body.count(split_form) != 0:
                raise SystemExit(f"{name}: include rewrite `{monolith_form}` is not a clean one-way rewrite")
            body = body.replace(monolith_form, split_form)
        files[name], anomalies[name] = body, bad
    return lines, spans, files, anomalies, manifest(lines, spans)


def cmd_snapshot():
    os.makedirs(GENERATED, exist_ok=True)
    data = read(MONOLITH)
    with open(SNAPSHOT, "w", encoding="utf-8", newline="") as handle:
        handle.write(data)
    print(f"snapshot {len(data.encode('utf-8'))} bytes, {data.count(chr(10))} lines, sha256 {sha(data)}")


def cmd_plan():
    snapshot_text = read(SNAPSHOT)
    lines, spans, files, anomalies, manifest_text = build(snapshot_text)
    print(f"snapshot sha256 {sha(snapshot_text)} lines {snapshot_text.count(chr(10))}")
    for name, (open_line, close_line) in spans.items():
        body = files[name]
        print(f"{name:9} lines {open_line + 1:5}-{close_line + 1:5} -> {folder(name)}/{SOURCE_FILE} {body.count(chr(10)):5} lines {len(body.encode('utf-8')):7} bytes anomalies {len(anomalies[name])}")
        for number, kind, line in anomalies[name]:
            print(f"    {number}: {kind}: {line[:120]!r}")
    rebuilt = inline(manifest_text, lambda name: files[name])
    print(f"manifest {manifest_text.count(chr(10))} lines; inline(files) == snapshot: {rebuilt == snapshot_text} (sha256 {sha(rebuilt)})")
    print(f"tokens equal: {tokens(rebuilt) == tokens(snapshot_text)} ({len(tokens(snapshot_text))} tokens)")
    print("----- manifest -----")
    print(manifest_text, end="")


def cmd_write():
    snapshot_text = read(SNAPSHOT)
    lines, spans, files, anomalies, manifest_text = build(snapshot_text)
    rebuilt = inline(manifest_text, lambda name: files[name])
    if rebuilt != snapshot_text:
        raise SystemExit("refusing to write: inline(files) != snapshot")
    for name in files:
        path = os.path.join(TUI, folder(name), SOURCE_FILE)
        if os.path.exists(path):
            raise SystemExit(f"refusing to write: {path} already exists")
    for name, body in files.items():
        write_new(os.path.join(TUI, folder(name), SOURCE_FILE), body)
    on_disk = inline(manifest_text, lambda name: read(os.path.join(TUI, folder(name), SOURCE_FILE)))
    if on_disk != snapshot_text:
        raise SystemExit("module files written but they do not re-inline to the snapshot; root left untouched")
    live = read(MONOLITH)
    if live != snapshot_text:
        raise SystemExit("the live monolith changed since the snapshot; root left untouched, re-apply the delta first")
    staged = MONOLITH + ".split-staged"
    with open(staged, "x", encoding="utf-8", newline="") as handle:
        handle.write(manifest_text)
    os.replace(staged, MONOLITH)
    print(f"wrote {len(files)} module files and the manifest ({manifest_text.count(chr(10))} lines); snapshot sha256 {sha(snapshot_text)}")


def live_inline():
    return inline(read(MONOLITH), lambda name: read(os.path.join(TUI, folder(name), SOURCE_FILE)))


def cmd_verify():
    snapshot_text = read(sys.argv[2] if len(sys.argv) > 2 else SNAPSHOT)
    rebuilt = live_inline()
    print(f"snapshot   sha256 {sha(snapshot_text)} bytes {len(snapshot_text.encode('utf-8'))} lines {snapshot_text.count(chr(10))}")
    print(f"re-inlined sha256 {sha(rebuilt)} bytes {len(rebuilt.encode('utf-8'))} lines {rebuilt.count(chr(10))}")
    print(f"byte-identical: {rebuilt == snapshot_text}")
    normalised = lambda text: " ".join(text.split())
    print(f"whitespace-normalised identical: {normalised(rebuilt) == normalised(snapshot_text)}")
    before, after = tokens(snapshot_text), tokens(rebuilt)
    print(f"token streams identical: {before == after} ({len(before)} vs {len(after)} tokens)")
    split_tokens = tokens(read(MONOLITH))
    for name, _, _, _ in MODULES:
        split_tokens += tokens(read(os.path.join(TUI, folder(name), SOURCE_FILE)))
    print(f"tokens in split tree (manifest + {len(MODULES)} files): {len(split_tokens)}")
    if rebuilt != snapshot_text:
        raise SystemExit(1)


def main():
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    if command == "snapshot":
        cmd_snapshot()
    elif command == "plan":
        cmd_plan()
    elif command == "write":
        cmd_write()
    elif command == "verify":
        cmd_verify()
    elif command == "inline":
        sys.stdout.write(live_inline())
    else:
        raise SystemExit(__doc__)
# endregion Commands


if __name__ == "__main__":
    main()
