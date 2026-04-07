#!/usr/bin/env python3
"""Fix bare #region (C#-style) section emojis."""
import os
import re
import hashlib

EMOJI_POOL = [
    "🧲","🤸","🎁","🔑","🕌","🎸","🎖️","🌧️","🎵","🪅",
    "🖥️","🕸️","🎼","🎺","🧨","🏩","🩻","🎊","📌","🌩️",
    "🧊","⏲️","💡","🎨","🎡","🪵","🔭","🎄","📃","🗻",
    "👓","🧭","📰","🗝️","👝","🥈","⛑️","📊","🥁","🌍",
    "💾","🏗️","🔧","📦","🎯","🗺️","🏂","📷","🧩","⚙️",
    "🔗","📋","📡","🌐","🎬","📐","🔤","🧮","🦉","💎",
    "🐙","🐹","🐍","🦀","🔷","🖼️","📔","🏪","✏️","🧬",
    "🧱","💻","🤖","🐘","🌈","🎪","🏆","🎲","🎭","🎹",
    "🎻","🔊","📢","📣","🔔","🎙️","📻","📱","🔍","📝",
    "📮","📧","📩","💌","📯","📜","🏷️","📎","🖇️","📌",
    "📍","🗂️","🗃️","🔒","🔓","🔐","🔑","🗡️","🛡️","🔮",
    "🔩","⚗️","🧫","🔬","🔭","🧲","⚡","🔋","💊","🩺",
    "🗿","🎗️","📹","📸","🎥","🖨️","🖲️","🕹️","🎮","🧸",
    "🪁","🪄","🪆","🪙","🪨","🛒","🛎️","🧳","⌛","⏳",
    "⏰","🕰️","⏱️","🌡️","🧊","🔥","💧","🌊","🌙","⭐",
    "🌟","✨","🌀","🌪️","🌈","🌤️","⛅","🌥️","🌦️","🌧️",
    "🌩️","🌨️","❄️","🎃","🎄","🎆","🎇","🎋","🎍","🎎",
    "🎏","🎐","🎑","🧨","🎈","🎉","🎊","🎁","🎀","🪩",
    "🪬","🧿","🎠","🎡","🎢","🏰","🗽","🗼","⛩️","🛕",
    "🌎","🎩","⛹","🏂","🔩","📯","🎵","🏩","🌥️","🩺",
]
seen = set()
UNIQUE_POOL = []
for e in EMOJI_POOL:
    if e not in seen:
        seen.add(e)
        UNIQUE_POOL.append(e)
EMOJI_POOL = UNIQUE_POOL

EMOJI_RE = re.compile(
    r'^('
    r'(?:'
    r'[\U0001F600-\U0001F64F]|[\U0001F300-\U0001F5FF]|[\U0001F680-\U0001F6FF]|'
    r'[\U0001F700-\U0001F77F]|[\U0001F780-\U0001F7FF]|[\U0001F800-\U0001F8FF]|'
    r'[\U0001F900-\U0001F9FF]|[\U0001FA00-\U0001FA6F]|[\U0001FA70-\U0001FAFF]|'
    r'[\U00002702-\U000027B0]|[\U000024C2-\U0001F251]|[\U0000FE00-\U0000FE0F]|'
    r'[\U0000200D]|[\U00002600-\U000026FF]|[\U00002300-\U000023FF]|'
    r'[\U0000FE0E]|[⛩⛏⛺⛽⛳⛲⛵⛴⛱⛰⛅⛈⛄⛹⛷⛸✨✏✒✍✋✊✌✅❌❎❇❗❓❕❣❤✝☸✡🕉☮♈♉♊♋♌♍♎♏♐♑♒♓⛎]'
    r')+'
    r')'
    r'(.*)'
)

def extract_emoji(name):
    m = EMOJI_RE.match(name)
    if m:
        return m.group(1), m.group(2)
    return "", name

