#!/usr/bin/env python3
"""Fix section emojis: assign unique emojis to sibling sections at each nesting level."""
import os
import re
import hashlib

# 200 diverse emojis, EXCLUDING 🔖 (old default)
EMOJI_POOL = [
    "🛕","🧪","🌎","🎩","📃","🤸","🗻","👓","🧭","🎖️",
    "📰","🗝️","👝","🥈","⛑️","📊","🥁","🏩","🕌","🏬",
    "🖋️","🧵","✈️","🎽","🌍","💾","🏗️","🔧","📦","🎯",
    "🗺️","🏂","📷","🧩","⚙️","🔗","📋","📡","🌐","🖥️",
    "🎬","📐","🔤","🧮","🕸️","🦉","💎","🐙","🐹","🐍",
    "🦀","🔷","🖼️","📔","🏪","✏️","🧬","🧱","💻","🤖",
    "🐘","🌈","🎪","🏆","🎲","🎸","🎨","🎭","🎹","🎺",
    "🎻","🎼","🎵","🎶","🔊","📢","📣","🔔","🎙️","📻",
    "🎚️","🎛️","📱","💡","🔍","🔎","📝","📮","📬","📭",
    "📧","📩","📨","💌","📯","📜","🏷️","📎","🖇️","📌",
    "📍","🗂️","🗃️","🗄️","🗑️","🔒","🔓","🔐","🔑","🗡️",
    "🛡️","🔮","💎","🔩","⚗️","🧫","🧬","🔬","🔭","📡",
    "🧲","⚡","🔋","💊","🩺","🩻","🧬","🗿","🎗️","🎞️",
    "📹","📸","🎥","🖨️","🖲️","🕹️","🎮","🧸","🪁","🪄",
    "🪅","🪆","🪙","🪨","🪵","🛒","🛎️","🧳","⌛","⏳",
    "⏰","🕰️","⏱️","⏲️","🌡️","🧊","🔥","💧","🌊","🌙",
    "⭐","🌟","✨","🌀","🌪️","🌈","🌤️","⛅","🌥️","🌦️",
    "🌧️","🌩️","🌨️","❄️","🎃","🎄","🎆","🎇","🎋","🎍",
    "🎎","🎏","🎐","🎑","🧨","🎈","🎉","🎊","🎁","🎀",
    "🪩","🪬","🧿","🎠","🎡","🎢","🏰","🗽","🗼","⛩️",
]

# Deduplicate while preserving order
seen = set()
UNIQUE_POOL = []
for e in EMOJI_POOL:
    if e not in seen:
        seen.add(e)
        UNIQUE_POOL.append(e)
EMOJI_POOL = UNIQUE_POOL

SKIP_DIRS = {
    'node_modules', '.venv', 'target', '.repo', 'storybook-static',
    'test-results', '.git', 'sb-addons', 'sb-common-assets', 'sb-manager',
    'manifests', 'debug', 'rust-analyzer', 'flycheck0'
}

EXTENSIONS = {'.go', '.ts', '.tsx', '.js', '.jsx', '.py', '.cs', '.rs', '.sql', '.rb', '.css'}

# Patterns for region markers by language
# Go/TS/JS/CSS/C#: // #region EmojiName  or  // #endregion EmojiName
# Python/Ruby: # #region EmojiName  or  # #endregion EmojiName  
# SQL: -- #region EmojiName  or  -- #endregion EmojiName
# Rust: mod modname { // EmojiName  or  } // EmojiName

