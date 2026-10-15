import re

IDENT_START = re.compile(r'[A-Za-z_]')

def lex(lines):
    """Per-line info: depth_at_start, state_at_start ('code','str','blk','raw'), idents (code tokens), depth_at_end."""
    infos = []
    depth = 0
    state = 'code'
    blk = 0
    raw_hashes = 0
    for line in lines:
        info = {'depth': depth, 'state': state, 'idents': []}
        i = 0
        n = len(line)
        idents = info['idents']
        while i < n:
            c = line[i]
            if state == 'blk':
                if line.startswith('*/', i):
                    blk -= 1
                    i += 2
                    if blk == 0:
                        state = 'code'
                elif line.startswith('/*', i):
                    blk += 1
                    i += 2
                else:
                    i += 1
                continue
            if state == 'str':
                if c == '\\':
                    i += 2
                elif c == '"':
                    state = 'code'
                    i += 1
                else:
                    i += 1
                continue
            if state == 'raw':
                if c == '"' and line.startswith('"' + '#' * raw_hashes, i):
                    i += 1 + raw_hashes
                    state = 'code'
                else:
                    i += 1
                continue
            if line.startswith('//', i):
                break
            if line.startswith('/*', i):
                state = 'blk'
                blk = 1
                i += 2
                continue
            if c == '"':
                state = 'str'
                i += 1
                continue
            if c == 'r' or c == 'b' or c == 'c':
                m = re.match(r'(?:br|rb|r|cr)(#*)"', line[i:])
                if m and (i == 0 or not (line[i-1].isalnum() or line[i-1] == '_')):
                    raw_hashes = len(m.group(1))
                    state = 'raw'
                    i += m.end()
                    continue
                if c in 'bc' and line.startswith('"', i + 1) and (i == 0 or not (line[i-1].isalnum() or line[i-1] == '_')):
                    state = 'str'
                    i += 2
                    continue
                if c == 'b' and line.startswith("'", i + 1) and (i == 0 or not (line[i-1].isalnum() or line[i-1] == '_')):
                    i += 1
                    c = "'"
            if c == "'":
                if i + 1 < n and line[i+1] == '\\':
                    j = line.find("'", i + 3)
                    i = (j + 1) if j != -1 else n
                    continue
                if i + 2 < n and line[i+2] == "'":
                    i += 3
                    continue
                m = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", line[i:])
                if m:
                    i += m.end()
                    continue
                i += 1
                continue
            if c == '{':
                depth += 1
                i += 1
                continue
            if c == '}':
                depth -= 1
                i += 1
                continue
            if IDENT_START.match(c):
                m = re.match(r'[A-Za-z_][A-Za-z0-9_]*', line[i:])
                idents.append(m.group(0))
                i += m.end()
                continue
            if c.isdigit():
                m = re.match(r'[0-9][A-Za-z0-9_]*(?:\.[0-9][A-Za-z0-9_]*)?', line[i:])
                i += m.end() if m else 1
                continue
            i += 1
        info['end_depth'] = depth
        infos.append(info)
    return infos
