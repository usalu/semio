import os, re, sys
root = sys.argv[1]
apply = len(sys.argv) > 2 and sys.argv[2] == 'apply'
pat = re.compile(r'impl\b[^{;]*\bErasedSnapshotRetirement\s+for\s+([^{]*?)(?:\s+where[^{]*)?\s*\{', re.S)
use_pat = re.compile(r'(semio_framework_value(?:_retirement)?)::(?:\{[^;]*\bErasedSnapshotRetirement\b|ErasedSnapshotRetirement\b)')
LINE = 'fn native_header_release(&self)->Option<{c}::RetirementHeaderRelease>{{self.terminal_is_empty().then(||unsafe{{{c}::RetirementHeaderRelease::native(self,0,0)}})}}\n'
done = unresolved = 0
for d, _, fs in os.walk(root):
    for f in fs:
        if not f.endswith('.rs'):
            continue
        p = os.path.join(d, f)
        if 'tests' in p or 'testing' in p:
            continue
        try:
            s = open(p, encoding='utf-8', newline='').read()
        except Exception:
            continue
        crate = 'semio_framework_value'
        um = use_pat.search(s)
        if um:
            crate = um.group(1)
        else:
            pm = re.search(r'(semio_framework_value(?:_retirement)?)::ErasedSnapshotRetirement for', s)
            if pm:
                crate = pm.group(1)
        out = []
        pos = 0
        changed = False
        for m in pat.finditer(s):
            i = m.end()
            depth = 1
            j = i
            while j < len(s) and depth:
                c = s[j]
                depth += (c == '{') - (c == '}')
                j += 1
            body = s[i:j]
            if 'native_header_release' in body or 'terminal_is_empty' not in body:
                continue
            if crate is None:
                print('UNRESOLVED', p, m.group(1).strip())
                unresolved += 1
                continue
            nl = '\r\n' if '\r\n' in s else '\n'
            out.append(s[pos:i])
            out.append(nl + '    ' + LINE.format(c=crate).rstrip('\n'))
            pos = i
            changed = True
            done += 1
            print('PATCH', crate, p.split('store')[-1][-60:], m.group(1).strip())
        if changed and apply:
            out.append(s[pos:])
            open(p, 'w', encoding='utf-8', newline='').write(''.join(out))
print('patched', done, 'unresolved', unresolved)