# Regex to extract emoji prefix from a name like "🛕Header" -> ("🛕", "Header")
# An emoji can be multi-codepoint (ZWJ, variation selectors, skin tones)
EMOJI_RE = re.compile(
    r'^('
    r'(?:'
    r'[\U0001F600-\U0001F64F]|[\U0001F300-\U0001F5FF]|[\U0001F680-\U0001F6FF]|'
    r'[\U0001F700-\U0001F77F]|[\U0001F780-\U0001F7FF]|[\U0001F800-\U0001F8FF]|'
    r'[\U0001F900-\U0001F9FF]|[\U0001FA00-\U0001FA6F]|[\U0001FA70-\U0001FAFF]|'
    r'[\U00002702-\U000027B0]|[\U000024C2-\U0001F251]|[\U0000FE00-\U0000FE0F]|'
    r'[\U0000200D]|[\U00002600-\U000026FF]|[\U00002300-\U000023FF]|'
    r'[\U0000231A-\U0000231B]|[\U000025AA-\U000025AB]|[\U000025B6]|[\U000025C0]|'
    r'[\U000025FB-\U000025FE]|[\U00002614-\U00002615]|[\U00002648-\U00002653]|'
    r'[\U0000267F]|[\U00002693]|[\U000026A1]|[\U000026AA-\U000026AB]|'
    r'[\U000026BD-\U000026BE]|[\U000026C4-\U000026C5]|[\U000026CE]|[\U000026D4]|'
    r'[\U000026EA]|[\U000026F2-\U000026F3]|[\U000026F5]|[\U000026FA]|[\U000026FD]|'
    r'[\U00002702]|[\U00002705]|[\U00002708-\U0000270D]|[\U0000270F]|'
    r'[\U00002712]|[\U00002714]|[\U00002716]|[\U0000271D]|[\U00002721]|'
    r'[\U00002728]|[\U00002733-\U00002734]|[\U00002744]|[\U00002747]|'
    r'[\U0000274C]|[\U0000274E]|[\U00002753-\U00002755]|[\U00002757]|'
    r'[\U00002763-\U00002764]|[\U00002795-\U00002797]|[\U000027A1]|[\U000027B0]|'
    r'[\U0000FE0F]|[\U0000200D]|[\U0001F1E0-\U0001F1FF]|'
    r'[\u2694-\u269C]|[\u26A0]|[\u26B0-\u26B1]|[\u26C8]|[\u26CF]|[\u26D1]|[\u26D3]|'
    r'[\u26E9]|[\u26F0-\u26F1]|[\u26F4]|[\u26F7-\u26F9]|'
    r'[\u2934-\u2935]|[\u23E9-\u23F3]|[\u23F8-\u23FA]|[\u25AA-\u25AB]|[\u25B6]|'
    r'[\u25C0]|[\u25FB-\u25FE]|[\u2B05-\u2B07]|[\u2B1B-\u2B1C]|[\u2B50]|[\u2B55]|'
    r'[\u3030]|[\u303D]|[\u3297]|[\u3299]|[\u00A9]|[\u00AE]|[\u200D]|[\uFE0F]|'
    r'[\U0000FE0E]|[⛩⛏⛺⛽⛳⛲⛵⛴⛱⛰⛅⛈⛄⛹⛷⛸✨✏✒✍✋✊✌✅❌❎❇❗❓❕❣❤✝☸✡🕉☮♈♉♊♋♌♍♎♏♐♑♒♓⛎]'
    r')+'
    r')'
    r'(.*)'
)

def extract_emoji(name):
    """Extract leading emoji from name. Returns (emoji, bare_name)."""
    m = EMOJI_RE.match(name)
    if m:
        return m.group(1), m.group(2)
    return "", name

def name_to_emoji_index(name, pool_size):
    """Hash a name to a pool index for deterministic assignment."""
    h = hashlib.md5(name.encode('utf-8')).hexdigest()
    return int(h, 16) % pool_size

# Region pattern: captures (prefix, region_keyword, emoji+name)
# prefix is the comment characters, keyword is "region" or "endregion"
REGION_PAT = re.compile(
    r'^(\s*(?://|#|--|/\*)\s*)#(region|endregion)\s+(.+?)\s*(?:\*/)?$'
)

# Rust section start: mod xxx { // EmojiName
RUST_START_PAT = re.compile(
    r'^(\s*(?:pub\s+)?mod\s+\w+\s*\{\s*//\s*)(.+?)\s*$'
)
# Rust section end: } // EmojiName
RUST_END_PAT = re.compile(
    r'^(\s*\}\s*//\s*)(.+?)\s*$'
)