def name_to_emoji_index(name, pool_size):
    h = hashlib.md5(name.encode('utf-8')).hexdigest()
    return int(h, 16) % pool_size

# Bare #region pattern (C# style) - no comment prefix
BARE_REGION_PAT = re.compile(r'^(\s*)#(region|endregion)\s+(.+?)\s*$')

def process_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except (UnicodeDecodeError, PermissionError):
        return False

    class SectionNode:
        def __init__(self, name, line_idx, marker_idx):
            self.name = name
            self.start_line = line_idx
            self.end_line = None
            self.start_marker_idx = marker_idx
            self.end_marker_idx = None
            self.children = []
            self.assigned_emoji = ""

    markers = []
    for i, line in enumerate(lines):
        stripped = line.rstrip('\n').rstrip('\r')
        m = BARE_REGION_PAT.match(stripped)
        if m:
            # But skip if preceded by a comment prefix (those were handled by main script)
            indent = m.group(1)
            after_indent = stripped[len(indent):]
            # Check if it's actually "// #region" or "# #region" etc.
            if after_indent.startswith('#region') or after_indent.startswith('#endregion'):
                # Make sure there's no comment prefix
                prefix_before = stripped[:len(indent)]
                if prefix_before.strip() == '' and not stripped.lstrip().startswith('//') and not stripped.lstrip().startswith('--'):
                    keyword = m.group(2)
                    name_part = m.group(3).strip()
                    emoji, bare = extract_emoji(name_part)
                    kind = 'start' if keyword == 'region' else 'end'
                    markers.append((i, kind, bare.strip(), emoji))

    if not markers:
        return False

    stack = []
    roots = []
    marker_to_section = {}

    for mi, (line_idx, kind, bare_name, old_emoji) in enumerate(markers):
        if kind == 'start':
            node = SectionNode(bare_name, line_idx, mi)
            if stack:
                stack[-1].children.append(node)
            else:
                roots.append(node)
            stack.append(node)
            marker_to_section[mi] = node
        elif kind == 'end':
            if stack:
                top = stack.pop()
                top.end_line = line_idx
                top.end_marker_idx = mi
                marker_to_section[mi] = top

    def assign_emojis(siblings):
        used = set()
        for node in siblings:
            idx = name_to_emoji_index(node.name, len(EMOJI_POOL))
            emoji = EMOJI_POOL[idx]
            attempts = 0
            while emoji in used and attempts < len(EMOJI_POOL):
                idx = (idx + 1) % len(EMOJI_POOL)
                emoji = EMOJI_POOL[idx]
                attempts += 1
            used.add(emoji)
            node.assigned_emoji = emoji
            if node.children:
                assign_emojis(node.children)

    assign_emojis(roots)

    changed = False
    new_lines = list(lines)

    for mi, (line_idx, kind, bare_name, old_emoji) in enumerate(markers):
        node = marker_to_section.get(mi)
        if node is None:
            continue
        new_emoji = node.assigned_emoji
        if not new_emoji:
            continue
        line = new_lines[line_idx]
        stripped = line.rstrip('\n').rstrip('\r')
        line_ending = line[len(stripped):]
        m = BARE_REGION_PAT.match(stripped)
        if m:
            indent = m.group(1)
            keyword = m.group(2)
            new_line = f"{indent}#{keyword} {new_emoji}{bare_name}{line_ending}"
            if new_line != line:
                new_lines[line_idx] = new_line
                changed = True

    if changed:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)

    return changed

def main():
    root = '/workspaces/semio'
    skip_dirs = {'node_modules', '.venv', 'target', '.repo', 'storybook-static', 'test-results', '.git'}
    updated = 0
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in skip_dirs]
        for fname in filenames:
            if fname.endswith('.cs'):
                filepath = os.path.join(dirpath, fname)
                if process_file(filepath):
                    updated += 1
                    print(f"  Updated: {filepath}")
    print(f"\nTotal files updated: {updated}")

if __name__ == '__main__':
    main()
