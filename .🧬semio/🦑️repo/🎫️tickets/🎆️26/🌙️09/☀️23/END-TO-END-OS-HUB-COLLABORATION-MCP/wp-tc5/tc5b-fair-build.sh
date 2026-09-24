#!/usr/bin/env zsh
set -u
ROOT=/Users/ueli/Documents/semio
WP="$ROOT/.tmp-ticket/wp-tc5"
GEN="$WP/generated"
MUTEX=$(ls "$ROOT/.tmp-ticket/"*fleet-mutex.sh | head -1)
HUB_SCRIPT=$(ls -d "$ROOT"/*hub/📦️packages/🦀️rust/📜️script.ts | head -1)
DATA=$(python3 -c 'from pathlib import Path
root=Path("/Users/ueli/Documents/semio")
for p in sorted(root.glob(".*")):
  for h in p.glob("*hub"):
    print(h/"tc5-boot"); raise SystemExit')
PKGS_FILE="$GEN/tc5b-packages.txt"
REPORT="$ROOT/.tmp-ticket/wp-tc5.md"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$WP/target"
export SEMIO_BUILD_BUDGET_MS=172800000
export OS_HUB_DATA="$DATA"
mkdir -p "$DATA" "$GEN"
chmod 700 "$DATA"

update_report() {
  local pkg="$1" wall="$2" rc="$3" note="$4"
  python3 - "$REPORT" "$pkg" "$wall" "$rc" "$note" <<'PY'
import sys
from pathlib import Path
report, pkg, wall, rc, note = sys.argv[1:6]
p = Path(report)
t = p.read_text()
row = f"| {pkg} | {wall} | {rc} | {note} |\n"
marker = "| (cutover) |"
# insert row after cutover row if present, else after table header
if f"| {pkg} |" in t and f"| {pkg} |" in t.split("### 4.4")[0]:
    # replace existing pkg row
    import re
    t2, n = re.subn(rf"\| {re.escape(pkg)} \|[^\n]*\n", row, t, count=1)
    if n:
        p.write_text(t2)
        print("replaced row", pkg)
        raise SystemExit
# append after cutover
if marker in t:
    i = t.find(marker)
    j = t.find("\n", i) + 1
    t = t[:j] + row + t[j:]
else:
    # after table header line containing package |
    key = "| package | wall_s |"
    i = t.find(key)
    if i >= 0:
        j = t.find("\n", t.find("\n", i)+1) + 1
        t = t[:j] + row + t[j:]
p.write_text(t)
print("appended row", pkg)
PY
}

START_FROM="${1:-}"
skip=1
if [ -z "$START_FROM" ]; then skip=0; fi

while IFS= read -r pkg; do
  [ -n "$pkg" ] || continue
  if [ "$skip" -eq 1 ]; then
    if [ "$pkg" = "$START_FROM" ]; then skip=0; else
      echo "skip $pkg (before $START_FROM)"
      continue
    fi
  fi
  # resume: skip if success evidence exists
  if [ -f "$GEN/tc5b-$pkg.txt" ] && grep -q "EXIT=0" "$GEN/tc5b-$pkg.txt" 2>/dev/null; then
    echo "already ok $pkg"
    continue
  fi
  echo "=== BEGIN $pkg $(date -Iseconds) lock=$(cat /tmp/semio-wasm-build.lock/owner 2>/dev/null) queue=$(ls /tmp/semio-wasm-build.queue 2>/dev/null | wc -l | tr -d ' ') ==="
  t0=$(date +%s)
  set +e
  zsh "$MUTEX" wasm tc5b -- env \
    NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 \
    CARGO_TARGET_DIR="$CARGO_TARGET_DIR" SEMIO_BUILD_BUDGET_MS="$SEMIO_BUILD_BUDGET_MS" \
    OS_HUB_DATA="$DATA" \
    bun "$HUB_SCRIPT" trusted-catalog-bootstrap --packages "$pkg" \
    > "$GEN/tc5b-$pkg.txt" 2>&1
  rc=$?
  set -e
  t1=$(date +%s)
  wall=$((t1 - t0))
  echo "EXIT=$rc WALL=${wall}s" >> "$GEN/tc5b-$pkg.txt"
  echo "=== END $pkg exit=$rc wall=${wall}s $(date -Iseconds) ==="
  tail -30 "$GEN/tc5b-$pkg.txt"
  note="ok"
  if [ "$rc" -ne 0 ]; then
    note="FAIL — see generated/tc5b-$pkg.txt"
    echo "$pkg $rc $wall" >> "$GEN/tc5b-failures.txt"
  fi
  update_report "$pkg" "$wall" "$rc" "$note"
  # on failure stop for root-fix (caller resumes)
  if [ "$rc" -ne 0 ]; then
    echo "STOP_ON_FAIL $pkg"
    exit "$rc"
  fi
done < "$PKGS_FILE"
echo "=== ALL_PACKAGES_ONE_BY_ONE_DONE $(date -Iseconds) ==="
exit 0
