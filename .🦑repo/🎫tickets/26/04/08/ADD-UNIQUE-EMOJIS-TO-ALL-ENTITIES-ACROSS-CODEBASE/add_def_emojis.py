#!/usr/bin/env python3
"""Add unique semantic emojis to all definition docstrings across the codebase."""
import re
import os
import sys
import unicodedata

# Semantic emoji mapping: keywords → emoji
# These are chosen to be semantically meaningful based on the definition name/context
SEMANTIC_MAP = {
    # Types/Concepts
    'event': '📡', 'payload': '📦', 'emit': '📤', 'config': '⚙️', 'error': '❌',
    'request': '📨', 'response': '📩', 'handler': '🎯', 'middleware': '🔗',
    'server': '🖥️', 'client': '💻', 'connection': '🔌', 'socket': '🔌',
    'auth': '🔐', 'token': '🎟️', 'session': '🪪', 'cookie': '🍪',
    'user': '👤', 'contributor': '🤝', 'author': '✍️', 'owner': '👑',
    'ticket': '🎫', 'goal': '⛳', 'todo': '✅', 'task': '📋', 'draft': '📝',
    'file': '📄', 'folder': '📁', 'directory': '📂', 'path': '🛤️',
    'section': '📑', 'definition': '📖', 'scope': '🔭',
    'tree': '🌳', 'node': '🌿', 'leaf': '🍃', 'branch': '🌿', 'root': '🌱',
    'cache': '💾', 'store': '🏪', 'database': '🗄️', 'table': '📊',
    'query': '🔍', 'search': '🔎', 'filter': '🧹', 'sort': '📶',
    'parse': '🔬', 'scan': '📡', 'extract': '🧲', 'transform': '🔄',
    'render': '🎨', 'display': '📺', 'view': '👁️', 'template': '📋',
    'test': '🧪', 'mock': '🎭', 'fixture': '🔧', 'assert': '✔️',
    'build': '🏗️', 'compile': '⚒️', 'deploy': '🚀', 'install': '📥',
    'log': '📜', 'debug': '🐛', 'trace': '🔬', 'monitor': '📊',
    'type': '🏷️', 'kind': '🏷️', 'enum': '📇', 'interface': '🔌',
    'struct': '🧱', 'class': '🏛️', 'record': '💿',
    'func': '⚡', 'method': '🔧', 'procedure': '📐',
    'const': '🔒', 'var': '📦', 'param': '🎛️', 'arg': '🎛️',
    'string': '🔤', 'number': '🔢', 'bool': '🔘', 'array': '📚', 'map': '🗺️',
    'json': '📋', 'xml': '📃', 'yaml': '📃', 'csv': '📊',
    'url': '🌐', 'uri': '🔗', 'link': '🔗', 'href': '🔗',
    'regex': '🧩', 'pattern': '🧩', 'match': '🎯',
    'color': '🎨', 'theme': '🎨', 'style': '💄', 'icon': '🖼️',
    'button': '🔘', 'input': '📝', 'form': '📋', 'dialog': '💬',
    'list': '📋', 'queue': '📋', 'stack': '📚', 'set': '🗃️',
    'work': '💼', 'item': '🔖', 'entry': '📍', 'record': '💿',
    'provider': '🔌', 'factory': '🏭', 'builder': '👷', 'adapter': '🔄',
    'open': '📬', 'close': '📪', 'create': '🆕', 'delete': '🗑️',
    'change': '♻️', 'update': '🔁', 'move': '🚚', 'copy': '📋',
    'read': '📖', 'write': '✏️', 'send': '📤', 'receive': '📥',
    'start': '▶️', 'stop': '⏹️', 'pause': '⏸️', 'resume': '▶️',
    'add': '➕', 'remove': '➖', 'insert': '📌', 'append': '➡️',
    'integrate': '🧬', 'export': '📤', 'import': '📥',
    'analyze': '🔬', 'fix': '🔧', 'policy': '📜', 'check': '✔️',
    'git': '🐙', 'github': '🐙', 'commit': '💾', 'push': '⬆️', 'pull': '⬇️',
    'editor': '📝', 'ide': '💻', 'vscode': '💻',
    'graphql': '🕸️', 'rest': '🌐', 'grpc': '📡', 'websocket': '🔌',
    'emoji': '😀', 'unicode': '🔤', 'text': '📝', 'font': '🔤',
    'comment': '💬', 'annotation': '📌', 'label': '🏷️', 'tag': '🏷️',
    'heading': '📰', 'title': '📌', 'summary': '📋', 'description': '📝',
    'technology': '🛠️', 'bundle': '📦', 'monorepo': '🏢',
    'language': '🗣️', 'syntax': '📐', 'grammar': '📐',
    'ansi': '🎨', 'mermaid': '🧜', 'markdown': '📰',
    'artifact': '🏺', 'resource': '🎁', 'asset': '🖼️',
    'sandbox': '🏖️', 'container': '📦', 'devcontainer': '🐳',
    'checkpoint': '💾', 'snapshot': '📸', 'version': '📌',
    'milestone': '🎯', 'release': '🚀', 'publish': '📢',
    'subscriber': '📬', 'notification': '🔔', 'alert': '⚠️', 'warn': '⚠️',
    'reopen': '🔓', 'flag': '🚩',
}

