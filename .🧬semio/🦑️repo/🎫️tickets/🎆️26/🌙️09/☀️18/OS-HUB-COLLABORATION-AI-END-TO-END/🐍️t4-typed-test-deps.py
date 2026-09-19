#!/usr/bin/env python3
"""🧪️ Replaces `dependencies: any` in extracted test modules with the caller's exact export type.

Every `registerTestsN(import.meta.vitest, { a, b, c }, …)` call site names the precise values its
test module receives. This derives `Pick<typeof import("<caller>"), "a" | "b" | "c">` from that call
site, so the harness parameter carries the real types instead of `any`.
"""
import io, os, re, sys, subprocess

ROOT = os.path.abspath(".")
OS_DIR = "🧰️framework/🛍️products/💻️os"
IMPORT_RE = re.compile(r'const \{ (registerTests\d*) \} = await import\("([^"]+)"\)')
CALL_RE = re.compile(r'await (registerTests\d*)\(import\.meta\.vitest, \{([^}]*)\}')

EXPORT_DECL_RE = re.compile(r"export\s+(?:async\s+)?(?:const|let|function|class|type|interface|enum)\s+([A-Za-z_$][A-Za-z0-9_$]*)")
EXPORT_BLOCK_RE = re.compile(r"export\s*\{([^}]*)\}")

def module_exports(path: str) -> set:
    """📤️ Collects every name a module exports, across declaration and clause forms."""
    text = io.open(path, encoding="utf-8").read()
    names = set(EXPORT_DECL_RE.findall(text))
    for blk in EXPORT_BLOCK_RE.findall(text):
        for part in blk.split(","):
            part = part.strip()
            if not part:
                continue
            part = re.sub(r"^type\s+", "", part)
            names.add(part.split(" as ")[-1].strip() if " as " in part else part)
    return names

def find_callers():
    """🔎️ Maps each (test module, register fn) to its caller module and destructured key list."""
    out = subprocess.run(
        ["grep", "-rl", "registerTests", "--include=🟦️.ts", "--include=🟦️.tsx", OS_DIR],
        capture_output=True, text=True).stdout.split("\n")
    mapping = {}
    for caller in out:
        if not caller or "/dist/" in caller:
            continue
        text = io.open(caller, encoding="utf-8").read()
        pending = {}
        for line in text.split("\n"):
            m = IMPORT_RE.search(line)
            if m:
                pending[m.group(1)] = m.group(2)
            c = CALL_RE.search(line)
            if c and c.group(1) in pending:
                rel = pending[c.group(1)]
                target = os.path.normpath(os.path.join(os.path.dirname(caller), rel))
                keys = [k.strip() for k in c.group(2).split(",") if k.strip() and ":" not in k]
                if keys:
                    mapping[(target, c.group(1))] = (caller, keys)
    return mapping

def main(apply: bool) -> int:
    mapping = find_callers()
    changed = 0
    by_file = {}
    for (target, fn), (caller, keys) in mapping.items():
        by_file.setdefault(target, []).append((fn, caller, keys))
    for target, entries in sorted(by_file.items()):
        if not os.path.exists(target):
            print(f"SKIP missing test module {target}")
            continue
        text = io.open(target, encoding="utf-8").read()
        original = text
        for fn, caller, keys in entries:
            sig = f"export async function {fn}(vitest: NonNullable<ImportMeta[\"vitest\"]>, dependencies: any,"
            if sig not in text:
                continue
            exported = module_exports(caller)
            missing = [k for k in keys if k not in exported]
            if missing:
                print(f"DEFER {target}\n    {fn}: caller keeps {len(missing)}/{len(keys)} values module-internal, e.g. {missing[:4]}")
                continue
            rel = os.path.relpath(caller, os.path.dirname(target))
            if not rel.startswith("."):
                rel = "./" + rel
            union = " | ".join(f'"{k}"' for k in keys)
            typed = f"Pick<typeof import(\"{rel}\"), {union}>"
            text = text.replace(sig, sig.replace("dependencies: any,", f"dependencies: {typed},"))
            print(f"{target}\n    {fn} <- {rel}  ({len(keys)} keys)")
            changed += 1
        if apply and text != original:
            io.open(target, "w", encoding="utf-8").write(text)
    print(f"{'applied' if apply else 'would apply'} {changed} signature rewrites")
    return 0

if __name__ == "__main__":
    sys.exit(main("--apply" in sys.argv))
