#!/usr/bin/env python3
"""Add emoji field to all ticket.json files based on title keywords."""
import json
import os
import re

KEYWORD_EMOJIS = [
    # Bug fixes
    (r'\bfix\b', '🔧'),
    (r'\bbug\b', '🐛'),
    (r'\bhotfix\b', '🚑'),
    (r'\bpatch\b', '🩹'),
    (r'\bregression\b', '🔙'),
    (r'\bcrash\b', '💥'),
    (r'\berror\b', '❌'),
    (r'\bbroken\b', '🔧'),
    (r'\bfailing\b', '🔧'),
    # Architecture & Refactoring
    (r'\brefactor\b', '♻️'),
    (r'\brestructure\b', '🏗️'),
    (r'\barchitect\b', '🏛️'),
    (r'\bconsolidate\b', '🧹'),
    (r'\bclean\b', '🧹'),
    (r'\bsimplif\b', '✂️'),
    (r'\bnormalize\b', '📏'),
    (r'\bdecouple\b', '🔓'),
    (r'\bmigrat\b', '🚚'),
    (r'\bmove\b', '📦'),
    (r'\brename\b', '🏷️'),
    (r'\bremove\b', '🗑️'),
    (r'\bdelete\b', '🗑️'),
    (r'\breplace\b', '🔄'),
    (r'\bmerge\b', '🔀'),
    (r'\bsplit\b', '✂️'),
    (r'\bextract\b', '📤'),
    # Testing
    (r'\btest\b', '🧪'),
    (r'\be2e\b', '🧪'),
    (r'\bci\b', '⚙️'),
    (r'\bpipeline\b', '⚙️'),
    (r'\bbenchmark\b', '📊'),
    (r'\bcoverage\b', '📊'),
    (r'\bvalidat\b', '✅'),
    # Features
    (r'\badd\b', '➕'),
    (r'\bimplement\b', '🔨'),
    (r'\bintroduc\b', '🌟'),
    (r'\bcreate\b', '🆕'),
    (r'\bbuild\b', '🔨'),
    (r'\bgenerat\b', '⚡'),
    (r'\benable\b', '✨'),
    (r'\bextend\b', '📐'),
    (r'\bsupport\b', '🤝'),
    (r'\ballow\b', '✨'),
    # UI/Frontend
    (r'\bui\b', '🎨'),
    (r'\bstyle\b', '🎨'),
    (r'\bcss\b', '🎨'),
    (r'\btheme\b', '🎨'),
    (r'\blayout\b', '📐'),
    (r'\bcomponent\b', '🧩'),
    (r'\bstorybook\b', '📚'),
    (r'\bstory\b', '📚'),
    (r'\bdiagram\b', '📊'),
    (r'\bsketchpad\b', '✏️'),
    (r'\bdesktop\b', '🖥️'),
    (r'\bnavigation\b', '🧭'),
    (r'\bbreadcrumb\b', '🍞'),
    (r'\bpanel\b', '📋'),
    (r'\btoolbar\b', '🔧'),
    (r'\bsidebar\b', '📋'),
    (r'\bmodal\b', '🪟'),
    (r'\bdialog\b', '🪟'),
    (r'\bscene\b', '🎬'),
    (r'\b3d\b', '🎯'),
    (r'\bcanvas\b', '🖼️'),
    (r'\brender\b', '🖼️'),
    # Database/Store
    (r'\bdatabase\b', '🗄️'),
    (r'\bdb\b', '🗄️'),
    (r'\bsqlite\b', '🗄️'),
    (r'\bstore\b', '🏪'),
    (r'\bschema\b', '📋'),
    (r'\bquery\b', '🔍'),
    # Server/API
    (r'\bserver\b', '🖥️'),
    (r'\bapi\b', '🔌'),
    (r'\bgraphql\b', '🔗'),
    (r'\bmcp\b', '🔌'),
    (r'\bendpoint\b', '🔌'),
    (r'\brest\b', '🔌'),
    (r'\bwebsocket\b', '🌐'),
    (r'\broute\b', '🛤️'),
    # Git/Repo
    (r'\bgit\b', '📝'),
    (r'\bhook\b', '🪝'),
    (r'\bcommit\b', '📝'),
    (r'\bbranch\b', '🌿'),
    (r'\brepo\b', '📂'),
    (r'\bticket\b', '🎫'),
    (r'\bgoal\b', '🎯'),
    # Documentation
    (r'\bdoc\b', '📝'),
    (r'\breadme\b', '📝'),
    (r'\bcomment\b', '💬'),
    (r'\bannotat\b', '📝'),
    # Config/Setup
    (r'\bconfig\b', '⚙️'),
    (r'\bsetup\b', '⚙️'),
    (r'\binstall\b', '📥'),
    (r'\bdeploy\b', '🚀'),
    (r'\bpackage\b', '📦'),
    (r'\bbundle\b', '📦'),
    (r'\bdependenc\b', '📦'),
    # Performance
    (r'\bperform\b', '⚡'),
    (r'\boptimiz\b', '⚡'),
    (r'\bcache\b', '💾'),
    (r'\bspeed\b', '⚡'),
    (r'\bfast\b', '⚡'),
    # Security
    (r'\bsecur\b', '🔒'),
    (r'\bauth\b', '🔐'),
    (r'\bpermission\b', '🔐'),
    (r'\btoken\b', '🔑'),
    # AI/ML
    (r'\bai\b', '🤖'),
    (r'\bllm\b', '🤖'),
    (r'\bassistant\b', '🤖'),
    (r'\bontolog\b', '🧠'),
    # Data/Format
    (r'\bjson\b', '📋'),
    (r'\byaml\b', '📋'),
    (r'\bxml\b', '📋'),
    (r'\bformat\b', '📋'),
    (r'\bpars\b', '📋'),
    (r'\bserial\b', '📋'),
    # VS Code
    (r'\bvscode\b', '💻'),
    (r'\bextension\b', '🔌'),
    (r'\beditor\b', '✏️'),
    # Build
    (r'\bvite\b', '⚡'),
    (r'\bwebpack\b', '📦'),
    (r'\belectron\b', '🖥️'),
    # Domain specific
    (r'\bkit\b', '🧰'),
    (r'\bdesign\b', '🎨'),
    (r'\bpiece\b', '🧩'),
    (r'\bconnect\b', '🔗'),
    (r'\bport\b', '🔌'),
    (r'\bquality\b', '✅'),
    (r'\bworkspa\b', '🏢'),
    (r'\bworkbench\b', '🔨'),
    (r'\bengine\b', '🏭'),
    (r'\bcoda\b', '🎵'),
    (r'\bprogram\b', '📝'),
    (r'\balgorithm\b', '🧮'),
    (r'\brust\b', '🦀'),
    (r'\bgo\b', '🐹'),
    (r'\btypescript\b', '📘'),
    (r'\bpython\b', '🐍'),
    # Generic action
    (r'\bupdate\b', '🔄'),
    (r'\bupgrad\b', '⬆️'),
    (r'\bintegrat\b', '🔗'),
    (r'\bwire\b', '🔗'),
    (r'\bsync\b', '🔄'),
    (r'\binit\b', '🚀'),
    (r'\bbootstrap\b', '🚀'),
    (r'\bexclude\b', '🚫'),
    (r'\bignore\b', '🚫'),
    (r'\bdisable\b', '⛔'),
]

