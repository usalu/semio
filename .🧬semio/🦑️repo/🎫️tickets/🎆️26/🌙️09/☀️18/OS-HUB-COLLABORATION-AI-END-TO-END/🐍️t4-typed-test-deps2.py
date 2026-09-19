#!/usr/bin/env python3
"""🧬️ Second pass: types a test harness bag whose values the caller owns internally or re-imports.

For each `registerTestsN` key the caller does not export, this resolves the key to the caller's own
`import { … } from "<spec>"` when one provides it, and otherwise exports the caller's local
declaration. The harness parameter then becomes an intersection of exact `Pick<typeof import(…)>`
slices — no `any`, no hand-written shapes that can drift from the values actually passed.
"""
import io, os, re, sys, importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("pass1", os.path.join(HERE, "\U0001f40d️t4-typed-test-deps.py"))
pass1 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pass1)

IMPORT_RE = re.compile(r'import\s*\{([^}]*)\}\s*from\s*"([^"]+)"')
LOCAL_DECL_RE = "^(const|let|function|async function|class) {name}\\b"
SKIP = {"\U0001f9ea️space-artifact-creation-owner"}

def caller_imports(text: str) -> dict:
    """\U0001f4e5️ Maps each imported binding name to the specifier that provides it."""
    out = {}
    for blk, spec_ in IMPORT_RE.findall(text):
        for part in blk.split(","):
            part = part.strip()
            if not part or part.startswith("type "):
                continue
            out[part.split(" as ")[-1].strip() if " as " in part else part] = spec_
    return out

def rebase(spec_: str, caller: str, target: str) -> str:
    """\U0001f9ed️ Rewrites a caller-relative specifier so it resolves from the test module."""
    if not spec_.startswith("."):
        return spec_
    abs_ = os.path.normpath(os.path.join(os.path.dirname(caller), spec_))
    rel = os.path.relpath(abs_, os.path.dirname(target))
    return rel if rel.startswith(".") else "./" + rel

def main(apply: bool) -> int:
    mapping = pass1.find_callers()
    changed = 0
    for (target, fn), (caller, keys) in sorted(mapping.items()):
        if not os.path.exists(target) or os.path.basename(os.path.dirname(target)) in SKIP:
            continue
        ttext = io.open(target, encoding="utf-8").read()
        sig = f'export async function {fn}(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any,'
        if sig not in ttext:
            continue
        ctext = io.open(caller, encoding="utf-8").read()
        exported = pass1.module_exports(caller)
        imports = caller_imports(ctext)
        groups, to_export, unresolved = {}, [], []
        own = rebase("./" + os.path.basename(caller), caller, target)
        for k in keys:
            if k in exported:
                groups.setdefault(own, []).append(k)
            elif k in imports:
                groups.setdefault(rebase(imports[k], caller, target), []).append(k)
            elif re.search(LOCAL_DECL_RE.format(name=re.escape(k)), ctext, re.M):
                to_export.append(k)
                groups.setdefault(own, []).append(k)
            else:
                unresolved.append(k)
        if unresolved:
            print(f"UNRESOLVED {os.path.basename(os.path.dirname(target))} {fn}: {unresolved[:6]}")
            continue
        typed = " & ".join(
            f'Pick<typeof import("{spec_}"), {" | ".join(sorted(chr(34) + k + chr(34) for k in ks))}>'
            for spec_, ks in sorted(groups.items()))
        ttext = ttext.replace(sig, sig.replace("dependencies: any,", f"dependencies: {typed},"))
        for k in to_export:
            ctext = re.sub(LOCAL_DECL_RE.format(name=re.escape(k)), r"export \1 " + k, ctext, count=1, flags=re.M)
        print(f"{os.path.basename(os.path.dirname(target))} {fn}: {len(groups)} source(s), exported {to_export}")
        changed += 1
        if apply:
            io.open(target, "w", encoding="utf-8").write(ttext)
            if to_export:
                io.open(caller, "w", encoding="utf-8").write(ctext)
    print(f"{'applied' if apply else 'would apply'} {changed} rewrites")
    return 0

if __name__ == "__main__":
    sys.exit(main("--apply" in sys.argv))
