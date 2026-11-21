"""🧾 Identify the exact module closure behind a validation result."""
from hashlib import sha256
import re

def module_digest(root, *entrypoints):
    sources=set(entrypoints);pending=list(sources)
    imports=re.compile(r"""(?:from\s+|import\s*\(\s*|import\s+)["'](\.[^"']+)["']""")
    while pending:
        source=pending.pop()
        if source.suffix!=".ts":continue
        for name in imports.findall(source.read_text()):
            dependency=(source.parent/name).resolve()
            if dependency.is_file() and dependency not in sources:
                pending.append(dependency);sources.add(dependency)
    result=sha256()
    for source in sorted(sources):
        result.update(str(source.relative_to(root)).encode());result.update(source.read_bytes())
    return result.hexdigest()
