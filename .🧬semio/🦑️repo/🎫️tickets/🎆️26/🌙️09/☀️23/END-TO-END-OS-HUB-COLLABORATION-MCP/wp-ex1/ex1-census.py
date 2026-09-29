#!/usr/bin/env python3
"""📚️ EX1 census: every example loader in `✏️s/🔌️plugins/**` whose example payload decode or example-id resolution falls back
silently (empty/genesis/other document, or a silent no-op) instead of loading for real or refusing by code.

Usage: python3 ex1-census.py [--root <repo>] [--json <out.json>] [--md <out.md>]

Scans every non-test source (Rust `.rs`; TS/JS/Python are scanned for runtime loaders too). A statement = the matched line plus
the following lines up to its end (`;`, `,` or a closing brace at the statement's depth, max 6 lines). Categories (exact regexes
in `PATTERNS`):
  L1 decode-fallback   an example decode whose Err arm yields a document (`unwrap_or_default`, `unwrap_or_else(|_| empty…)`,
                       `or_else(|_| parse other)`, a `.ok()` feeding `unwrap_or…`)
  L2 decode-noop       an example decode whose Err arm is a silent no-op (`.ok()` → None → `Emit::default()`, `let Ok … else`)
  L3 unknown-genesis   an unknown (non-empty) example id loads the genesis / empty / default document
  L4 unknown-noop      an unknown example id is a silent no-op (`Ok(Emit::default())`, `None => …default()`, `if let Some`)
  L5 untyped-refusal   refuses, but with free text (`Fault::from`, `format!` message) — row 12 converts these; EX1 unifies
  L6 missing-id        `exampleId` absent → "" / a default id, silently
  L7 own-codes         per-plugin example refusal codes duplicating the ONE resolver's `app.example.*`
  P  law-guarded       an example decode that panics (`expect`/`panic!`) — loud, not a fallback; kept, guarded by the law
"""
import argparse, json, os, re, sys

TEST_DIR = "🧪️tests"
EXAMPLE_REF = re.compile(r"PRIMARY_TEXT|EXAMPLE|examples::|example_snapshot|example_document|example_text|fixture_dsl_for_preset|DSL_TEXT|FIXTURE_DSL|document_json\(\)|\.document\(\)|example_catalogue|example_model|_EXAMPLE_JSON")
DECODE = r"(?:parse_dsl|from_json_str|decode_\w+_dsl|example_snapshot|example_document|example_text|fixture_dsl_for_preset|example_model)\s*\("
PATTERNS = [
    ("L1", "decode-unwrap-or-default", re.compile(DECODE + r"[\s\S]*?\)\s*\.unwrap_or_default\(\)")),
    ("L1", "decode-unwrap-or-else-document", re.compile(DECODE + r"[\s\S]*?\)\s*\.unwrap_or_else\(\s*(?:\|_\|\s*)?(?!\|error\|)[\w:]*(?:empty|default)\w*")),
    ("L1", "decode-or-else-other", re.compile(DECODE + r"[\s\S]*?\)\s*\.or_else\(\|_\|\s*parse_dsl")),
    ("L2", "decode-ok-option", re.compile(DECODE + r"[^;]*?\)\s*\.ok\(\)")),
    ("L2", "decode-let-else-noop", re.compile(r"let\s+(?:Ok|Some)\([^)]*\)\s*=\s*[^;]*?(?:parse_dsl|example_snapshot|example_text|example_document)[^;]*?else\s*\{\s*return\s+Ok\(Emit::default\(\)\)")),
    ("L2", "decode-err-noop", re.compile(r"Err\(_\)\s*=>\s*(?:Ok\()?Emit::default\(\)")),
    ("L3", "unknown-else-default", re.compile(r"\}\s*else\s*\{\s*(?:<?[A-Z]\w*Snapshot>?::default\(\)|[\w:]*empty_\w+\(\)|[\w:]*default_\w*snapshot\(\))\s*\}")),
    ("L3", "unknown-arm-default", re.compile(r"_\s*=>\s*(?:&EMPTY|[\w:]*empty_\w+\(\)|<?[A-Z]\w*Snapshot>?::default\(\))\s*,")),
    ("L4", "unknown-return-noop", re.compile(r"(?:return\s+Ok\(Emit::default\(\)\)|None\s*=>\s*(?:Ok\()?Emit::default\(\)|_\s*=>\s*return\s+Ok\(Emit::default\(\)\)|\}\s*else\s*\{\s*Ok\(Emit::default\(\)\)\s*\})")),
    ("L4", "unknown-arm-none", re.compile(r"_\s*=>\s*None\s*,?\s*$")),
    ("L5", "untyped-refusal", re.compile(r"Fault::from\((?:format!|\"|error|&)")),
    ("L6", "missing-id-default", re.compile(r"(?:\"exampleId\"|example_id)[^;\n]*?(?:\.unwrap_or_default\(\)|\.unwrap_or\(\"\"\)|\.unwrap_or_else\(\|\|\s*[\w:]*(?:ID|DEFAULT)\w*)|example_id_argument\(args,\s*\"\"\)")),
    ("L7", "own-example-code", re.compile(r"FaultCode::new\(\"[\w./-]*(?:example|template)[\w./-]*\"\)|app_fault\(\"[\w./-]*(?:example|template)[\w./-]*\"\)|Fault::from\(\"[\w./-]*(?:example|template)[\w./-]*\"\)")),
    ("P", "law-guarded-panic", re.compile(DECODE + r"[\s\S]*?\)\s*\.(?:expect\(|unwrap_or_else\(\|error\|\s*panic!)")),
]
HANDLER_FILE = re.compile(r"set-active-example|📚️example|🎨️example|📚️examples|🖥️app-surface")
EXAMPLE_FILE = re.compile(r"set-active-example|📚️example|🎨️example|examples|🎮️commands/📄️document|/📝️text/|🧬️schema/🦀️\.rs|✏️editor/🦀️\.rs|👁️viewer/🦀️\.rs|🖥️app-surface")


