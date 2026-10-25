#!/usr/bin/env python3
"""bt.py apply LIST : port old build_tool_job (ToolOperationSpec::new) to admit_retained_tool_producer."""
import re, sys
sys.path.insert(0, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-b-tools")
import tjf

SIG = re.compile(r'(?P<pre>(?:pub(?:\([a-z]+\))?\s+)?)fn (?P<name>\w+)(?P<gen>(?:<[^()]*>)?)\s*\(\s*(?P<arg>\w+)\s*:\s*(?P<ty>(?:\w+::)*ArtifactOwnedToolJobRequest<.*?>)\s*\)\s*->\s*Result<Option<[\w:]*ToolOperationSpec>,\s*Fault>\s*\{', re.S)
NEWSIG = "{pre}fn {name}{gen}({arg}: &mut Option<{ty}>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {{"
VE = "semio_framework_plugin::__value::ValueError"
VK = "semio_framework_plugin::__value::ValueRefusalKind"

def split_args(t):
    out, d, cur, i = [], 0, '', 0
    while i < len(t):
        c = t[i]
        if c == '"':
            j = i + 1
            while t[j] != '"':
                j += 2 if t[j] == '\\' else 1
            cur += t[i:j+1]; i = j + 1; continue
        if c in '([{': d += 1
        if c in ')]}': d -= 1
        if c == ',' and d == 0:
            out.append(cur); cur = ''
        else:
            cur += c
        i += 1
    if cur.strip(): out.append(cur)
    return [a.strip() for a in out]

def stmt_span(text, start):
    """span of the statement starting at `start` (ends at the first top-level ';')"""
    d = 0; i = start
    while i < len(text):
        c = text[i]
        if c == '"':
            i += 1
            while text[i] != '"':
                i += 2 if text[i] == '\\' else 1
        elif c in '([{': d += 1
        elif c in ')]}': d -= 1
        elif c == ';' and d == 0: return start, i + 1
        i += 1
    raise ValueError

import os
def fail(n):
    if os.environ.get('BT_DEBUG'): print('   fail', n)
    return None

def convert(body, req, resume):
    m = re.search(r'let\s+(?:mut\s+)?payload\s*=\s*(?:\w+::)*ArtifactRetainedCommandPayload::new\(', body)
    if not m: return fail(1)
    p0 = m.end() - 1
    p1 = tjf.match_brace(body.replace('(', '{').replace(')', '}'), p0)
    args = split_args(body[p0+1:p1])
    if len(args) != 5: return fail(2)
    inputs, cmd, raw, items, work = args
    after = body[p1+1:]
    if not re.fullmatch(r'\s*;\s*Ok\(Some\([\w:]*ToolOperationSpec::new\([^;]*\)\)\)\s*', after, re.S): return fail(3)
    before = body[:m.start()]
    om = re.search(r'(?<![\w.])operation\s*(?::\s*(\w+))?\s*(?:,|$)', inputs)
    if not om: return fail(4)
    opname = om.group(1) or 'operation'
    removed = []
    ol = re.search(r'let\s+%s\s*=\s*(?:[\w:]*::)?AppOperationContext\s*\{' % opname, before)
    op_lit = None
    if ol:
        a0, a1 = stmt_span(before, ol.start())
        op_lit = before[ol.start():a1].rstrip(';')
        removed.append((a0, a1))
    wexpr = None
    wm = re.fullmatch(r'Box::new\((.*)\)', work, re.S)
    if wm:
        wexpr = wm.group(1).strip()
    elif re.fullmatch(r'\w+', work):
        wl = re.search(r'let\s+%s\s*(?::[^=;]+)?=\s*' % work, before)
        if not wl: return fail(5)
        a0, a1 = stmt_span(before, wl.start())
        txt = before[wl.start():a1]
        body_expr = re.fullmatch(r'let\s+\w+\s*(?::[^=;]+)?=\s*(.*?)\s*;', txt, re.S).group(1).strip()
        typed = ':' in txt[:txt.index('=')]
        bm = re.fullmatch(r'Box::new\((.*)\)', body_expr, re.S)
        wexpr = bm.group(1).strip() if (bm and not typed) else body_expr
        removed.append((a0, a1))
    else:
        wexpr = '{ let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<_>> = ' + work + '; work }'
    if re.search(r'\?|\breturn\b', wexpr): return fail(7)
    for a0, a1 in sorted(removed, reverse=True):
        before = before[:a0] + before[a1:]
    pre = before
    prelude = re.sub(r'return\s+([\w:<>]+)\(\s*%s\s*\)\s*;' % req, r'return \1(@@REQ@@, output, cx);', pre)
    pre = re.sub(r'\b%s\.' % req, 'original.', prelude)
    pre = pre.replace('return Ok(None)', 'return Ok(false)').replace('@@REQ@@', req)
    def fix_err(mm):
        msg = mm.group(1)
        return f'return Err({VE}::literal({VK}::InvariantViolated, {msg}).into())'
    pre = re.sub(r'return Err\(Fault::new\([^;]*?,\s*[^;]*?,\s*("[^"]*")\)\)', fix_err, pre)
    pre = re.sub(r'return Err\([\w:]*fault\(\s*"[^"]*"\s*,\s*("[^"]*")\s*\)\)', fix_err, pre)
    pre = re.sub(r'return Err\(Fault::from\(("[^"]*")\)\)', fix_err, pre)
    if re.search(r'\breturn\b', re.sub(r'return (?:Ok\(false\)|Err\([^;]*\)|[\w:<>]+\(%s, output, cx\));' % req, '', pre)): return fail(8)
    if re.search(r'\b%s\b(?![\w]*\s*,\s*output)' % req, re.sub(r'\(%s, output, cx\)' % req, '', pre)): return fail(9)
    opuse = re.search(r'\b%s\b' % opname, wexpr) is not None
    if opuse and op_lit is None: return fail(10)
    if opuse:
        lit = re.sub(r'\b%s\.' % req, 'request.', op_lit)
        lit = re.sub(r'(request\.(?:parent_document_id|authoring_seed))(?!\.clone\(\))(?=\s*[,}])', r'\1.clone()', lit)
        lit = re.sub(r'(request\.(?:parent_document_id|authoring_seed))\.clone\(\)\.clone\(\)', r'\1.clone()', lit)
        clos = f'|request| {{ {lit}; {wexpr} }}'
    else:
        clos = f'|request| {wexpr}'
    head = f'let original = {req}.as_ref().ok_or_else(|| {VE}::literal({VK}::InvariantViolated, "original retained producer request is absent"))?;'
    lines = [l for l in pre.strip('\n').split('\n') if l.strip()]
    out = '\n        ' + head + '\n'
    if lines: out += '\n'.join(lines) + '\n'
    out += f'        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer({req}, output, cx, {cmd}, {raw}, {items}, {resume}, {clos})\n    '
    return out

def convert_multi(body, req, resume):
    PAY = re.compile(r'let\s+(?:mut\s+)?payload\s*=\s*(?:\w+::)*ArtifactRetainedCommandPayload::new\(')
    pays = list(PAY.finditer(body))
    if not pays: return fail(101)
    text = body
    for m in reversed(pays):
        p0 = m.end() - 1
        p1 = tjf.match_brace(text.replace('(', '{').replace(')', '}'), p0)
        args = split_args(text[p0+1:p1])
        if len(args) != 5: return fail(102)
        inputs, cmd, raw, items, work = args
        rest = text[p1+1:]
        sm = re.match(r'\s*;\s*(return\s+)?Ok\(Some\([\w:]*ToolOperationSpec::new\(', rest)
        if not sm: return fail(103)
        q0 = p1 + 1 + sm.end() - 1
        q1 = tjf.match_brace(text.replace('(', '{').replace(')', '}'), q0)
        # after ToolOperationSpec::new(...) comes ')' of Some and ')' of Ok then optional ';'
        tail = re.match(r'\)\)\s*(;?)', text[q1+1:])
        if not tail: return fail(104)
        end = q1 + 1 + tail.end()
        is_return = bool(sm.group(1))
        semi = tail.group(1) == ';'
        om = re.search(r'(?<![\w.])operation\s*(?::\s*(\w+))?\s*(?:,|$)', inputs)
        if not om: return fail(105)
        opname = om.group(1) or 'operation'
        start = m.start()
        removed = []
        region = text[:start]
        ol = None
        for mm in re.finditer(r'let\s+%s\s*=\s*(?:[\w:]*::)?AppOperationContext\s*\{' % opname, region): ol = mm
        op_lit = None
        if ol:
            a0, a1 = stmt_span(region, ol.start())
            op_lit = region[ol.start():a1].rstrip(';')
            removed.append((a0, a1))
        wexpr = None
        wm = re.fullmatch(r'Box::new\((.*)\)', work, re.S)
        if wm: wexpr = wm.group(1).strip()
        elif re.fullmatch(r'\w+', work):
            wl = None
            for mm in re.finditer(r'let\s+%s\s*(?::[^=;]+)?=\s*' % work, region): wl = mm
            if not wl: return fail(106)
            a0, a1 = stmt_span(region, wl.start())
            t = region[wl.start():a1]
            be = re.fullmatch(r'let\s+\w+\s*(?::[^=;]+)?=\s*(.*?)\s*;', t, re.S).group(1).strip()
            typed = ':' in t[:t.index('=')]
            bm = re.fullmatch(r'Box::new\((.*)\)', be, re.S)
            wexpr = bm.group(1).strip() if (bm and not typed) else be
            removed.append((a0, a1))
        else: wexpr = '{ let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<_>> = ' + work + '; work }'
        if re.search(r'\?|\breturn\b', wexpr): return fail(108)
        opuse = re.search(r'\b%s\b' % opname, wexpr) is not None
        if opuse and op_lit is None: return fail(109)
        wexpr = re.sub(r'\b%s\.' % req, '@@RQ@@.', wexpr)
        if opuse:
            lit = re.sub(r'\b%s\.' % req, '@@RQ@@.', op_lit)
            lit = re.sub(r'(@@RQ@@\.(?:parent_document_id|authoring_seed))(?!\.clone\(\))(?=\s*[,}])', r'\1.clone()', lit)
            clos = f'|request| {{ {lit}; {wexpr} }}'
        else:
            clos = f'|request| {wexpr}'
        call = f'semio_framework_plugin::retained_command::producer::admit_retained_tool_producer({req}, output, cx, {cmd}, {raw}, {items}, {resume}, {clos})'
        new = ('return ' + call + ';') if is_return else call
        if is_return and not semi: new = 'return ' + call
        # splice: remove earlier statements (high index first), replace payload..end
        text = text[:start] + new + text[end:]
        for a0, a1 in sorted(removed, reverse=True):
            text = text[:a0] + text[a1:]
    # globally convert the rest
    text = re.sub(r'return\s+([\w:<>]+)\(\s*%s\s*\)\s*;' % req, r'return \1(@@REQ@@, output, cx);', text)
    text = re.sub(r'\b%s\.' % req, 'original.', text)
    text = text.replace('return Ok(None)', 'return Ok(false)').replace('@@REQ@@', req)
    text = re.sub(r'return Err\(Fault::new\([^;]*?,\s*[^;]*?,\s*("[^"]*")\)\)', lambda mm: f'return Err({VE}::literal({VK}::InvariantViolated, {mm.group(1)}).into())', text)
    text = re.sub(r'return Err\(Fault::from\(("[^"]*")\)\)', lambda mm: f'return Err({VE}::literal({VK}::InvariantViolated, {mm.group(1)}).into())', text)
    text = re.sub(r'return Err\([\w:]*fault\(\s*"[^"]*"\s*,\s*("[^"]*")\s*\)\)', lambda mm: f'return Err({VE}::literal({VK}::InvariantViolated, {mm.group(1)}).into())', text)
    # the closure params keep `request.`; undo the global rewrite inside closures
    def undo(mm):
        return mm.group(0).replace('original.', 'request.')
    text = re.sub(r'\|request\| (?:\{.*?\}\)(?=\s*[;\n}])|[^\n]*)', undo, text, flags=re.S) if False else text
    text = text.replace('@@RQ@@', 'request')
    head = f'let original = {req}.as_ref().ok_or_else(|| {VE}::literal({VK}::InvariantViolated, "original retained producer request is absent"))?;'
    return '\n        ' + head + '\n' + text.strip('\n') + '\n    '

def apply(path):
    s = open(path, encoding='utf-8').read()
    resume = 'true' if re.search(r'checkpoint_resume:\s*true', s) else 'false'
    done = 0; manual = []
    for m in reversed(list(SIG.finditer(s))):
        b = m.end() - 1
        e = tjf.match_brace(s, b)
        body = s[b+1:e]
        req = m.group('arg')
        if 'ToolOperationSpec::new' not in body:
            dm = re.fullmatch(r'\s*([\w:<>]+)\(\s*%s\s*\)\s*' % req, body)
            if dm:
                s = s[:m.start()] + NEWSIG.format(**{k: m.group(k) for k in ('pre', 'name', 'gen', 'arg', 'ty')}) + '\n        ' + dm.group(1) + f'({req}, output, cx)\n    ' + s[e:]
                done += 1
            else:
                manual.append((m.group('name'), 'nospec'))
            continue
        try:
            nb = convert(body, req, resume)
        except Exception as ex:
            nb = None
        if nb is None:
            try:
                nb = convert_multi(body, req, resume)
            except Exception as ex:
                nb = None
        if nb is None:
            manual.append((m.group('name'), 'shape')); continue
        s = s[:m.start()] + NEWSIG.format(**{k: m.group(k) for k in ('pre', 'name', 'gen', 'arg', 'ty')}) + nb + s[e:]
        done += 1
    if done: open(path, 'w', encoding='utf-8').write(s)
    print(f"{path}: ported {done}, manual {manual}")

if __name__ == '__main__':
    for p in open(sys.argv[2]).read().split('\n'):
        if p.strip(): apply(p)
