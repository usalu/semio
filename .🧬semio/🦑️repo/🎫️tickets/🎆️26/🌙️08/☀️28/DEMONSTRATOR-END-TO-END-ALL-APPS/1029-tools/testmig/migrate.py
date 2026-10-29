#!/usr/bin/env python3
"""Rewrites `RECV.dispatch_typed(CMD, META, IDENTITY, CX)` to the canonical
`artifact_app_laws::dispatch_original_fixture_command(&mut RECV, &CMD, META, POLICY, IDENTITY, CX)`.
usage: migrate.py [--apply] file...   (prints anomalies, never guesses)"""
import re, sys

POLICY = "semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT"
HELPER = "semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command"
NEEDLE = ".dispatch_typed("
CHAIN = re.compile(r"[A-Za-z_][A-Za-z0-9_]*(?:\s*\.\s*[A-Za-z0-9_]+)*\s*$")


def split_args(text, start):
    depth, args, cur, i = 0, [], start, start
    in_str = False
    while i < len(text):
        c = text[i]
        if in_str:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                args.append(text[cur:i])
                return args, i
            depth -= 1
        elif c == "," and depth == 0:
            args.append(text[cur:i])
            cur = i + 1
        elif c == "<" and False:
            pass
        i += 1
    raise ValueError("unbalanced")


def rewrite(text, path):
    out, pos, notes = [], 0, []
    while True:
        k = text.find(NEEDLE, pos)
        if k < 0:
            out.append(text[pos:])
            break
        line = text.count("\n", 0, k) + 1
        lo = max(pos, k - 300)
        m = CHAIN.search(text[lo:k])
        if not m:
            notes.append(f"{path}:{line}: complex receiver, skipped")
            out.append(text[pos : k + len(NEEDLE)])
            pos = k + len(NEEDLE)
            continue
        j = lo + m.start()
        recv = re.sub(r"\s+", "", m.group(0))
        args, end = split_args(text, k + len(NEEDLE))
        args = [a.strip() for a in args if a.strip()]
        if len(args) != 4:
            notes.append(f"{path}:{line}: {len(args)} args, skipped")
            out.append(text[pos : k + len(NEEDLE)])
            pos = k + len(NEEDLE)
            continue
        cmd, meta, ident, cx = args
        if "." in recv or recv.startswith("self"):
            r = f"&mut {recv}"
        elif re.search(r"\b" + recv + r"\s*:\s*&mut\b", text):
            r = f"&mut *{recv}"
        elif re.search(r"let\s+mut\s+" + recv + r"\b", text):
            r = f"&mut {recv}"
        else:
            r = f"&mut *{recv}"
            notes.append(f"{path}:{line}: ambiguous plain receiver `{recv}`")
        out.append(text[pos:j])
        out.append(f"{HELPER}({r}, &{cmd}, {meta}, {POLICY}, {ident}, {cx}")
        pos = end
    return "".join(out), notes


def main():
    apply = "--apply" in sys.argv
    files = [a for a in sys.argv[1:] if not a.startswith("--")]
    total = 0
    for f in files:
        text = open(f, encoding="utf-8").read()
        new, notes = rewrite(text, f)
        for n in notes:
            print(n)
        if new != text:
            total += text.count(NEEDLE) - new.count(NEEDLE)
            if apply:
                open(f, "w", encoding="utf-8").write(new)
    print("sites rewritten:", total)


main()