# Fallback emojis when no semantic match found (unique per sibling index)
FALLBACK_EMOJIS = [
    '🔷', '🔶', '🔹', '🔸', '🔺', '🔻', '⬛', '⬜', '🟥', '🟧',
    '🟨', '🟩', '🟦', '🟪', '🟫', '💠', '🔳', '🔲', '▪️', '▫️',
    '◾', '◽', '◻️', '◼️', '🔵', '🔴', '🟠', '🟡', '🟢', '🟣',
    '🟤', '⚪', '⚫', '🩵', '🩶', '🩷', '💜', '💙', '💚', '💛',
    '🧡', '❤️', '🤍', '🖤', '🤎', '💗', '💖', '💝', '💘', '💕',
]


def is_emoji(ch):
    """Check if a character is an emoji."""
    cp = ord(ch)
    return (
        0x1F600 <= cp <= 0x1F64F or
        0x1F300 <= cp <= 0x1F5FF or
        0x1F680 <= cp <= 0x1F6FF or
        0x1F900 <= cp <= 0x1F9FF or
        0x1FA00 <= cp <= 0x1FAFF or
        0x2600 <= cp <= 0x26FF or
        0x2700 <= cp <= 0x27BF or
        0x2300 <= cp <= 0x23FF or
        0x2B50 <= cp <= 0x2B55 or
        0x200D <= cp <= 0x200D or
        0xFE00 <= cp <= 0xFE0F or
        cp in (0x203C, 0x2049, 0x20E3, 0x00A9, 0x00AE, 0x2122, 0x2139) or
        0x2194 <= cp <= 0x2199 or
        0x21A9 <= cp <= 0x21AA or
        0x231A <= cp <= 0x231B or
        0x25AA <= cp <= 0x25AB or
        0x25B6 <= cp <= 0x25C0 or
        0x25FB <= cp <= 0x25FE or
        unicodedata.category(ch).startswith('So')
    )


def starts_with_emoji(text):
    """Check if text already starts with an emoji."""
    if not text:
        return False
    return is_emoji(text[0])


def get_semantic_emoji(name, doc, used_emojis):
    """Get a semantically meaningful emoji for a definition."""
    combined = (name + ' ' + doc).lower()
    
    # Try specific compound matches first
    compounds = [
        ('ticket_open', '📬'), ('ticket_close', '📪'), ('ticket_reopen', '🔓'),
        ('ticket_change', '♻️'), ('ticket_read', '📖'),
        ('goal_open', '🎯'), ('goal_close', '🏁'), ('goal_reopen', '🔄'), ('goal_change', '📐'),
        ('todo_create', '🆕'), ('todo_change', '✏️'), ('todo_delete', '🗑️'),
        ('draft_create', '📝'), ('draft_delete', '🗑️'),
        ('file_create', '📄'), ('file_move', '🚚'), ('file_delete', '🗑️'),
        ('folder_create', '📁'), ('folder_move', '🚚'), ('folder_delete', '🗑️'),
        ('section_create', '📑'), ('section_move', '🚚'), ('section_delete', '🗑️'),
        ('contributor_add', '➕'), ('contributor_remove', '➖'),
        ('parsed_section', '📑'), ('parsed_definition', '📖'),
        ('section_frame', '🖼️'), ('scope_entry', '📍'),
        ('work_item', '💼'), ('contributor_work', '🤝'),
        ('event_kind', '📡'), ('checkpoint', '💾'),
        ('integrate', '🧬'), ('extract', '🧲'), ('export', '📤'),
        ('analyze', '🔬'), ('fix', '🔧'), ('policy', '📜'),
        ('graphql', '🕸️'), ('move', '🚚'), ('tree', '🌳'),
    ]
    
    name_lower = name.lower()
    for compound, emoji in compounds:
        if compound in name_lower and emoji not in used_emojis:
            return emoji
    
    # Try individual keyword matches
    for keyword, emoji in sorted(SEMANTIC_MAP.items(), key=lambda x: -len(x[0])):
        if keyword in combined and emoji not in used_emojis:
            return emoji
    
    # Fallback
    for emoji in FALLBACK_EMOJIS:
        if emoji not in used_emojis:
            return emoji
    
    return '🔖'


