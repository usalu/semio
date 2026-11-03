import re, sys, glob, os
ROOT = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts'
SUB = '🏅️standards/🔖️1/🪆️subsets/✳️any'
rule_re = re.compile(r'SetFieldRule\s*\{\s*path:\s*"([^"]+)"')
lit_re = re.compile(r'"([^"\n]*)"')
leaf_re = re.compile(r'^[a-z][A-Za-z0-9]*$')
for art in sorted(glob.glob(ROOT + '/*/')):
    handles = art + SUB + '/🎬️scene/🕹️handles/🦀️.rs'
    rules = art + SUB + '/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs'
    if not (os.path.exists(handles) and os.path.exists(rules)):
        continue
    rule_paths = set(rule_re.findall(open(rules).read()))
    rule_leaves = {p.rsplit('.', 1)[-1] for p in rule_paths}
    src = open(handles).read()
    code = '\n'.join(l for l in src.splitlines() if not l.lstrip().startswith('//'))
    found = {}
    for m in re.finditer(r'(?:format!\(\s*|path\(|_path\([^,)]*,\s*|member_path\([^,)]*,\s*)"([^"]*)"', code):
        lit = m.group(1)
        norm = re.sub(r'\[id=\{[^}]*\}\]', '[]', lit)
        leaf = norm.rsplit('.', 1)[-1]
        if re.fullmatch(r'\{[^}]*\}', leaf) or not leaf_re.match(leaf):
            continue
        found.setdefault(norm, leaf)
    for m in re.finditer(r'\bpath:\s*"([^"]+)"|binding\(\s*"([^"]+)"|FieldBinding::new\(\s*"([^"]+)"', code):
        lit = next(g for g in m.groups() if g)
        found.setdefault(lit, lit.rsplit('.', 1)[-1])
    id_removals = set(re.findall(r'IdRemoval\s*\{\s*path:\s*"([^"]+)"', open(rules).read()))
    exact = sorted(p for p in found if '{' not in p and not p.startswith('view.') and p not in rule_paths and '[]' not in p.replace('[]', '', 0) or ('{' not in p and p not in rule_paths and '[' in p))
    exact = sorted(set(p for p in exact if p not in rule_paths))
    missing = sorted(p for p, leaf in found.items() if leaf not in rule_leaves and p not in rule_paths and leaf_re.match(leaf))
    print(os.path.basename(art.rstrip('/')), 'paths', len(found), 'rule leaves', len(rule_leaves), 'LEAF-MISSING', missing, 'EXACT-MISSING', exact)
