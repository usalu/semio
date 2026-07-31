#!/usr/bin/env python3
"""Second pass: add emojis to Go definitions missing them."""
import re
import sys
import unicodedata

SEMANTIC_RULES = [
    # Specific compound patterns (checked first, longest match wins)
    (r'ticket.*open', '📬️'), (r'ticket.*close', '📪️'), (r'ticket.*reopen', '🔓️'),
    (r'ticket.*change', '♻️'), (r'ticket.*read', '📖️'),
    (r'goal.*open', '🎯️'), (r'goal.*close', '🏁️'), (r'goal.*reopen', '🔄️'), (r'goal.*change', '📐️'),
    (r'todo.*create', '🆕️'), (r'todo.*change', '✏️'), (r'todo.*delete', '🗑️'),
    (r'draft.*create', '📝️'), (r'draft.*delete', '🗑️'),
    (r'file.*create', '📄️'), (r'file.*move', '🚚️'), (r'file.*delete', '🗑️'),
    (r'folder.*create', '📁️'), (r'folder.*move', '🚚️'), (r'folder.*delete', '🗑️'),
    (r'section.*create', '📑️'), (r'section.*move', '🚚️'), (r'section.*delete', '🗑️'),
    (r'graphql|query', '🕸️'), (r'monorepo', '🏢️'), (r'new.*root|newroot', '🌱️'),
    (r'build.*tree|tree.*build', '🌳️'), (r'filter.*tree|tree.*filter', '🔍️'),
    (r'search.*tree|tree.*search', '🔎️'), (r'render.*tree|tree.*render', '🎨️'),
    (r'render.*markdown|markdown.*render', '📰️'), (r'render.*mermaid|mermaid', '🧜️'),
    (r'render.*text|text.*render', '📜️'), (r'render.*ansi|ansi.*render', '🎨️'),
    (r'entity.*kind|entitykind', '🏷️'), (r'entity.*emoji', '😀️'),
    (r'definition.*kind|definitionkind', '📖️'),
    (r'parse.*time|flexible.*time', '⏰️'), (r'parse.*section', '📑️'),
    (r'parse.*definition', '📖️'), (r'parse.*region', '💬️'),
    (r'parse.*markdown', '📰️'), (r'parse.*heading', '📰️'),
    (r'scope.*id|build.*scope', '🔭️'), (r'scope.*entry', '📍️'),
    (r'cache.*schema|schema.*version', '📌️'), (r'cache', '💾️'),
    (r'provider.*registry|registry', '📋️'), (r'provider.*interface', '🔌️'),
    (r'provider', '🔌️'), (r'factory', '🏭️'), (r'adapter', '🔄️'),
    (r'config|configuration', '⚙️'), (r'command', '⌨️'),
    (r'policy|statute', '📜️'), (r'breach', '⚠️'), (r'territory', '🗺️'),
    (r'flag', '🚩️'), (r'analyze|analysis', '🔬️'), (r'fix', '🔧️'),
    (r'integrate', '🧬️'), (r'extract', '🧲️'), (r'export', '📤️'), (r'import', '📥️'),
    (r'embed', '📎️'), (r'template', '📋️'), (r'format', '🖊️'),
    (r'error|err', '❌️'), (r'warn', '⚠️'), (r'info', 'ℹ'),
    (r'scan|walk', '🔬️'), (r'visit', '👣️'), (r'traverse', '🚶️'),
    (r'valid|check', '✔️'), (r'filter', '🔍️'), (r'search', '🔎️'),
    (r'sort', '📶️'), (r'group', '📊️'), (r'merge', '🔀️'), (r'split', '✂️'),
    (r'create|new', '🆕️'), (r'delete|remove|drop', '🗑️'),
    (r'update|change|modify|edit', '✏️'), (r'insert|add|append', '➕️'),
    (r'read|get|fetch|load|find', '📖️'), (r'write|save|store|put', '💾️'),
    (r'open', '📬️'), (r'close', '📪️'), (r'start|begin|init', '▶️'),
    (r'stop|end|finish', '⏹️'), (r'send|emit|dispatch|publish', '📤️'),
    (r'receive|subscribe|listen', '📥️'),
    (r'connect|join|link', '🔗️'), (r'disconnect|detach', '🔌️'),
    (r'lock', '🔒️'), (r'unlock', '🔓️'), (r'encrypt', '🔐️'), (r'decrypt', '🔑️'),
    (r'string|text|name|label|title', '🔤️'), (r'number|count|size|length|int', '🔢️'),
    (r'bool|flag|toggle|enable|disable', '🔘️'), (r'list|array|slice|collection', '📋️'),
    (r'map|dict|hash|set', '🗺️'), (r'tree|document|nested', '🌳️'),
    (r'node|item|element|entry', '🔖️'), (r'key|id|identifier', '🔑️'),
    (r'value|data|content|body', '📦️'), (r'header|meta|info', '📰️'),
    (r'color|colour|rgb|hex', '🎨️'), (r'style|css|theme', '💄️'),
    (r'icon|image|picture|photo', '🖼️'), (r'font|typography', '🔤️'),
    (r'button|btn', '🔘️'), (r'input|field|form', '📝️'),
    (r'dialog|modal|popup|overlay', '💬️'), (r'menu|nav|sidebar', '📑️'),
    (r'panel|card|box|container', '📦️'), (r'layout|grid|flex', '📐️'),
    (r'run|execute|invoke|call|dispatch', '⚡️'), (r'is|has|can|should|report', '❓️'),
    (r'convert|transform|map', '🔄️'), (r'compare|diff|equal', '⚖️'),
    (r'copy|clone|duplicate', '📋️'), (r'move|rename|relocate', '🚚️'),
    (r'count|total|sum', '🔢️'), (r'print|log|output|display|show|render', '📺️'),
    (r'test|spec|assert', '🧪️'), (r'mock|stub|fake', '🎭️'),
    (r'helper|util|utility|tool', '🛠️'), (r'wrapper|proxy|decorator', '🎀️'),
    (r'callback|handler|hook|listener', '🎯️'), (r'event|signal|message', '📡️'),
    (r'request|req', '📨️'), (r'response|res|resp', '📩️'),
    (r'auth|login|logout|permission', '🔐️'), (r'user|account|profile', '👤️'),
    (r'session|cookie|token', '🪪️'), (r'url|uri|link|href', '🌐️'),
    (r'file|path|dir|folder', '📄️'), (r'regex|pattern|glob|match', '🧩️'),
    (r'json|yaml|toml|xml', '📋️'), (r'github|git|repo', '🐙️'),
    (r'commit|push|pull|branch|tag|checkout', '💾️'),
    (r'server|service|daemon', '🖥️'), (r'client', '💻️'),
    (r'database|db|sql|table', '🗄️'), (r'api|endpoint|route', '🌐️'),
    (r'type|struct|class|interface|enum|record', '🧱️'),
    (r'func|method|procedure', '⚡️'), (r'var|const|field|property|param', '📦️'),
    (r'ephemeral|temporary|transient', '⏳️'),
    (r'holds|data|fields|record', '📦️'),
]