def non_test_extent(lines):
    """✂️ Index of the first line of a trailing `#[cfg(test)] mod x {` block (or the file end)."""
    for i, line in enumerate(lines):
        if re.match(r"\s*#\[cfg\(test\)\]\s*$", line) and i + 1 < len(lines) and re.match(r"\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", lines[i + 1]):
            return i
    return len(lines)


def statement(lines, i, stop):
    """🧾️ The statement starting at line `i` (max 6 lines)."""
    out, depth = [], 0
    for j in range(i, min(stop, i + 6)):
        out.append(lines[j])
        depth += lines[j].count("(") + lines[j].count("{") + lines[j].count("[") - lines[j].count(")") - lines[j].count("}") - lines[j].count("]")
        if depth <= 0 and re.search(r"[;,{}]\s*$", lines[j]):
            break
    return "\n".join(out)


def scan(root):
    plugins = os.path.join(root, "✏️s", "🔌️plugins")
    sites, other_languages = [], []
    for dp, dns, fns in os.walk(plugins):
        dns[:] = [d for d in dns if d not in ("node_modules", "target", TEST_DIR) and not d.startswith(".")]
        for fn in fns:
            path = os.path.join(dp, fn)
            rel = os.path.relpath(path, root)
            if fn.endswith((".ts", ".tsx", ".js", ".mjs", ".py")):
                text = open(path, encoding="utf-8", errors="replace").read()
                if re.search(r"setActiveExample|PRIMARY_TEXT|exampleId", text) and not fn.startswith(("📜️script", "🧪️")) and "🎚️config" not in rel:
                    other_languages.append(rel)
                continue
            if not fn.endswith(".rs"):
                continue
            lines = open(path, encoding="utf-8").read().split("\n")
            stop = non_test_extent(lines)
            seen = set()
            for i in range(stop):
                window = statement(lines, i, stop)
                raw = "\n".join(lines[i:min(stop, i + 3)])
                first = lines[i]
                for cat, rule, rx in PATTERNS:
                    m = rx.search(raw if cat == "L3" else window)
                    if not m or m.start() >= len(first):
                        continue
                    if re.match(r"\s*(pub(\(crate\))?\s+)?fn\s", first):
                        continue
                    if cat in ("L1", "L2", "P") and not EXAMPLE_REF.search(window[: m.end()]) and not (HANDLER_FILE.search(rel) and EXAMPLE_REF.search("\n".join(lines[max(0, i - 8):min(stop, i + 3)]))):
                        continue
                    if cat in ("L3", "L4", "L6") and not EXAMPLE_FILE.search(rel):
                        continue
                    if cat == "L5" and not HANDLER_FILE.search(rel):
                        continue
                    lo = max(0, i - 12)
                    context = "\n".join(lines[lo:min(stop, i + 6)])
                    if cat in ("L3", "L4", "L5") and not EXAMPLE_REF.search(context) and "example" not in context.lower():
                        continue
                    if cat == "L6" and "example" not in first.lower():
                        continue
                    key = (i, cat)
                    if key in seen:
                        continue
                    seen.add(key)
                    parts = rel.split("/")
                    artifact = parts[4] if len(parts) > 4 and parts[3] == "🗿️artifacts" else parts[3]
                    sites.append({"plugin": parts[2], "artifact": artifact, "file": rel, "line": i + 1, "category": cat, "rule": rule, "text": first.strip()[:240]})
    return sites, other_languages


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default="/Users/ueli/Documents/semio")
    ap.add_argument("--json")
    ap.add_argument("--md")
    a = ap.parse_args()
    sites, other = scan(a.root)
    by = {}
    for s in sites:
        by.setdefault(s["category"], 0)
        by[s["category"]] += 1
    result = {"root": a.root, "patterns": [{"category": c, "rule": r, "regex": x.pattern} for c, r, x in PATTERNS], "counts": by, "otherLanguageRuntimeLoaders": other, "sites": sites}
    if a.json:
        json.dump(result, open(a.json, "w"), ensure_ascii=False, indent=1)
    print(json.dumps(by, sort_keys=True), "sites", len(sites), "other-language", len(other))


if __name__ == "__main__":
    main()
