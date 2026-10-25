#!/usr/bin/env python3
"""tjf.py survey|apply FILELIST : port old ToolJobFactory impls (create_job*) to retained_command_factory_admission!."""
import re, sys, subprocess

def match_brace(s, i):
    d = 0; j = i
    in_str = False
    while j < len(s):
        c = s[j]
        if c == '"' :
            j += 1
            while j < len(s) and s[j] != '"':
                if s[j] == '\\': j += 1
                j += 1
        elif c == "'" and j + 2 < len(s) and (s[j+2] == "'" or (s[j+1] == '\\' and s[j+3] == "'")):
            j += 3 if s[j+2] == "'" else 4
        elif c == '/' and s.startswith('//', j):
            j = s.index('\n', j)
        elif c == '{': d += 1
        elif c == '}':
            d -= 1
            if d == 0: return j
        j += 1
    raise ValueError

IMPL = re.compile(r'impl(?:<[^{]*?>)?\s+(?:[\w:]*::)?ToolJobFactory\s+for\s+([^{]+?)\s*(?:where[^{]*)?\{', re.S)
FN = re.compile(r'\n[ \t]*fn (create_job_from_wire_pages_with_payload|create_job_from_wire|create_job)\b')

def fn_span(s, start, end):
    m = FN.search(s, start, end)
    if not m: return None
    spans = []
    return m

def sites(s):
    for m in IMPL.finditer(s):
        ob = m.end() - 1
        cb = match_brace(s, ob)
        yield m, ob, cb

def methods(s, ob, cb):
    out = {}
    for m in FN.finditer(s, ob, cb):
        name = m.group(1)
        k = s.index('{', s.index(')', m.start()) if False else m.start())
        # body brace: first '{' after the signature's return type: find "->" then first '{' after it at depth 0
        sig_end = s.index('->', m.start())
        b = s.index('{', sig_end)
        e = match_brace(s, b)
        out[name] = (m.start() + 1, e + 1, s[b:e+1], s[m.start():b])
    return out

def survey(path):
    s = open(path, encoding='utf-8').read()
    for m, ob, cb in sites(s):
        body = s[ob:cb]
        jt = re.search(r'type Job\s*=\s*([^;]+);', body)
        meth = methods(s, ob, cb)
        w = meth.get('create_job_from_wire_pages_with_payload')
        print(f"{path}\n  impl {m.group(1)}\n  Job={jt.group(1).strip() if jt else None}\n  fns={list(meth)}")
        if w:
            print("  WIRE:", re.sub(r'\s+', ' ', w[2])[:700])


MACRO = "semio_framework_plugin::retained_command::retained_command_factory_admission!"

def policy(wire):
    b = re.sub(r'\s+', ' ', wire)
    m = re.search(r'input\.declared_bytes\(\)\s*>\s*(.+?)\s*(\|\||\{)', b)
    if not m: return None
    mx = m.group(1).strip()
    if m.group(2) == '{':
        r = re.search(r'Some\(checkpoint\) if (E::\w+) =>', b)
        if not r: return None
        resume = r.group(1)
    elif 'checkpoint.is_some()' in b[m.end()-2:m.end()+40]:
        resume = 'false'
    elif 'is_some_and' in b and 'from_wire_with_checkpoint' in b:
        resume = 'true'
    else:
        return None
    if mx.startswith('payload.'):
        mx = '|payload| ' + mx
    return mx, resume

def line_start(s, i):
    return s.rfind('\n', 0, i) + 1

def with_attrs(s, start):
    ls = line_start(s, start)
    while True:
        prev = s.rfind('\n', 0, ls - 1) + 1
        line = s[prev:ls].strip()
        if line.startswith('///') or line.startswith('#['):
            ls = prev
        else:
            break
    return ls

def apply(path):
    s = open(path, encoding='utf-8').read()
    done = 0; skipped = []
    for m, ob, cb in reversed(list(sites(s))):
        body = s[ob:cb]
        jt = re.search(r'type Job\s*=\s*([^;]+);', body)
        meth = methods(s, ob, cb)
        if not meth: continue
        if not jt or 'ArtifactRetainedCommandJob' not in jt.group(1) or ('create_job_from_wire_pages_with_payload' not in meth and 'retained_command_factory_admission' not in body):
            skipped.append((m.group(1), 'shape')); continue
        w = meth.get('create_job_from_wire_pages_with_payload', (0, 0, '', ''))
        has_macro = 'retained_command_factory_admission' in body
        pol = None if has_macro else policy(w[2])
        if not pol and not has_macro:
            skipped.append((m.group(1), 'policy')); continue
        mx, resume = pol if pol else (None, None)
        spans = sorted((with_attrs(s, v[0]), s.index('\n', v[1]) + 1 if s[v[1]:v[1]+1] != '\n' else v[1] + 1) for v in meth.values())
        first = spans[0][0]
        indent = re.match(r'[ \t]*', s[first:]).group(0)
        text = s
        for a, b in reversed(spans):
            text = text[:a] + text[b:]
        if not has_macro:
            stmt = f"{indent}{MACRO}(maximum_raw_bytes: {mx}, checkpoint_resume: {resume});\n"
            text = text[:first] + stmt + text[first:]
        s = text
        done += 1
    if done:
        open(path, 'w', encoding='utf-8').write(s)
    print(f"{path}: ported {done}, skipped {skipped}")

RESERVED = "semio_framework_plugin::retained_command::reserved_tool_job_factory_admission!();"

def apply_reserved(path):
    s = open(path, encoding='utf-8').read()
    done = 0
    for m, ob, cb in reversed(list(sites(s))):
        body = s[ob:cb]
        pt = re.search(r'type Payload\s*=\s*([^;]+);', body)
        jt = re.search(r'type Job\s*=\s*([^;]+);', body)
        meth = methods(s, ob, cb)
        if not pt or not jt or pt.group(1).strip() != 'ArtifactReservedToolJob' or jt.group(1).strip() != 'ArtifactReservedToolJob': continue
        if list(meth) != ['create_job'] or not re.sub(r'\s+', '', meth['create_job'][2]) == '{Ok(payload)}': continue
        a, b = meth['create_job'][0], meth['create_job'][1]
        a = with_attrs(s, a)
        b = s.index('\n', b) + 1
        indent = re.match(r'[ \t]*', s[a:]).group(0)
        s = s[:a] + indent + RESERVED + "\n" + s[b:]
        done += 1
    if done: open(path, 'w', encoding='utf-8').write(s)
    print(f"{path}: reserved {done}")

if __name__ == '__main__':
    mode = sys.argv[1]
    for p in open(sys.argv[2]).read().split('\n'):
        if p.strip():
            if mode == 'survey': survey(p)
            elif mode == 'apply': apply(p)
            elif mode == 'reserved': apply_reserved(p)
