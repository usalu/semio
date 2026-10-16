"""🔬 Minimal Rust lexer helpers: comment/string stripping, identifier extraction, defined-name extraction.

Everything here is analysis-only; no source is written.
"""
import re

IDENT = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')


def strip(src):
    """Blank out comments, string and char literals (keeps newlines and offsets)."""
    out = []
    i = 0
    n = len(src)
    while i < n:
        c = src[i]
        if c == '/' and i + 1 < n and src[i + 1] == '/':
            j = src.find('\n', i)
            j = n if j < 0 else j
            out.append(' ' * (j - i))
            i = j
        elif c == '/' and i + 1 < n and src[i + 1] == '*':
            depth = 1
            j = i + 2
            while j < n and depth:
                if src.startswith('/*', j):
                    depth += 1
                    j += 2
                elif src.startswith('*/', j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            seg = src[i:j]
            out.append(re.sub(r'[^\n]', ' ', seg))
            i = j
        elif c == 'r' and re.match(r'r#*"', src[i:i + 8]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == '_')):
            m = re.match(r'r(#*)"', src[i:i + 8])
            hashes = m.group(1)
            end = src.find('"' + hashes, i + len(m.group(0)))
            end = n if end < 0 else end + 1 + len(hashes)
            seg = src[i:end]
            out.append(re.sub(r'[^\n]', ' ', seg))
            i = end
        elif c == '"':
            j = i + 1
            while j < n:
                if src[j] == '\\':
                    j += 2
                    continue
                if src[j] == '"':
                    j += 1
                    break
                j += 1
            seg = src[i:j]
            out.append(re.sub(r'[^\n]', ' ', seg))
            i = j
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", src[i:i + 12])
            if m:
                out.append(' ' * len(m.group(0)))
                i += len(m.group(0))
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return ''.join(out)


def idents(text):
    return set(IDENT.findall(text))


DEF = re.compile(
    r'^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?'
    r'(fn|struct|enum|trait|type|static|const|mod|union|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)',
    re.M)


def defined_names(stripped):
    return {m.group(2) for m in DEF.finditer(stripped)}