def process_go_file(filepath):
    """Process a Go file and add emojis to definition docstrings."""
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    modified = False
    # Track which sections definitions belong to
    # Group definitions by section for sibling uniqueness
    section_stack = []
    current_section = 'root'
    section_used_emojis = {}  # section -> set of used emojis
    
    # First pass: identify sections and definitions
    defs_by_section = {}  # section -> [(line_idx, name, doc_line_idx)]
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Track sections
        if stripped.startswith('// #region ') or stripped.startswith('#region '):
            section_name = stripped.split('#region ', 1)[1].strip()
            section_stack.append(current_section)
            current_section = section_name
            if current_section not in section_used_emojis:
                section_used_emojis[current_section] = set()
        elif stripped.startswith('// #endregion') or stripped.startswith('#endregion'):
            if section_stack:
                current_section = section_stack.pop()
        
        # Find Go definitions (type, func, var, const) with preceding docstring
        if re.match(r'^(type|func|var|const)\s', stripped):
            # Find the docstring line(s) above
            doc_idx = i - 1
            while doc_idx >= 0 and lines[doc_idx].strip().startswith('//'):
                doc_idx -= 1
            doc_idx += 1  # First comment line
            
            if doc_idx < i and lines[doc_idx].strip().startswith('//'):
                doc_text = lines[doc_idx].strip().lstrip('/ ')
                name_match = re.match(r'^(?:type|func|var|const)\s+(?:\([^\)]*\)\s*)?(\w+)', stripped)
                if name_match:
                    def_name = name_match.group(1)
                    if current_section not in defs_by_section:
                        defs_by_section[current_section] = []
                    defs_by_section[current_section].append((doc_idx, def_name, doc_text))
    
    # Second pass: assign emojis and modify
    for section, defs in defs_by_section.items():
        used = section_used_emojis.get(section, set())
        # Also collect already-used emojis from existing definitions
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                # Extract existing emoji
                emoji_end = 1
                while emoji_end < len(doc_text) and (
                    is_emoji(doc_text[emoji_end]) or 
                    ord(doc_text[emoji_end]) in (0xFE0F, 0xFE0E, 0x200D, 0x20E3) or
                    0x1F3FB <= ord(doc_text[emoji_end]) <= 0x1F3FF
                ):
                    emoji_end += 1
                used.add(doc_text[:emoji_end])
        
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                continue  # Already has emoji
            
            emoji = get_semantic_emoji(def_name, doc_text, used)
            used.add(emoji)
            
            # Modify the docstring line
            old_line = lines[doc_idx]
            # Find the start of text after "// "
            prefix_match = re.match(r'^(\s*//\s*)', old_line)
            if prefix_match:
                prefix = prefix_match.group(1)
                rest = old_line[len(prefix):]
                new_line = prefix + emoji + rest
                lines[doc_idx] = new_line
                modified = True
    
    if modified:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    return False


