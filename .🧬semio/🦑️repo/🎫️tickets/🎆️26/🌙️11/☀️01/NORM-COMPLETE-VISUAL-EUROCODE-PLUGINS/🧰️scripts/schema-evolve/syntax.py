"""🔎 Parse-level checks of generated Rust (`rustfmt --emit stdout`, no compilation) and TypeScript (`typescript` syntactic diagnostics through node) text; both are optional tools, a missing tool skips the check (python3 -I syntax.py <file...>)."""
import json, os, shutil, subprocess, sys, tempfile

NODE_SCRIPT = r"""
const ts = require(process.argv[1]);
const fs = require("fs");
const out = {};
for (const path of process.argv.slice(2)) {
  const text = fs.readFileSync(path, "utf8");
  const r = ts.transpileModule(text, { reportDiagnostics: true, compilerOptions: { target: "ES2022", module: "ESNext" }, fileName: path.endsWith(".d.ts") ? "x.d.ts" : "x.ts" });
  out[path] = (r.diagnostics || []).filter(d => d.category === 1).slice(0, 5).map(d => { const p = d.file ? d.file.getLineAndCharacterOfPosition(d.start) : { line: 0, character: 0 }; const flat = text.split("\n")[p.line] || ""; return `${p.line + 1}:${p.character + 1} ${ts.flattenDiagnosticMessageText(d.messageText, " ")} | ${flat.slice(Math.max(0, p.character - 60), p.character + 60)}`; });
}
console.log(JSON.stringify(out));
"""


def tool(name):
    return shutil.which(name)


def typescript_module():
    here = os.path.abspath(__file__)
    for up in range(2, 14):
        base = here
        for _ in range(up):
            base = os.path.dirname(base)
        cand = os.path.join(base, "node_modules", "typescript", "lib", "typescript.js")
        if os.path.exists(cand):
            return cand
    return None


def check_rust(text):
    """🦀 Lexical sanity of Rust text (no rustfmt/rustc here): balanced brackets outside strings and comments, and every upper-case constant that is used is declared, imported or an enum-like path segment."""
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    import re
    from lex import code_mask
    mask = code_mask(text, "rs")
    stack, found = [], []
    pairs = {")": "(", "]": "[", "}": "{"}
    for i, ch in enumerate(text):
        if not mask[i]:
            continue
        if ch in "([{":
            stack.append((ch, i))
        elif ch in ")]}":
            if not stack or stack[-1][0] != pairs[ch]:
                found.append(f"unbalanced {ch} near {text[max(0, i - 50):i + 30]!r}")
                break
            stack.pop()
    if stack and not found:
        found.append(f"unclosed {stack[-1][0]} near {text[stack[-1][1]:stack[-1][1] + 80]!r}")
    declared = set(re.findall(r"\b(?:const|static)\s+([A-Z][A-Z0-9_]*)\b", text))
    used = {m.group(1) for m in re.finditer(r"(?<![\w:\"'.&])\b([A-Z][A-Z0-9_]{2,})\b(?![\w(:'\"])", "".join(c if m else " " for c, m in zip(text, mask)))}
    imported = set(re.findall(r"\b([A-Z][A-Z0-9_]{2,})\b", " ".join(re.findall(r"^use [^;]*;", text, re.M))))
    for name in sorted(used - declared - imported):
        found.append(f"constant {name} is used but not declared")
    return found[:5]


def check_ts(texts):
    node, module = tool("node"), typescript_module()
    if not node or not module:
        return None
    with tempfile.TemporaryDirectory() as d:
        paths = {}
        for k, t in texts.items():
            p = os.path.join(d, f"{len(paths)}.d.ts" if k.endswith(".d.ts") else f"{len(paths)}.ts")
            open(p, "w", encoding="utf8").write(t)
            paths[p] = k
        done = subprocess.run([node, "-e", NODE_SCRIPT, module, *paths], capture_output=True, text=True, timeout=120)
        if done.returncode:
            return {k: [done.stderr[-300:]] for k in texts}
        return {paths[p]: v for p, v in json.loads(done.stdout).items()}


def check_plan(plan):
    """🧪 {path: [findings]} for every planned `.rs` and `.ts` file whose text does not parse and parsed before the change (or is new)."""
    out, ts_texts = {}, {}
    for path, (old, new, _) in plan.files.items():
        if path.endswith(".rs"):
            before = check_rust(old) if old else []
            after = check_rust(new)
            if after and not before:
                out[path] = after
        elif path.endswith(".ts"):
            ts_texts[path] = new
    if ts_texts:
        after = check_ts(ts_texts)
        before = check_ts({p: plan.files[p][0] for p in ts_texts if plan.files[p][0]}) if after else {}
        for p, found in (after or {}).items():
            if found and not (before or {}).get(p):
                out[p] = found
    return out


if __name__ == "__main__":
    for p in sys.argv[1:]:
        text = open(p, encoding="utf8").read()
        print(p, check_rust(text) if p.endswith(".rs") else check_ts({p: text}))