FALLBACK_EMOJIS = [
    '🔷️', '🔶️', '🔹️', '🔸️', '🔺️', '🔻️', '⬛️', '⬜️', '🟥️', '🟧️',
    '🟨️', '🟩️', '🟦️', '🟪️', '🟫️', '💠️', '🔳️', '🔲️', '▪️', '▫️',
    '◾', '◽', '◻', '◼', '🔵️', '🔴️', '🟠️', '🟡️', '🟢️', '🟣️',
    '🟤️', '⚪️', '⚫️', '🩵️', '🩶️', '🩷️', '💜️', '💙️', '💚️', '💛️',
    '🧡️', '❤️', '🤍️', '🖤️', '🤎️', '💗️', '💖️', '💝️', '💘️', '💕️',
    '🏵️', '🌸️', '🌺️', '🌻️', '🌼️', '🌷️', '🌹️', '🥀️', '🪻️', '🪷️',
    '🍁️', '🍂️', '🍃️', '🌿️', '☘️', '🍀️', '🪴️', '🌱️', '🌲️', '🌳️',
]


def is_emoji_cp(cp):
    return (
        0x1F600 <= cp <= 0x1F64F or 0x1F300 <= cp <= 0x1F5FF or
        0x1F680 <= cp <= 0x1F6FF or 0x1F900 <= cp <= 0x1F9FF or
        0x1FA00 <= cp <= 0x1FAFF or 0x2600 <= cp <= 0x26FF or
        0x2700 <= cp <= 0x27BF or 0x2300 <= cp <= 0x23FF or
        0x2B50 <= cp <= 0x2B55 or cp == 0x200D or 0xFE00 <= cp <= 0xFE0F or
        cp in (0x203C, 0x2049, 0x20E3, 0x00A9, 0x00AE, 0x2122, 0x2139) or
        0x2194 <= cp <= 0x2199 or 0x21A9 <= cp <= 0x21AA or
        0x231A <= cp <= 0x231B or 0x25AA <= cp <= 0x25FE or
        unicodedata.category(chr(cp)).startswith('So')
    )


