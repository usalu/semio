#!/usr/bin/env python3
"""Append `.await` after every CALL (never a `fn NAME(` definition) of a whitelisted, confirmed-async
helper function, within one file. Bracket-matched (not naive regex) so nested calls are handled
correctly across multiple passes. Skips a call that already has `.await` immediately after it.
"""
import sys

path = sys.argv[1]
names = sys.argv[2:]

with open(path, encoding="utf-8") as f:
    text = f.read()

def find_calls(text, name):
    """Yield (start, end_of_matching_paren) for each call site `name(...)` that isn't `fn name(`."""
    i = 0
    out = []
    key = name + "("
    while True:
        j = text.find(key, i)
        if j == -1:
            break
        # reject if preceded by "fn " (a definition) - check up to 4 chars back for "fn "
        prefix = text[max(0, j-4):j]
        if prefix.endswith("fn "):
            i = j + len(key)
            continue
        # reject if part of a longer identifier (preceding char is alnum/_)
        if j > 0 and (text[j-1].isalnum() or text[j-1] == '_'):
            i = j + len(key)
            continue
        # bracket-match from the '(' at j+len(name)
        paren_start = j + len(name)
        depth = 0
        k = paren_start
        while k < len(text):
            if text[k] == '(':
                depth += 1
            elif text[k] == ')':
                depth -= 1
                if depth == 0:
                    break
            k += 1
        if depth != 0:
            i = j + len(key)
            continue
        end = k + 1  # one past the matching ')'
        # skip if already awaited
        if text[end:end+6] == '.await':
            i = end
            continue
        out.append((j, end))
        i = end
    return out

total = 0
for name in names:
    while True:
        calls = find_calls(text, name)
        if not calls:
            break
        # apply from the end backward so offsets stay valid
        calls.sort(key=lambda t: t[0], reverse=True)
        changed = False
        for start, end in calls:
            text = text[:end] + ".await" + text[end:]
            total += 1
            changed = True
        if not changed:
            break
        # loop again in case new instances appear identical (shouldn't, but safe)
        break

with open(path, "w", encoding="utf-8") as f:
    f.write(text)

print(f"applied {total} .await insertions across {len(names)} names")
