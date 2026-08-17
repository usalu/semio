#!/usr/bin/env python3
import re, sys, json

CFG_RE = re.compile(r'#\[cfg\((?:[^)]*\btarget_arch\s*=\s*"wasm32"[^)]*|[^)]*\bfeature\s*=\s*"wasm"[^)]*)\)\]')

def find_spans(path):
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        lines = f.readlines()
    spans = []
    n = len(lines)
    i = 0
    while i < n:
        line = lines[i]
        if CFG_RE.search(line):
            cfg_start = i
            # skip subsequent attribute lines (e.g. #[cfg_attr...], #[wasm_bindgen...], doc comments)
            j = i + 1
            while j < n:
                stripped = lines[j].strip()
                if stripped.startswith('#[') or stripped.startswith('///') or stripped.startswith('//!') or stripped == '':
                    j += 1
                    continue
                break
            if j >= n:
                break
            target_line = lines[j]
            # find first '{' from j onward on the same logical item header (could span multiple lines e.g. fn sig)
            # find opening brace searching forward until we hit a '{' or a ';' (item w/o body, e.g. `use` or attribute on a single stmt)
            k = j
            brace_line = None
            brace_col = None
            found_semicolon_first = False
            while k < n:
                text = lines[k]
                brace_idx = text.find('{')
                semi_idx = text.find(';')
                if brace_idx != -1 and (semi_idx == -1 or brace_idx < semi_idx):
                    brace_line = k
                    brace_col = brace_idx
                    break
                if semi_idx != -1:
                    found_semicolon_first = True
                    break
                k += 1
            if brace_line is None:
                # no brace found; treat as single-line/statement span
                spans.append((cfg_start+1, j+1, path))
                i = j + 1
                continue
            # brace matching from brace_line/brace_col
            depth = 0
            end_line = None
            in_str = False
            in_char = False
            in_line_comment = False
            in_block_comment = False
            prev_char = ''
            for li in range(brace_line, n):
                text = lines[li]
                start_col = brace_col if li == brace_line else 0
                ci = start_col
                in_line_comment = False
                while ci < len(text):
                    c = text[ci]
                    c2 = text[ci:ci+2]
                    if in_block_comment:
                        if c2 == '*/':
                            in_block_comment = False
                            ci += 2
                            continue
                        ci += 1
                        continue
                    if in_line_comment:
                        break
                    if in_str:
                        if c == '\\':
                            ci += 2
                            continue
                        if c == '"':
                            in_str = False
                        ci += 1
                        continue
                    if in_char:
                        if c == '\\':
                            ci += 2
                            continue
                        if c == "'":
                            in_char = False
                        ci += 1
                        continue
                    if c2 == '//':
                        in_line_comment = True
                        break
                    if c2 == '/*':
                        in_block_comment = True
                        ci += 2
                        continue
                    if c == '"':
                        in_str = True
                        ci += 1
                        continue
                    if c == "'":
                        # could be lifetime; heuristic: char literal if pattern 'x' or '\x'
                        # just treat naively - lifetimes rarely cause brace issues, skip robustly
                        # look ahead: if next-next char is a quote or escape-quote treat as char literal
                        nxt = text[ci+1:ci+3]
                        if len(nxt) >= 2 and nxt[1] == "'":
                            in_char = True
                        elif len(nxt) >= 1 and nxt[0] == '\\':
                            in_char = True
                        ci += 1
                        continue
                    if c == '{':
                        depth += 1
                    elif c == '}':
                        depth -= 1
                        if depth == 0:
                            end_line = li
                            break
                    ci += 1
                if end_line is not None:
                    break
            if end_line is None:
                end_line = n - 1
            spans.append((cfg_start+1, end_line+1, path))
            i = end_line + 1
        else:
            i += 1
    return spans

if __name__ == '__main__':
    files_list = sys.argv[1]
    with open(files_list) as f:
        files = [l.strip() for l in f if l.strip()]
    all_spans = []
    for fpath in files:
        try:
            spans = find_spans(fpath)
        except Exception as e:
            print(f"ERROR {fpath}: {e}", file=sys.stderr)
            continue
        for s in spans:
            all_spans.append(s)
    for start, end, path in all_spans:
        print(f"{path}\t{start}\t{end}")