def process_file(filepath):
    """Process a single file, assigning unique emojis to sibling sections."""
    ext = os.path.splitext(filepath)[1]
    if ext not in EXTENSIONS:
        return False
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except (UnicodeDecodeError, PermissionError):
        return False
    
    is_rust = ext == '.rs'
    
    # Phase 1: Parse all region markers
    # Each entry: (line_index, kind='start'|'end', bare_name, original_emoji)
    markers = []
    
    for i, line in enumerate(lines):
        stripped = line.rstrip('\n').rstrip('\r')
        
        if is_rust:
            # Check Rust start pattern
            m = RUST_START_PAT.match(stripped)
            if m:
                name_part = m.group(2).strip()
                emoji, bare = extract_emoji(name_part)
                markers.append((i, 'start', bare.strip(), emoji, 'rust_start'))
                continue
            # Check Rust end pattern
            m = RUST_END_PAT.match(stripped)
            if m:
                name_part = m.group(2).strip()
                emoji, bare = extract_emoji(name_part)
                markers.append((i, 'end', bare.strip(), emoji, 'rust_end'))
                continue
        
        # Check standard region pattern
        m = REGION_PAT.match(stripped)
        if m:
            keyword = m.group(2)  # 'region' or 'endregion'
            name_part = m.group(3).strip()
            emoji, bare = extract_emoji(name_part)
            kind = 'start' if keyword == 'region' else 'end'
            markers.append((i, kind, bare.strip(), emoji, 'standard'))
            continue

    if not markers:
        return False
    
    # Phase 2: Build section tree and identify sibling groups
    # Use a stack to match starts/ends and group siblings
    
    class SectionNode:
        def __init__(self, name, line_idx, marker_idx):
            self.name = name
            self.start_line = line_idx
            self.end_line = None
            self.start_marker_idx = marker_idx
            self.end_marker_idx = None
            self.children = []
            self.assigned_emoji = ""
    
    stack = []
    roots = []
    marker_to_section = {}
    
    for mi, (line_idx, kind, bare_name, old_emoji, pat_type) in enumerate(markers):
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
    
    # Phase 3: Assign unique emojis per sibling group
    def assign_emojis(siblings):
        used = set()
        for node in siblings:
            # Start with hash-based index for determinism
            idx = name_to_emoji_index(node.name, len(EMOJI_POOL))
            emoji = EMOJI_POOL[idx]
            # Resolve collisions
            attempts = 0
            while emoji in used and attempts < len(EMOJI_POOL):
                idx = (idx + 1) % len(EMOJI_POOL)
                emoji = EMOJI_POOL[idx]
                attempts += 1
            used.add(emoji)
            node.assigned_emoji = emoji
            # Recurse to children
            if node.children:
                assign_emojis(node.children)
    
    assign_emojis(roots)
    
    # Phase 4: Rewrite lines with new emojis
    changed = False
    new_lines = list(lines)
    
    for mi, (line_idx, kind, bare_name, old_emoji, pat_type) in enumerate(markers):
        node = marker_to_section.get(mi)
        if node is None:
            continue
        
        new_emoji = node.assigned_emoji
        if not new_emoji:
            continue
        
        line = new_lines[line_idx]
        stripped = line.rstrip('\n').rstrip('\r')
        line_ending = line[len(stripped):]
        
        if pat_type == 'standard':
            m = REGION_PAT.match(stripped)
            if m:
                prefix = m.group(1)
                keyword = m.group(2)
                new_line = f"{prefix}#{keyword} {new_emoji}{bare_name}{line_ending}"
                if new_line != line:
                    new_lines[line_idx] = new_line
                    changed = True
        elif pat_type == 'rust_start':
            m = RUST_START_PAT.match(stripped)
            if m:
                prefix = m.group(1)
                new_line = f"{prefix}{new_emoji}{bare_name}{line_ending}"
                if new_line != line:
                    new_lines[line_idx] = new_line
                    changed = True
        elif pat_type == 'rust_end':
            m = RUST_END_PAT.match(stripped)
            if m:
                prefix = m.group(1)
                new_line = f"{prefix}{new_emoji}{bare_name}{line_ending}"
                if new_line != line:
                    new_lines[line_idx] = new_line
                    changed = True
    
    if changed:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
    
    return changed


def main():
    root = '/workspaces/semio'
    updated = 0
    total_markers = 0
    
    for dirpath, dirnames, filenames in os.walk(root):
        # Skip excluded directories
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        
        for fname in filenames:
            ext = os.path.splitext(fname)[1]
            if ext not in EXTENSIONS:
                continue
            filepath = os.path.join(dirpath, fname)
            if process_file(filepath):
                updated += 1
                print(f"  Updated: {filepath}")
    
    print(f"\nTotal files updated: {updated}")


if __name__ == '__main__':
    main()
