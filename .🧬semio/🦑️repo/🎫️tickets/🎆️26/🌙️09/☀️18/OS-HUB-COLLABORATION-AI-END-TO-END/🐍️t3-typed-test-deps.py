#!/usr/bin/env python3
"""🧪️ Types the `dependencies` bag of `🧰️framework/🔨️modules` extracted test suites from its call site.

Every `registerTestsN(import.meta.vitest, { a, b, c }, …)` call site names the exact values its test
module receives. This derives the parameter type from that call site instead of `any`:

* a key the caller module exports    → `Pick<typeof import("<caller>"), "key">`
* a key the caller re-imports        → `Pick<typeof import("<rebased spec>"), "key">`
* an aliased / namespace / default import → an explicit member so the alias cannot drift
* a key the caller declares locally  → the declaration gains `export` and joins the caller's Pick

The parts are intersected, so nothing is widened and no hand-written shape can drift from the values
actually passed. A key that resolves to none of the above aborts that signature and is reported.

Usage: python3 🐍️t3-typed-test-deps.py [--apply] [<path substring filter> …]
"""
import io
import os
import re
import subprocess
import sys

SCOPE = "🧰️framework/🔨️modules"
IMPORT_BINDING = re.compile(r'const \{ (registerTests\d*) \} = await import\("([^"]+)"\)')
CALL = re.compile(r'await (registerTests\d*)\(import\.meta\.vitest, \{([^}]*)\}')
EXPORT_DECL = re.compile(r"export\s+(?:async\s+)?(?:const|let|var|function|class|type|interface|enum)\s+([A-Za-z_$][\w$]*)")
EXPORT_CLAUSE = re.compile(r"export\s*\{([^}]*)\}")
NAMED_IMPORT = re.compile(r'import\s+(?:type\s+)?\{([^}]*)\}\s*from\s*"([^"]+)"')
NAMESPACE_IMPORT = re.compile(r'import\s+\*\s+as\s+([A-Za-z_$][\w$]*)\s+from\s*"([^"]+)"')
DEFAULT_IMPORT = re.compile(r'^import\s+([A-Za-z_$][\w$]*)\s*(?:,\s*\{[^}]*\})?\s+from\s*"([^"]+)"', re.M)
LOCAL_DECL = "^(const|let|var|async function|function|class) {name}\\b"


def module_exports(text):
    """📤️ Every name a module exports, across declaration and clause forms."""
    names = set(EXPORT_DECL.findall(text))
    for block in EXPORT_CLAUSE.findall(text):
        for part in block.split(","):
            part = re.sub(r"^type\s+", "", part.strip())
            if part:
                names.add(part.split(" as ")[-1].strip())
    return names


def module_imports(text):
    """📥️ Maps each imported local binding to `(specifier, exported name or "*" or "default")`."""
    bindings = {}
    for block, spec in NAMED_IMPORT.findall(text):
        for part in block.split(","):
            part = re.sub(r"^type\s+", "", part.strip())
            if not part:
                continue
            if " as " in part:
                source, local = (segment.strip() for segment in part.split(" as ", 1))
                bindings[local] = (spec, source)
            else:
                bindings[part] = (spec, part)
    for local, spec in NAMESPACE_IMPORT.findall(text):
        bindings[local] = (spec, "*")
    for local, spec in DEFAULT_IMPORT.findall(text):
        bindings.setdefault(local, (spec, "default"))
    return bindings


def rebase(spec, caller, target):
    """🧭️ Rewrites a caller-relative specifier so it resolves from the test module."""
    if not spec.startswith("."):
        return spec
    absolute = os.path.normpath(os.path.join(os.path.dirname(caller), spec))
    relative = os.path.relpath(absolute, os.path.dirname(target))
    return relative if relative.startswith(".") else "./" + relative


def parameter_span(text, function_name):
    """🎯️ The half-open span of the `dependencies` parameter's type annotation, brackets balanced."""
    signature = re.search(rf"export\s+async\s+function\s+{re.escape(function_name)}\s*\(", text)
    if signature is None:
        return None
    marker = text.find("dependencies: ", signature.end())
    if marker < 0:
        return None
    start = marker + len("dependencies: ")
    depth = 0
    for index in range(start, len(text)):
        character = text[index]
        if character in "<([{":
            depth += 1
        elif character in ">)]}":
            if depth == 0:
                return (start, index)
            depth -= 1
        elif character == "," and depth == 0:
            return (start, index)
    return None


