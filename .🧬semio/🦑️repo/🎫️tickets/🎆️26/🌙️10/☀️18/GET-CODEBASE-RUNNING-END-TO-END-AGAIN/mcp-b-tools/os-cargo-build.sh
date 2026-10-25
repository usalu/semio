#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/mcp-b"
nice -n 10 repo cargo --store=shared --configuration=prototype -- build --manifest-path "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml" -p semio-framework-os-mcp --bin semio-os-mcp --message-format=json-render-diagnostics > "$G/os-cargo-build.log" 2>&1
rc=$?
exe=$(/usr/bin/grep -a '"reason":"compiler-artifact"' "$G/os-cargo-build.log" | /usr/bin/grep -a '"name":"semio-os-mcp"' | tail -1 | python3 -c "import sys,json
for l in sys.stdin:
    d=json.loads(l); print(d.get('executable') or '')")
[ -n "$exe" ] && cp "$exe" "$G/semio-os-mcp" && echo "copied $exe" >> "$G/os-cargo-build.copy"
exit $rc
