#!/usr/bin/env python3
"""Brace-aware codemod: insert `menu: None,` into every UiNode-struct-literal missing the new
`menu: Option<UiMenuRef>` field, located via rustc's E0063 JSON diagnostics (span points at the
struct type name; we insert right after the following `{`). Same pattern as the repo's prior
"UiNode field addition" codemods (see memory: semio-react-parity-workflow).

Usage: cargo check -p <crate> --message-format=json > diag.json
       python3 insert_menu_field_codemod.py <real_file_path> diag.json
"""
import json
import sys

def main():
    real_file, diag_path = sys.argv[1], sys.argv[2]
    with open(real_file, "rb") as f:
        data = f.read()

    spans = []
    with open(diag_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            d = json.loads(line)
            if d.get("reason") != "compiler-message":
                continue
            msg = d["message"]
            code = msg.get("code") or {}
            if code.get("code") != "E0063":
                continue
            # only care about the missing `menu` field variant
            if "missing `menu`" not in msg.get("message", "") and "menu" not in msg.get("message", ""):
                continue
            span = msg["spans"][0]
            spans.append((span["byte_start"], span["byte_end"]))

    # dedupe + sort descending so earlier byte offsets stay valid as we insert
    spans = sorted(set(spans), key=lambda s: -s[0])
    inserted = 0
    for byte_start, byte_end in spans:
        # find the first `{` after the type name
        brace_pos = data.find(b"{", byte_end)
        if brace_pos == -1:
            print(f"WARN: no `{{` found after byte {byte_end}", file=sys.stderr)
            continue
        insertion = b"\n            menu: None,"
        data = data[: brace_pos + 1] + insertion + data[brace_pos + 1 :]
        inserted += 1

    with open(real_file, "wb") as f:
        f.write(data)
    print(f"Inserted `menu: None,` at {inserted} site(s).")

if __name__ == "__main__":
    main()