def call_sites():
    """🔎️ Maps each (test module, register function) to its caller module and destructured keys."""
    listing = subprocess.run(
        ["grep", "-rl", "registerTests", "--include=🟦️.ts", "--include=🟦️.tsx", "--include=📜️script.ts", SCOPE],
        capture_output=True, text=True).stdout.split("\n")
    sites = {}
    for caller in listing:
        if not caller or "/dist/" in caller or "/🤖️generated/" in caller:
            continue
        text = io.open(caller, encoding="utf-8").read()
        pending = {}
        for line in text.split("\n"):
            binding = IMPORT_BINDING.search(line)
            if binding:
                pending[binding.group(1)] = binding.group(2)
            call = CALL.search(line)
            if call and call.group(1) in pending:
                target = os.path.normpath(os.path.join(os.path.dirname(caller), pending[call.group(1)]))
                keys = [key.strip() for key in call.group(2).split(",") if key.strip() and ":" not in key]
                if keys:
                    sites[(target, call.group(1))] = (caller, keys)
    return sites


def typed_annotation(keys, caller, target, caller_text):
    """🧬️ The intersection type for one call site, plus the caller-local names that need exporting."""
    exported = module_exports(caller_text)
    imported = module_imports(caller_text)
    own = rebase("./" + os.path.basename(caller), caller, target)
    groups, members, to_export, unresolved = {}, [], [], []
    for key in keys:
        if key in exported:
            groups.setdefault(own, []).append(key)
        elif key in imported:
            spec, source = imported[key]
            spec = rebase(spec, caller, target)
            if source == key:
                groups.setdefault(spec, []).append(key)
            elif source == "*":
                members.append(f'{{ readonly {key}: typeof import("{spec}") }}')
            else:
                members.append(f'{{ readonly {key}: (typeof import("{spec}"))["{source}"] }}')
        elif re.search(LOCAL_DECL.format(name=re.escape(key)), caller_text, re.M):
            to_export.append(key)
            groups.setdefault(own, []).append(key)
        else:
            unresolved.append(key)
    if unresolved:
        return None, unresolved, []
    parts = [f'Pick<typeof import("{spec}"), {" | ".join(sorted(chr(34) + key + chr(34) for key in names))}>'
             for spec, names in sorted(groups.items())]
    return " & ".join(parts + sorted(members)), [], to_export


def main(argv):
    apply = "--apply" in argv
    filters = [argument for argument in argv[1:] if not argument.startswith("--")]
    changed = 0
    for (target, function_name), (caller, keys) in sorted(call_sites().items()):
        if filters and not any(needle in target for needle in filters):
            continue
        if not os.path.exists(target):
            print(f"MISSING {target}")
            continue
        target_text = io.open(target, encoding="utf-8").read()
        span = parameter_span(target_text, function_name)
        if span is None:
            print(f"NO SIGNATURE {target} {function_name}")
            continue
        caller_text = io.open(caller, encoding="utf-8").read()
        annotation, unresolved, to_export = typed_annotation(keys, caller, target, caller_text)
        if annotation is None:
            print(f"UNRESOLVED {target} {function_name}: {unresolved[:6]}")
            continue
        current = target_text[span[0]:span[1]]
        if current == annotation:
            continue
        target_text = target_text[:span[0]] + annotation + target_text[span[1]:]
        for key in to_export:
            caller_text = re.sub(LOCAL_DECL.format(name=re.escape(key)), r"export \1 " + key, caller_text, count=1, flags=re.M)
        print(f"{target}\n    {function_name}: {len(keys)} keys, exported {to_export}")
        changed += 1
        if apply:
            io.open(target, "w", encoding="utf-8").write(target_text)
            if to_export:
                io.open(caller, "w", encoding="utf-8").write(caller_text)
    print(f"{'applied' if apply else 'would apply'} {changed} rewrites")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
