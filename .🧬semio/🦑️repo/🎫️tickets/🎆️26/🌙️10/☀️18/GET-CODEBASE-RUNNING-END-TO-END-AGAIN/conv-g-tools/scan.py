import sys, re, resource

EMOJI_RANGES = [(0x1F000, 0x1FAFF), (0x2600, 0x27BF), (0x2B00, 0x2BFF), (0x2300, 0x23FF),
                (0x2190, 0x21FF), (0x25A0, 0x25FF), (0x2934, 0x2935), (0x3030, 0x3030),
                (0x303D, 0x303D), (0x3297, 0x3299), (0x00A9, 0x00A9), (0x00AE, 0x00AE),
                (0x203C, 0x203C), (0x2049, 0x2049), (0x2122, 0x2122), (0x2139, 0x2139)]

def is_emoji(ch):
    cp = ord(ch)
    return any(a <= cp <= b for a, b in EMOJI_RANGES)

def scan_ts(src):
    out = []
    i, n, line, depth = 0, len(src), 1, 0
    while i < n:
        c = src[i]
        if c == '\n':
            line += 1; i += 1; continue
        if src.startswith('//', i):
            j = src.find('\n', i)
            j = n if j < 0 else j
            out.append(('line', line, depth, src[i:j]))
            i = j; continue
        if src.startswith('/*', i):
            j = src.find('*/', i + 2)
            j = n if j < 0 else j + 2
            text = src[i:j]
            kind = 'doc' if text.startswith('/**') and not text.startswith('/**/') else 'block'
            out.append((kind, line, depth, text))
            line += text.count('\n'); i = j; continue
        if c in '"\'':
            j = i + 1
            while j < n and src[j] != c and src[j] != '\n':
                if src[j] == '\\':
                    j += 1
                j += 1
            i = j + 1; continue
        if c == '`':
            j = i + 1
            while j < n and src[j] != '`':
                if src[j] == '\\':
                    j += 1
                j += 1
            line += src[i:j + 1].count('\n'); i = j + 1; continue
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
        i += 1
    return out

EXPORT_RE = re.compile(r'^export\s+(default\s+)?(declare\s+)?(async\s+)?(function\*?|const|let|var|class|type|interface|enum|abstract\s+class)\b')

def lead_key(text):
    body = re.sub(r'^/\*\*+', '', text)
    body = body.rstrip('/').rstrip('*')
    body = body.lstrip(' \t\r\n*')
    if not body:
        return None, ''
    ch = body[0]
    key = ch
    if len(body) > 1 and body[1] == '️':
        key += body[1]
    return (ch if is_emoji(ch) else None), key

def audit_ts(path, src, lines):
    f = []
    comments = scan_ts(src)
    docs_by_end = {}
    keys = {}
    for kind, ln, depth, text in comments:
        end = ln + text.count('\n')
        if kind == 'doc':
            docs_by_end[end] = (ln, text)
            emo, key = lead_key(text)
            if emo is None:
                f.append(('DOC-NOT-EMOJI', ln, f'docstring starts with {key!r}'))
            else:
                keys.setdefault(key, []).append(ln)
            if depth >= 2:
                f.append(('COMMENT-IN-BODY', ln, f'doc block at depth {depth}'))
        elif depth >= 1:
            f.append(('COMMENT-IN-BODY', ln, f'{kind} comment at depth {depth}: {text.strip()[:90]}'))
    for key, lns in keys.items():
        if len(lns) > 1:
            for ln in lns:
                f.append(('DUP-EMOJI', ln, f'{key} used by {len(lns)} docstrings in file (lines {",".join(map(str, lns))})'))
    for idx, raw in enumerate(lines, start=1):
        m = EXPORT_RE.match(raw)
        if not m:
            continue
        if (idx - 1) not in docs_by_end:
            f.append(('EXPORT-NO-DOC', idx, raw.strip()[:110]))
    return f

NUM_POLICY = re.compile(r'(\b[A-Z][A-Z0-9_]{2,}\s*=\s*[-+]?\d[\d_.]*\b)|(\d[\d_]*\s*\*\s*(1024|1000)\b)|(\b1024\s*\*\s*1024)|(\b1e\d+\b)')
KEYWORDS = re.compile(r'GB|MB|GiB|MiB|byte|RSS|RAM|ram\b|memory|budget|floor|timeout|TIMEOUT|retr|ttl|TTL|limit|LIMIT|max|MAX|min\b|MIN|ratio|threshold|cap\b|capacity|deadline|interval|\bms\b|second|percent|concurren|parallel|quota|cutoff|window|\bsize|batch|slot|pct|Ms\b|bytes', re.I)
NUM = re.compile(r'(?<![\w.])(\d[\d_]*(\.\d+)?)(?![\w.])')
USER_KW = re.compile(r'refus|notice|progress|Error\(|throw |message|warn|console\.|label|title|prompt|hint|help|usage|reason|status', re.I)
STR = re.compile(r'(["\'`])((?:(?!\1).){0,400}?)\1')

def audit_lines(path, lines):
    f = []
    for idx, raw in enumerate(lines, start=1):
        s = raw.strip()
        if not s or s.startswith('//') or s.startswith('*') or s.startswith('/*') or s.startswith('import '):
            continue
        code = re.sub(r'//.*$', '', raw)
        if NUM_POLICY.search(code) or (KEYWORDS.search(code) and NUM.search(code) and not re.search(r'\bfor\b.*\bof\b', code)):
            f.append(('POLICY-CONST?', idx, s[:150]))
        for m in STR.finditer(code):
            val = m.group(2)
            if len(val.split()) >= 3 and re.search(r'[A-Za-z]{3}', val) and USER_KW.search(code):
                f.append(('USER-STRING?', idx, s[:150]))
                break
    return f

def main():
    lst = sys.argv[1]
    out = sys.argv[2]
    paths = [l.rstrip('\n') for l in open(lst, encoding='utf-8') if l.strip()]
    with open(out, 'w', encoding='utf-8') as o:
        for p in paths:
            with open(p, encoding='utf-8') as fh:
                src = fh.read()
            lines = src.split('\n')
            if p.endswith('.py') or p.endswith('.rs'):
                findings = []
            else:
                findings = audit_ts(p, src, lines) + audit_lines(p, lines)
            for kind, ln, msg in findings:
                o.write(f'{kind}\t{p}:{ln}\t{msg}\n')
    print('maxrss_mb', resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // (1024 * 1024) if sys.platform == 'darwin' else resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // 1024)

if __name__ == '__main__':
    main()