DEFAULT_EMOJI = '📋'

def pick_emoji(title):
    title_lower = title.lower()
    for pattern, emoji in KEYWORD_EMOJIS:
        if re.search(pattern, title_lower):
            return emoji
    return DEFAULT_EMOJI

def process_ticket(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        data = json.loads(content)
        if 'emoji' in data and data['emoji']:
            return False
        title = data.get('title', '')
        if not title:
            return False
        emoji = pick_emoji(title)
        # Insert emoji after title to preserve field order
        new_content = content.replace(
            f'"title": "{title}"',
            f'"title": "{title}",\n  "emoji": "{emoji}"',
            1
        )
        if new_content == content:
            # Try with escaped title
            escaped = title.replace('"', '\\"')
            new_content = content.replace(
                f'"title": "{escaped}"',
                f'"title": "{escaped}",\n  "emoji": "{emoji}"',
                1
            )
        if new_content != content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True
        else:
            # Fallback: modify JSON data directly
            # Build ordered output
            data_with_emoji = {}
            for k, v in data.items():
                data_with_emoji[k] = v
                if k == 'title':
                    data_with_emoji['emoji'] = emoji
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(data_with_emoji, f, indent=2, ensure_ascii=False)
                f.write('\n')
            return True
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    repo_root = '/workspaces/semio'
    tickets_dir = os.path.join(repo_root, '.repo', '🎫')
    count = 0
    for root, dirs, files in os.walk(tickets_dir):
        for f in files:
            if f == 'ticket.json':
                fp = os.path.join(root, f)
                if process_ticket(fp):
                    count += 1
    print(f"Added emoji to {count} tickets")

if __name__ == '__main__':
    main()
