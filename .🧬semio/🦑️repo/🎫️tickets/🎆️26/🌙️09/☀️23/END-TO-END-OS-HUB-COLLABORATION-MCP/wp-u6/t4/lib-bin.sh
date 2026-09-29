#!/bin/zsh
# 🧪️ Prints the lib-test executable of one crate (built without running): lib-bin.sh <cargo test args…>
cargo test --offline --lib --no-run --message-format json "$@" 2>/dev/null | python3 -c '
import sys, json
for line in sys.stdin:
    try: message = json.loads(line)
    except Exception: continue
    if message.get("reason") == "compiler-artifact" and message.get("executable") and message["target"]["kind"] == ["lib"]: print(message["executable"])'