def starts_with_emoji(text):
    if not text:
        return False
    return is_emoji_cp(ord(text[0]))


def get_semantic_emoji(name, doc, used_emojis):
    combined = (name + ' ' + doc).lower()
    for pattern, emoji in SEMANTIC_RULES:
        if re.search(pattern, combined) and emoji not in used_emojis:
            return emoji
    for emoji in FALLBACK_EMOJIS:
        if emoji not in used_emojis:
            return emoji
    return '🔖️'


def extract_emoji(text):
    """Extract leading emoji from text."""
    if not text or not is_emoji_cp(ord(text[0])):
        return '', text
    i = 0
    while i < len(text) and (is_emoji_cp(ord(text[i])) or
            ord(text[i]) in (0xFE0F, 0xFE0E, 0x200D, 0x20E3) or
            0x1F3FB <= ord(text[i]) <= 0x1F3FF):
        i += 1
    return text[:i], text[i:]


def process_go_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    modified = False
    # Track sections for sibling grouping
    section_stack = []
    current_section = 'root'
    section_used = {}  # section -> set of used emojis

    # First pass: collect already-used emojis per section
    for i, line in enumerate(lines):
        stripped = line.strip()
        if re.match(r'^//\s*#region\s', stripped) or stripped.startswith('#region '):
            section_name = stripped.split('#region', 1)[1].strip()
            section_stack.append(current_section)
            current_section = section_name
            if current_section not in section_used:
                section_used[current_section] = set()
        elif re.match(r'^//\s*#endregion', stripped) or stripped.startswith('#endregion'):
            if section_stack:
                current_section = section_stack.pop()

        if re.match(r'^(type|func|var|const)\s', stripped):
            if i > 0 and lines[i-1].strip().startswith('//'):
                doc = lines[i-1].strip()
                # Remove leading //
                doc_text = re.sub(r'^//\s*', '', doc)
                if starts_with_emoji(doc_text):
                    emoji, _ = extract_emoji(doc_text)
                    if current_section not in section_used:
                        section_used[current_section] = set()
                    section_used[current_section].add(emoji)

    # Reset for second pass
    section_stack = []
    current_section = 'root'

    for i, line in enumerate(lines):
        stripped = line.strip()
        if re.match(r'^//\s*#region\s', stripped) or stripped.startswith('#region '):
            section_name = stripped.split('#region', 1)[1].strip()
            section_stack.append(current_section)
            current_section = section_name
            if current_section not in section_used:
                section_used[current_section] = set()
        elif re.match(r'^//\s*#endregion', stripped) or stripped.startswith('#endregion'):
            if section_stack:
                current_section = section_stack.pop()

        if re.match(r'^(type|func|var|const)\s', stripped):
            if i > 0 and lines[i-1].strip().startswith('//'):
                doc = lines[i-1].strip()
                doc_text = re.sub(r'^//\s*', '', doc)
                if not starts_with_emoji(doc_text):
                    # Need to add emoji
                    name_match = re.match(r'^(?:type|func|var|const)\s+(?:\([^\)]*\)\s*)?(\w+)', stripped)
                    def_name = name_match.group(1) if name_match else 'unknown'
                    used = section_used.get(current_section, set())
                    emoji = get_semantic_emoji(def_name, doc_text, used)
                    used.add(emoji)
                    section_used[current_section] = used

                    # Insert emoji after "// "
                    prefix_match = re.match(r'^(\s*//\s*)', lines[i-1])
                    if prefix_match:
                        prefix = prefix_match.group(1)
                        rest = lines[i-1][len(prefix):]
                        lines[i-1] = prefix + emoji + rest
                        modified = True

    if modified:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    return False


if __name__ == '__main__':
    files = sys.argv[1:]
    for f in files:
        if process_go_file(f):
            print(f'Modified: {f}')
        else:
            print(f'No changes: {f}')
