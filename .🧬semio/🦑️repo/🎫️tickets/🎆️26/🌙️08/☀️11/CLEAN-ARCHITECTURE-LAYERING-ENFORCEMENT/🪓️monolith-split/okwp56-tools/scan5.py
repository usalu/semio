import em, collections, re, rs
t = em.read(); tab = em.method_table(t)
L = t.split('\n')
def sig(m):
    # text from header line to the line containing the opening brace of the body
    txt = []
    for i in range(m['h'] - 1, m['e']):
        txt.append(L[i])
        st = rs.strip('\n'.join(txt))
        if '{' in st or st.rstrip().endswith(';'): break
    s = '\n'.join(txt)
    st = rs.strip(s)
    k = st.index('{') if '{' in st else len(st)
    return s[:k].strip()
recv = collections.Counter(); gens = []; oddsig = []
for n, m in tab.items():
    if n.startswith('<'): continue
    s = sig(m); m['sig'] = s
    r = re.search(r'\(\s*(&\s*(?:\'\w+\s+)?(?:mut\s+)?self|mut self|self)\b', s)
    recv[r.group(1) if r else 'static'] += 1
    if re.search(r'fn\s+\w+\s*<', s): gens.append((n, re.search(r'fn\s+\w+\s*(<[^(]*>)\s*\(', s, re.S).group(1)[:80] if re.search(r'fn\s+\w+\s*(<[^(]*>)\s*\(', s, re.S) else '?'))
    if 'where' in rs.strip(s).split(')')[-1] or '-> impl' in s or "'a" in s: oddsig.append((n, ' '.join(s.split())[:200]))
print(recv)
print('generic fns', len(gens)); [print('  ', g) for g in gens]
print('odd', len(oddsig)); [print('  ', g) for g in oddsig]
asy = sum(1 for n, m in tab.items() if not n.startswith('<') and re.match(r'.*\basync\s+fn', m['sig'].replace('\n',' ')))
print('async', asy)
print('--- params/bodies mentioning Self or ArtifactStore other than receiver')
for n, m in tab.items():
    if n.startswith('<'): continue
    body = rs.strip(m['text'])
    hits = re.findall(r'(?<![\w:])(?:Self|ArtifactStore)\b(?!::[a-z_]+\()[^;\n]{0,60}', body)
    hits = [h for h in hits if not h.startswith('Self::')]
    if hits: print(n, hits[:4])
