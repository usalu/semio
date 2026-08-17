#!/usr/bin/env python3
import re, sys

PATTERNS = {
    'store_new': re.compile(r'\b\w*Store::new\('),
    'diff_call': re.compile(r'\.diff\('),
    'diff_fn': re.compile(r'fn\s+diff\s*\('),
    'validate_fn': re.compile(r'fn\s+validate\s*\('),
    'hint': re.compile(r'\bHint\b'),
    'crdt_vocab': re.compile(r'\b(merge_strategy|ConflictRule|reconcile_with_last|SpaceConflict|assert_crdt_\w+|MergeStrategyKind|merge_concurrent_diffs|ResolutionPlan|protocol_crdt)\b'),
    'apply_call': re.compile(r'\.apply\('),
    'envelope_construct': re.compile(r'ArtifactEnvelope\s*\{'),
}

def main(spans_file):
    with open(spans_file) as f:
        lines = [l.rstrip('\n') for l in f if l.strip()]
    by_file = {}
    for line in lines:
        path, start, end = line.split('\t')
        by_file.setdefault(path, []).append((int(start), int(end)))

    for path, spans in by_file.items():
        try:
            with open(path, encoding='utf-8', errors='replace') as f:
                filelines = f.readlines()
        except Exception as e:
            print(f"ERROR reading {path}: {e}")
            continue
        for start, end in spans:
            hits = []
            for ln in range(start, end+1):
                if ln-1 >= len(filelines):
                    continue
                text = filelines[ln-1]
                for pname, pat in PATTERNS.items():
                    if pat.search(text):
                        hits.append((ln, pname, text.strip()))
            if hits:
                print(f"=== {path} [{start}-{end}] ===")
                for ln, pname, text in hits:
                    print(f"  {ln}:{pname}: {text}")

if __name__ == '__main__':
    main(sys.argv[1])
