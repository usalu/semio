"""🧺️ S3-CLOSURE step 4: rewrites every hand-declared one-item fold footprint whose enclosing function holds the mutation
into `ArtifactStoreOneItemFootprint::for_leaf(mutation, bytes)` (design §20.5). `--write` applies; otherwise it lists."""
import re, subprocess, sys
ROOT = '/Users/ueli/Documents/semio'
SKIP = ('/🧪️tests/', '/🏪️store/', '/🔌️plugin/🧪️tests/', '.🧬semio/')
PATTERN = re.compile(r'(?P<path>(?:store|app_store|fixture_store)::ArtifactStoreOneItemFootprint)(?:::for_one_invertible_item\((?P<bytes1>(?:[^()]|\([^()]*(?:\([^()]*\))*[^()]*\))*)\)|\s*\{\s*work_items:\s*\d+,\s*retained_bytes(?::\s*(?P<bytes2>[^{}]+?))?\s*\})')
SIGNATURE = re.compile(r'fn (?P<name>\w+)(?P<generics><[^>{]*>)?\((?P<params>[^)]*)\)\s*->')
files = subprocess.run(['git', 'grep', '-l', '-E', r'for_one_invertible_item|ArtifactStoreOneItemFootprint \{ work_items', '--', '*.rs'], cwd=ROOT, capture_output=True, text=True).stdout.split()
write = '--write' in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith('--')]
for path in files:
    if any(s in path for s in SKIP) or (only and not any(o in path for o in only)):
        continue
    text = open(f'{ROOT}/{path}', encoding='utf-8').read()
    edits = []
    for match in PATTERN.finditer(text):
        head = text[:match.start()]
        signature = list(SIGNATURE.finditer(head))[-1]
        params = signature.group('params')
        param = re.search(r'\b(_?mutation)\s*:\s*&(?P<ty>[\w:<>]+)', params)
        bytes_expr = (match.group('bytes1') or match.group('bytes2') or 'retained_bytes').strip()
        line = head.count('\n') + 1
        if not param:
            print(f'MANUAL {path.split("/")[2]} {path.split("/")[-2]}:{line} fn {signature.group("name")}({params[:90]})')
            continue
        edits.append((match.start(), match.end(), f'{match.group("path")}::for_leaf(mutation, {bytes_expr})', signature, param))
        print(f'AUTO   {path.split("/")[2]} {path.split("/")[-2]}:{line} fn {signature.group("name")} [{param.group(1)}: &{param.group("ty")}] -> for_leaf(mutation, {bytes_expr[:60]})')
    if write and edits:
        spans = {(start, end): new for start, end, new, signature, param in edits}
        for start, end, new, signature, param in edits:
            if param.group(1) == '_mutation':
                s = signature.start('params') + param.start(1)
                spans[(s, s + 9)] = 'mutation'
        for (start, end), new in sorted(spans.items(), key=lambda item: -item[0][0]):
            text = text[:start] + new + text[end:]
        open(f'{ROOT}/{path}', 'w', encoding='utf-8').write(text)
