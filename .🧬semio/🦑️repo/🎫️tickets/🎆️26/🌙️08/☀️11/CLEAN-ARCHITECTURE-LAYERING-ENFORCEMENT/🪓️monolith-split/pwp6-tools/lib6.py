import sys, os, re, json, collections, hashlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common6 import *

def mask(t):
    """same-length mask: comments, string and char literal contents -> spaces (newlines kept)"""
    o = list(t); n = len(t); i = 0
    def blank(a, b):
        for k in range(a, b):
            if o[k] != '\n': o[k] = ' '
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); j = n if j < 0 else j; blank(i, j); i = j; continue
        if t.startswith('/*', i):
            d = 1; j = i + 2
            while j < n and d:
                if t.startswith('/*', j): d += 1; j += 2
                elif t.startswith('*/', j): d -= 1; j += 2
                else: j += 1
            blank(i, j); i = j; continue
        if c in 'rbc' and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == '_')):
            m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i + 12])
            if m:
                e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
                blank(i + m.end(), j - len(e)); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            blank(i + 1, min(j, n)); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*?|[^\\'])'", t[i:i + 14])
            if m: blank(i + 1, i + m.end() - 1); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: i += m.end(); continue
        i += 1
    return ''.join(o)

def line_offsets(t):
    offs = [0]
    for m in re.finditer('\n', t): offs.append(m.end())
    return offs

def sha(t): return hashlib.sha256(t.encode('utf-8')).hexdigest()[:16]
