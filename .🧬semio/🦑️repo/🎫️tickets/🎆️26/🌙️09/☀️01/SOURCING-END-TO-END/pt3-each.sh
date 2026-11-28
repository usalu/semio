#!/bin/zsh
BIN="/Users/ueli/t/src-pt3/b/debug/build/semio-plugin-tests-14-2/481d7318896469ec/out/semio_plugin_tests_14_2-481d7318896469ec"
export RUST_MIN_STACK=134217728
mkdir -p /tmp/pt2out 2>/dev/null
OUT="${PT2_OUT:-/Users/ueli/t/src-pt3/out}"; mkdir -p "$OUT"
names=("$@"); [ ${#names[@]} -eq 0 ] && names=($("$BIN" --list 2>/dev/null | grep ': test' | sed 's/: test//'))
for name in $names; do
  timeout 240 "$BIN" --exact "$name" > "$OUT/$name.log" 2>&1; code=$?
  if [ $code -eq 0 ]; then echo "PASS $name"; else echo "FAIL($code) $name"; grep -a -A3 "panicked at" "$OUT/$name.log" | head -6 | cut -c1-1500; fi
done
