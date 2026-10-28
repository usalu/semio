import re, sys, os, resource
EMOJI = re.compile(r'^[☀-➿\U0001F000-\U0001FAFF⬀-⯿©®‼-㊙]')
def head(path):
    with open(path, 'rb') as f:
        return f.read(4096).decode('utf-8', 'replace')
def doc_of(path, text):
    ext = os.path.splitext(path)[1]
    if ext == '.py':
        m = re.search(r'^\s*(?:#![^\n]*\n)?(?:#[^\n]*\n\s*)*("""|\'\'\')(.*)', text, re.S)
        if not m: return None
        return m.group(2)
    if ext == '.rs':
        lines = [l for l in text.splitlines() if l.strip()]
        if lines and lines[0].lstrip().startswith('//!'):
            return lines[0].lstrip()[3:]
        return None
    m = re.search(r'^\s*/\*\*\s*(.*)', text, re.S)
    if not m:
        return None
    return m.group(1)
for path in sys.stdin.read().split('\n'):
    if not path or not os.path.isfile(path):
        continue
    text = head(path)
    d = doc_of(path, text)
    ext = os.path.splitext(path)[1]
    if ext not in ('.ts', '.mjs', '.py', '.rs'):
        continue
    if d is None:
        print('NODOC\t' + path)
    elif not EMOJI.match(d.lstrip()):
        print('NOEMOJI\t' + path + '\t' + d[:40].replace('\n', ' '))
print('maxrss_mb', resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // (1024*1024) if sys.platform == 'darwin' else resource.getrusage(resource.RUSAGE_SELF).ru_maxrss // 1024, file=sys.stderr)
