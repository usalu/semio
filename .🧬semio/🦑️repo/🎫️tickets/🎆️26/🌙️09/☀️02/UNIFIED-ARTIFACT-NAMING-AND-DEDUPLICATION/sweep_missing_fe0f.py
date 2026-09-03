#!/usr/bin/env python3
"""🔍 Sweep artifact trees for directory-name components whose leading emoji
codepoint lacks the U+FE0F variation selector, when the same base emoji is
used elsewhere (sibling dirs / registry) WITH U+FE0F. Read-only reporting."""
import os, sys, unicodedata, json, collections

REPO = "/Users/ueli/Documents/semio"
EXCLUDE = {"node_modules", "target", "dist", ".git"}
FE0F = "️"

def is_emoji_start(ch):
    cp = ord(ch)
    return cp > 0x2000 and cp not in (0xfe0f,)

def split_lead(name):
    """Return (lead_char, has_fe0f, rest) if name starts with an emoji-ish char."""
    if not name:
        return None
    lead = name[0]
    if not is_emoji_start(lead):
        return None
    idx = 1
    has_fe0f = False
    if len(name) > 1 and name[1] == FE0F:
        has_fe0f = True
        idx = 2
    rest = name[idx:]
    return (lead, has_fe0f, rest)

base_has_fe0f = collections.defaultdict(set)   # lead_char -> set of bool seen
base_examples = collections.defaultdict(lambda: collections.defaultdict(list))  # lead_char -> has_fe0f -> [full names]

all_dirs = []
for root, dirs, files in os.walk(REPO):
    dirs[:] = [d for d in dirs if d not in EXCLUDE]
    for d in dirs:
        full = os.path.join(root, d)
        all_dirs.append(full)
        parsed = split_lead(d)
        if parsed:
            lead, has_fe0f, rest = parsed
            base_has_fe0f[lead].add(has_fe0f)
            base_examples[lead][has_fe0f].append(full)

print("=== Bases with INCONSISTENT fe0f usage across the whole repo ===")
inconsistent = {k: v for k, v in base_has_fe0f.items() if len(v) > 1}
for lead, s in sorted(inconsistent.items(), key=lambda x: -len(base_examples[x[0]][False])):
    cp = hex(ord(lead))
    name = unicodedata.name(lead, "?")
    without = base_examples[lead][False]
    withfe = base_examples[lead][True]
    print(f"\nLEAD {lead!r} U+{ord(lead):04X} ({name})  WITHOUT_FE0F={len(without)}  WITH_FE0F={len(withfe)}")
    for p in without[:20]:
        print("   MISSING:", p)
    if len(without) > 20:
        print(f"   ... and {len(without)-20} more")
