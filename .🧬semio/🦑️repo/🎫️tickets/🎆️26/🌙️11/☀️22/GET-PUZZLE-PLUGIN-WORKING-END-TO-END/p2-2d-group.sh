#!/usr/bin/env bash
# 🧪 Lane 2 puzzle-2d package loop. Runs inside native-cargo.sh.
set -u
LOG_DIR="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/🗑️generated"
mkdir -p "$LOG_DIR"
GROUP="${1:?group}"
shift
fail=0
for spec in "$@"; do
  name="${spec%%|*}"
  extra="${spec#*|}"
  if [[ "$extra" == "$spec" ]]; then
    extra=""
  fi
  log="$LOG_DIR/p2-2d-${name}.log"
  echo "P2_BEGIN $name"
  # shellcheck disable=SC2086
  cargo test --jobs "${P2_JOBS:-2}" --offline --no-fail-fast --color never -p "$name" --all-targets $extra >"$log" 2>&1
  status=$?
  find /tmp/build -type f -perm -u+x ! -name "*.so" ! -name "*.rlib" ! -name "*.rmeta" ! -name "*build*script*" -delete 2>/dev/null || true
  summary=$(python3 -c 'import pathlib,sys
p=pathlib.Path(sys.argv[1])
t=p.read_text(errors="replace") if p.exists() else ""
rows=[ln.strip() for ln in t.splitlines() if ln.startswith("test result:") or "could not compile" in ln or ln.startswith("error[")]
print(" | ".join(rows[-6:]) if rows else "no-summary")
' "$log")
  echo "P2_END $name status=$status $summary"
  if [[ $status -ne 0 ]]; then
    fail=1
  fi
done
echo "P2_GROUP $GROUP fail=$fail"
exit $fail