def process_ts_file(filepath):
    """Process a TypeScript/JavaScript file and add emojis to definition docstrings."""
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    modified = False
    section_stack = []
    current_section = 'root'
    section_used_emojis = {}
    defs_by_section = {}
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Track sections
        if '//#region ' in stripped or '// #region ' in stripped:
            section_name = stripped.split('#region ', 1)[1].strip() if '#region ' in stripped else 'unknown'
            section_stack.append(current_section)
            current_section = section_name
            if current_section not in section_used_emojis:
                section_used_emojis[current_section] = set()
        elif '//#endregion' in stripped or '// #endregion' in stripped:
            if section_stack:
                current_section = section_stack.pop()
        
        # Find TS/JS definitions
        def_match = re.match(r'^\s*(?:export\s+)?(?:async\s+)?(?:function|class|interface|type|const|let|var|enum)\s+(\w+)', stripped)
        if def_match:
            def_name = def_match.group(1)
            # Find docstring above (JSDoc or // comment)
            doc_idx = i - 1
            
            # Check for JSDoc ending
            if doc_idx >= 0 and lines[doc_idx].strip() == '*/':
                # Find start of JSDoc
                jsdoc_start = doc_idx
                while jsdoc_start >= 0 and not lines[jsdoc_start].strip().startswith('/**'):
                    jsdoc_start -= 1
                if jsdoc_start >= 0:
                    # Find the first content line of JSDoc (after /**)
                    content_idx = jsdoc_start
                    content_line = lines[content_idx].strip()
                    if content_line == '/**':
                        content_idx += 1
                    if content_idx < len(lines):
                        content_line = lines[content_idx].strip()
                        if content_line.startswith('* ') or content_line.startswith('/**'):
                            text = content_line.lstrip('/* ')
                            if current_section not in defs_by_section:
                                defs_by_section[current_section] = []
                            defs_by_section[current_section].append((content_idx, def_name, text))
            
            # Check for // comment
            elif doc_idx >= 0 and lines[doc_idx].strip().startswith('//'):
                doc_text = lines[doc_idx].strip().lstrip('/ ')
                if current_section not in defs_by_section:
                    defs_by_section[current_section] = []
                defs_by_section[current_section].append((doc_idx, def_name, doc_text))
    
    for section, defs in defs_by_section.items():
        used = section_used_emojis.get(section, set())
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                emoji_end = 1
                while emoji_end < len(doc_text) and (
                    is_emoji(doc_text[emoji_end]) or
                    ord(doc_text[emoji_end]) in (0xFE0F, 0xFE0E, 0x200D, 0x20E3) or
                    0x1F3FB <= ord(doc_text[emoji_end]) <= 0x1F3FF
                ):
                    emoji_end += 1
                used.add(doc_text[:emoji_end])
        
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                continue
            
            emoji = get_semantic_emoji(def_name, doc_text, used)
            used.add(emoji)
            
            old_line = lines[doc_idx]
            # Handle JSDoc lines: " * text" or "/** text"
            jsdoc_match = re.match(r'^(\s*\*\s*)', old_line)
            jsdoc_start_match = re.match(r'^(\s*/\*\*\s*)', old_line)
            comment_match = re.match(r'^(\s*//\s*)', old_line)
            
            if jsdoc_match:
                prefix = jsdoc_match.group(1)
                rest = old_line[len(prefix):]
                lines[doc_idx] = prefix + emoji + rest
                modified = True
            elif jsdoc_start_match:
                prefix = jsdoc_start_match.group(1)
                rest = old_line[len(prefix):]
                lines[doc_idx] = prefix + emoji + rest
                modified = True
            elif comment_match:
                prefix = comment_match.group(1)
                rest = old_line[len(prefix):]
                lines[doc_idx] = prefix + emoji + rest
                modified = True
    
    if modified:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    return False


