#!/usr/bin/env python3
"""Remove [DEBUG]-prefixed temporary log statements from .rs/.ts/.tsx files.

usage: strip_debug.py [--apply] [--journal=PATH] FILE...   (FILE list on stdin when no FILE)
Dry-run by default. Writes only the files it was given, atomically, after verifying mtime unchanged.
"""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lexmask

RS_CALL = re.compile(r'\b(?:std::)?(?:e?println|e?print|dbg)!\s*\(\s*(?=r?#*"\[DEBUG\])')
TS_CALL = re.compile(r'\b(?:console\.(?:log|debug|error|warn|info)|process\.(?:stderr|stdout)\.write)\s*\(\s*(?=["\'`]\[DEBUG\])')


def skip_rs_string(s, i):
    """i at opening quote or r#*". returns index after the closing."""
    n = len(s)
    if s[i] == 'r':
        j = i + 1
        h = 0
        while s[j] == '#':
            h += 1
            j += 1
        assert s[j] == '"'
        end = '"' + '#' * h
        k = s.index(end, j + 1)
        return k + len(end)
    j = i + 1
    while j < n:
        c = s[j]
        if c == '\\':
            j += 2
            continue
        if c == '"':
            return j + 1
        j += 1
    raise ValueError('unterminated string')


def match_rs(s, i):
    """i just after '('. returns index after matching ')'."""
    n = len(s)
    depth = 1
    j = i
    while j < n:
        c = s[j]
        if c == '"':
            j = skip_rs_string(s, j)
            continue
        if c == 'r' and j + 1 < n and s[j + 1] in '"#' and (j == 0 or not (s[j - 1].isalnum() or s[j - 1] == '_')):
            k = j + 1
            while k < n and s[k] == '#':
                k += 1
            if k < n and s[k] == '"':
                j = skip_rs_string(s, j)
                continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", s[j:j + 12])
            if m:
                j += m.end()
                continue
            j += 1
            continue
        if c == '/' and s[j:j + 2] == '//':
            j = s.index('\n', j) if '\n' in s[j:] else n
            continue
        if c == '/' and s[j:j + 2] == '/*':
            j = s.index('*/', j) + 2
            continue
        if c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
            if depth == 0:
                return j + 1
        j += 1
    raise ValueError('unbalanced')


def skip_js_template(s, i):
    n = len(s)
    j = i + 1
    while j < n:
        c = s[j]
        if c == '\\':
            j += 2
            continue
        if c == '`':
            return j + 1
        if c == '$' and s[j + 1:j + 2] == '{':
            j = match_js(s, j + 2)
            continue
        j += 1
    raise ValueError('unterminated template')


def match_js(s, i):
    n = len(s)
    depth = 1
    j = i
    while j < n:
        c = s[j]
        if c in '"\'':
            k = j + 1
            while k < n and s[k] != c:
                if s[k] == '\\':
                    k += 1
                k += 1
            j = k + 1
            continue
        if c == '`':
            j = skip_js_template(s, j)
            continue
        if c == '/' and s[j:j + 2] == '//':
            j = s.index('\n', j) if '\n' in s[j:] else n
            continue
        if c == '/' and s[j:j + 2] == '/*':
            j = s.index('*/', j) + 2
            continue
        if c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
            if depth == 0:
                return j + 1
        j += 1
    raise ValueError('unbalanced')


def prev_sig(s, i):
    j = i - 1
    while j >= 0 and s[j] in ' \t\r\n':
        j -= 1
    return j


def js_control_paren(s, j):
    """s[j] == ')'; True when it closes an if/for/while/with header."""
    depth = 0
    k = j
    while k >= 0:
        c = s[k]
        if c == ')':
            depth += 1
        elif c == '(':
            depth -= 1
            if depth == 0:
                break
        k -= 1
    m = re.search(r'(\bif|\bfor|\bwhile|\bwith|\bawait\s+for)\s*$', s[max(0, k - 20):k])
    return bool(m)


SIDE = re.compile(r'\.(push|pop|shift|advance|insert|remove|clear|drain|fetch_add|store|send|recv|borrow_mut|close|poll|step|run|submit|retire)\s*\(|\+=|-=|\+\+|(?<![-\w])--(?![-\w])|&mut\b|\bawait\b')


def literal_of(args):
    m = re.match(r'(?:r#*)?["\'`]\[DEBUG\] ?([^"\'`\\$\{\n]{0,60})', args)
    return m.group(1).strip() if m else ''


def kept(args, stems):
    lit = literal_of(args)
    if len(lit) < 5:
        return False
    return any(st and (lit.startswith(st) or st.startswith(lit)) for st in stems)