def process_python_file(filepath):
    """Process a Python file and add emojis to definition docstrings."""
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    modified = False
    section_stack = []
    current_section = 'root'
    section_used_emojis = {}
    defs_by_section = {}
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Track sections (Python uses # region)
        if stripped.startswith('# region ') or stripped.startswith('#region '):
            section_name = stripped.split('region ', 1)[1].strip()
            section_stack.append(current_section)
            current_section = section_name
            if current_section not in section_used_emojis:
                section_used_emojis[current_section] = set()
        elif stripped.startswith('# endregion') or stripped.startswith('#endregion'):
            if section_stack:
                current_section = section_stack.pop()
        
        # Find Python definitions
        def_match = re.match(r'^\s*(def|class)\s+(\w+)', stripped)
        if def_match:
            def_name = def_match.group(2)
            # Find docstring below (""" or ''')
            doc_idx = i + 1
            # Skip decorator lines and empty lines
            while doc_idx < len(lines) and (lines[doc_idx].strip().startswith('@') or lines[doc_idx].strip() == ''):
                doc_idx += 1
            
            # Check for docstring
            if doc_idx < len(lines):
                doc_stripped = lines[doc_idx].strip()
                triple_match = re.match(r'^("""|\'\'\')\s*(.+?)(?:"""|\'\'\')?$', doc_stripped)
                if triple_match:
                    doc_text = triple_match.group(2)
                    if current_section not in defs_by_section:
                        defs_by_section[current_section] = []
                    defs_by_section[current_section].append((doc_idx, def_name, doc_text))
    
    for section, defs in defs_by_section.items():
        used = section_used_emojis.get(section, set())
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                emoji_end = 1
                while emoji_end < len(doc_text) and (
                    is_emoji(doc_text[emoji_end]) or
                    ord(doc_text[emoji_end]) in (0xFE0F, 0xFE0E, 0x200D, 0x20E3) or
                    0x1F3FB <= ord(doc_text[emoji_end]) <= 0x1F3FF
                ):
                    emoji_end += 1
                used.add(doc_text[:emoji_end])
        
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                continue
            
            emoji = get_semantic_emoji(def_name, doc_text, used)
            used.add(emoji)
            
            old_line = lines[doc_idx]
            # Find the opening triple quotes
            triple_match = re.match(r'^(\s*(?:"""|\'\'\')\s*)', old_line)
            if triple_match:
                prefix = triple_match.group(1)
                rest = old_line[len(prefix):]
                lines[doc_idx] = prefix + emoji + rest
                modified = True
    
    if modified:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    return False


def process_rust_file(filepath):
    """Process a Rust file and add emojis to definition docstrings."""
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    modified = False
    section_stack = []
    current_section = 'root'
    section_used_emojis = {}
    defs_by_section = {}
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Find Rust definitions
        def_match = re.match(r'^\s*(?:pub\s+)?(?:fn|struct|enum|trait|impl|const|static|type|mod)\s+(\w+)', stripped)
        if def_match:
            def_name = def_match.group(1)
            # Find /// docstring above
            doc_idx = i - 1
            while doc_idx >= 0 and lines[doc_idx].strip().startswith('///'):
                doc_idx -= 1
            doc_idx += 1  # First doc comment line
            
            if doc_idx < i and lines[doc_idx].strip().startswith('///'):
                doc_text = lines[doc_idx].strip().lstrip('/ ')
                if current_section not in defs_by_section:
                    defs_by_section[current_section] = []
                defs_by_section[current_section].append((doc_idx, def_name, doc_text))
    
    for section, defs in defs_by_section.items():
        used = section_used_emojis.get(section, set())
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                emoji_end = 1
                while emoji_end < len(doc_text) and (
                    is_emoji(doc_text[emoji_end]) or
                    ord(doc_text[emoji_end]) in (0xFE0F, 0xFE0E, 0x200D, 0x20E3) or
                    0x1F3FB <= ord(doc_text[emoji_end]) <= 0x1F3FF
                ):
                    emoji_end += 1
                used.add(doc_text[:emoji_end])
        
        for doc_idx, def_name, doc_text in defs:
            if starts_with_emoji(doc_text):
                continue
            
            emoji = get_semantic_emoji(def_name, doc_text, used)
            used.add(emoji)
            
            old_line = lines[doc_idx]
            prefix_match = re.match(r'^(\s*///\s*)', old_line)
            if prefix_match:
                prefix = prefix_match.group(1)
                rest = old_line[len(prefix):]
                lines[doc_idx] = prefix + emoji + rest
                modified = True
    
    if modified:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    return False


def process_cs_file(filepath):
    """Process a C# file and add emojis to definition docstrings."""
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    modified = False
    section_stack = []
    current_section = 'root'
    section_used_emojis = {}
    defs_by_section = {}
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Track sections
        if stripped.startswith('#region '):
            section_name = stripped.split('#region ', 1)[1].strip()
            section_stack.append(current_section)
            current_section = section_name
            if current_section not in section_used_emojis:
                section_used_emojis[current_section] = set()
        elif stripped.startswith('#endregion'):
            if section_stack:
                current_section = section_stack.pop()
        
        # Find C# definitions
        def_match = re.match(r'^\s*(?:public|private|protected|internal)?\s*(?:static\s+)?(?:partial\s+)?(?:abstract\s+)?(?:sealed\s+)?(?:class|struct|interface|enum|record)\s+(\w+)', stripped)
        if def_match:
            def_name = def_match.group(1)
            # Find summary XML docs above
            doc_idx = i - 1
            while doc_idx >= 0 and (lines[doc_idx].strip().startswith('///') or lines[doc_idx].strip().startswith('[') or lines[doc_idx].strip() == ''):
                doc_idx -= 1
            doc_idx += 1
            
            # Find <summary> line
            summary_idx = None
            for j in range(doc_idx, i):
                if '<summary>' in lines[j]:
                    summary_idx = j
                    break
            
            if summary_idx is not None:
                # Check if inline summary
                summary_match = re.search(r'<summary>\s*(.+?)\s*</summary>', lines[summary_idx])
                if summary_match:
                    doc_text = summary_match.group(1)
                    if current_section not in defs_by_section:
                        defs_by_section[current_section] = []
                    defs_by_section[current_section].append((summary_idx, def_name, doc_text, True))
                else:
                    # Multi-line summary - find the text line
                    next_line_idx = summary_idx + 1
                    if next_line_idx < len(lines) and '</summary>' not in lines[next_line_idx]:
                        doc_text = lines[next_line_idx].strip().lstrip('/ ')
                        if current_section not in defs_by_section:
                            defs_by_section[current_section] = []
                        defs_by_section[current_section].append((next_line_idx, def_name, doc_text, False))
    
    for section, defs in defs_by_section.items():
        used = section_used_emojis.get(section, set())
        for entry in defs:
            doc_idx, def_name, doc_text = entry[0], entry[1], entry[2]
            if starts_with_emoji(doc_text):
                emoji_end = 1
                while emoji_end < len(doc_text) and (
                    is_emoji(doc_text[emoji_end]) or
                    ord(doc_text[emoji_end]) in (0xFE0F, 0xFE0E, 0x200D, 0x20E3)
                ):
                    emoji_end += 1
                used.add(doc_text[:emoji_end])
        
        for entry in defs:
            doc_idx, def_name, doc_text, is_inline = entry[0], entry[1], entry[2], entry[3]
            if starts_with_emoji(doc_text):
                continue
            
            emoji = get_semantic_emoji(def_name, doc_text, used)
            used.add(emoji)
            
            old_line = lines[doc_idx]
            if is_inline:
                # Replace text inside <summary>...</summary>
                lines[doc_idx] = old_line.replace(doc_text, emoji + doc_text, 1)
                modified = True
            else:
                # Find prefix (///  or just whitespace)
                prefix_match = re.match(r'^(\s*///\s*)', old_line)
                if prefix_match:
                    prefix = prefix_match.group(1)
                    rest = old_line[len(prefix):]
                    lines[doc_idx] = prefix + emoji + rest
                    modified = True
    
    if modified:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    return False


EXCLUDED_DIRS = {
    'node_modules', '.git', 'target', 'dist', 'build', '.next', '.nuxt',
    'storybook-static', 'test-results', '.repo', 'temp', 'vendor',
    '__pycache__', '.tox', '.mypy_cache', 'bin', 'obj',
}

EXCLUDED_FILES = {
    'package-lock.json', 'yarn.lock', 'pnpm-lock.yaml',
}


def find_source_files(root):
    """Find all source files in the workspace."""
    files = {'go': [], 'ts': [], 'py': [], 'rs': [], 'cs': []}
    for dirpath, dirnames, filenames in os.walk(root):
        # Skip excluded dirs
        dirnames[:] = [d for d in dirnames if d not in EXCLUDED_DIRS]
        for f in filenames:
            if f in EXCLUDED_FILES:
                continue
            ext = os.path.splitext(f)[1]
            full = os.path.join(dirpath, f)
            if ext == '.go':
                files['go'].append(full)
            elif ext in ('.ts', '.tsx', '.js', '.jsx'):
                files['ts'].append(full)
            elif ext == '.py':
                files['py'].append(full)
            elif ext == '.rs':
                files['rs'].append(full)
            elif ext == '.cs':
                files['cs'].append(full)
    return files


def main():
    root = '/workspaces/semio'
    files = find_source_files(root)
    
    total = 0
    for lang, file_list in files.items():
        count = 0
        for f in file_list:
            try:
                if lang == 'go':
                    if process_go_file(f):
                        count += 1
                elif lang == 'ts':
                    if process_ts_file(f):
                        count += 1
                elif lang == 'py':
                    if process_python_file(f):
                        count += 1
                elif lang == 'rs':
                    if process_rust_file(f):
                        count += 1
                elif lang == 'cs':
                    if process_cs_file(f):
                        count += 1
            except Exception as e:
                print(f"  ERROR: {f}: {e}", file=sys.stderr)
        print(f"{lang}: {count}/{len(file_list)} files modified")
        total += count
    
    print(f"\nTotal: {total} files modified")


if __name__ == '__main__':
    main()