def process(path, text, legacy=False, stems=()):
    ext = path.rsplit('.', 1)[-1]
    rs = ext == 'rs'
    call = RS_CALL if rs else TS_CALL
    mask = None if legacy else (lexmask.mask_rs(text) if rs else lexmask.mask_ts(text))
    out = []
    pos = 0
    stats = {'delete': 0, 'expr': 0, 'unhandled': [], 'side': []}
    edits = []
    for m in call.finditer(text):
        if m.start() < pos:
            continue
        if mask is not None and mask[m.start()]:
            stats.setdefault('instring', []).append(m.start())
            continue
        try:
            end = match_rs(text, m.end()) if rs else match_js(text, m.end())
        except Exception as e:
            stats['unhandled'].append((m.start(), 'parse ' + str(e)))
            continue
        args = text[m.end():end - 1]
        if not legacy and kept(args, stems):
            stats.setdefault('kept', []).append(args[:80])
            continue
        start = m.start()
        j = prev_sig(text, start)
        pc = text[j] if j >= 0 else ''
        while rs and pc == ']':
            depth = 0
            q = j
            while q >= 0:
                if text[q] == ']':
                    depth += 1
                elif text[q] == '[':
                    depth -= 1
                    if depth == 0:
                        break
                q -= 1
            if q < 1 or text[q - 1] != '#':
                break
            start = q - 1
            j = prev_sig(text, start)
            pc = text[j] if j >= 0 else ''
        k = end
        while k < len(text) and text[k] in ' \t':
            k += 1
        semi = k < len(text) and text[k] == ';'
        endfull = k + 1 if semi else end
        if j < 0 or pc in ';{}':
            kind = 'delete'
        elif pc == ')' and (rs or not js_control_paren(text, j)):
            kind = 'delete' if not rs else 'delete'
            if not rs:
                kind = 'delete'
        elif pc == ')' and not rs:
            kind = 'block'
        else:
            kind = None
        if kind is None:
            word = re.search(r'(\w+)$', text[max(0, j - 10):j + 1])
            if text[j - 1:j + 1] == '=>' or (not rs and word and word.group(1) == 'else') or pc == '|' or (pc == ')' and False):
                kind = 'block'
            elif not rs and pc == ')':
                kind = 'block'
            else:
                stats['unhandled'].append((m.start(), 'ctx ' + repr(text[max(0, j - 12):j + 1])))
                continue
        if pc == ')' and not rs and js_control_paren(text, j):
            kind = 'block'
        if SIDE.search(args[args.index('"[DEBUG]') if '"[DEBUG]' in args else 0:]):
            stats['side'].append((m.start(), args[:120].replace('\n', ' ')))
            continue
        edits.append((start, endfull if kind in ('delete', 'block') else end, kind))
        pos = endfull
    res = []
    last = 0
    for a, b, kind in edits:
        res.append(text[last:a])
        if kind == 'block':
            res.append('{}')
            stats['expr'] += 1
        else:
            ls = text.rfind('\n', 0, a) + 1
            le = text.find('\n', b)
            le = len(text) if le < 0 else le
            if text[ls:a].strip() == '' and text[b:le].strip() == '':
                res_prefix = ''.join(res)
                cut = len(res_prefix) - (a - ls)
                res = [res_prefix[:cut]]
                last = le + 1 if le < len(text) else le
                stats['delete'] += 1
                continue
            stats['delete'] += 1
        last = b
    res.append(text[last:])
    new = ''.join(res)
    if rs and edits:
        names = set()
        for a, b, kind in edits:
            names |= set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', text[a:b]))
        names -= {'DEBUG', 'let', 'mut', 'as', 'u64', 'u32', 'usize', 'i64', 'f64', 'len', 'unwrap', 'expect', 'self', 'Some', 'None', 'Ok', 'Err', 'true', 'false', 'println', 'eprintln', 'print', 'eprint', 'dbg', 'str', 'String'}
        for name in sorted(names):
            if len(name) < 2:
                continue
            occ = list(re.finditer(r'(?<![A-Za-z0-9_])' + re.escape(name) + r'(?![A-Za-z0-9_])', new))
            if len(occ) != 1:
                continue
            mm = re.search(r'^[ \t]*let (?:mut )?' + re.escape(name) + r'(?:: [^=;]+)? = ([^\n;]*);[ \t]*\n', new, re.M)
            if not mm or mm.start() > occ[0].start() or mm.end() < occ[0].end():
                continue
            rhs = mm.group(1)
            if SIDE.search(rhs) or '?' in rhs or '.await' in rhs or ('(' in rhs and not re.search(r'(Instant::now|SystemTime::now|\.len|\.clone|\.to_string|\.to_vec|\.as_str|\.get)\(', rhs)):
                continue
            new = new[:mm.start()] + new[mm.end():]
            stats.setdefault('deadlets', []).append(name)
    return new, stats


def main():
    args = sys.argv[1:]
    apply = '--apply' in args
    jr = [a for a in args if a.startswith('--journal=')]
    files = [a for a in args if not a.startswith('--')]
    if not files:
        files = [l.rstrip('\n') for l in sys.stdin if l.strip()]
    root = os.getcwd()
    tot = {'files': 0, 'delete': 0, 'expr': 0, 'unhandled': 0, 'side': 0}
    rep = []
    for f in files:
        try:
            st = os.stat(f)
            text = open(f, encoding='utf-8').read()
        except Exception as e:
            rep.append({'file': f, 'error': str(e)})
            continue
        new, stats = process(f, text)
        if new == text and not stats['unhandled'] and not stats['side']:
            continue
        rep.append({'file': f, **{k: v for k, v in stats.items()}})
        tot['files'] += 1
        tot['delete'] += stats['delete']
        tot['expr'] += stats['expr']
        tot['unhandled'] += len(stats['unhandled'])
        tot['side'] += len(stats['side'])
        if apply and new != text:
            st2 = os.stat(f)
            if st2.st_mtime_ns != st.st_mtime_ns:
                rep[-1]['skipped'] = 'changed'
                continue
            real = os.path.realpath(f)
            assert real.startswith(root), real
            tmp = f + '.bd9tmp'
            open(tmp, 'w', encoding='utf-8').write(new)
            os.replace(tmp, f)
    print(json.dumps(tot))
    if jr:
        json.dump(rep, open(jr[0].split('=', 1)[1], 'w'), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
