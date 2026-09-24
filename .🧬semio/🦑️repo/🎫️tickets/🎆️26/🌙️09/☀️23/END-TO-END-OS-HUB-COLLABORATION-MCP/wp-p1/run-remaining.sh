#!/bin/zsh
set -e
cd /Users/ueli/Documents/semio
unset SEMIO_TEST_BUDGET_MS SEMIO_BUILD_BUDGET_MS CARGO_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
GEN=/Users/ueli/Documents/semio/.tmp-ticket/wp-p1/generated
mkdir -p "$GEN"
REPORT=/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md
echo "=== vcs start $(date -u +%H:%M:%S) ==="
OUT="$GEN/vcs-test-quick.txt"
{
  echo "=== vcs test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/vcs-plugin:test-quick
  echo "exit:$?"
  echo "=== vcs end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "vcs: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| vcs \\| pending \\|.*\\|", f"| vcs | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== mathematical start $(date -u +%H:%M:%S) ==="
OUT="$GEN/mathematical-test-quick.txt"
{
  echo "=== mathematical test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/mathematical-plugin:test-quick
  echo "exit:$?"
  echo "=== mathematical end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "mathematical: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| mathematical \\| pending \\|.*\\|", f"| mathematical | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== wfc start $(date -u +%H:%M:%S) ==="
OUT="$GEN/wfc-test-quick.txt"
{
  echo "=== wfc test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/wfc-plugin:test-quick
  echo "exit:$?"
  echo "=== wfc end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "wfc: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| wfc \\| pending \\|.*\\|", f"| wfc | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== procedural start $(date -u +%H:%M:%S) ==="
OUT="$GEN/procedural-test-quick.txt"
{
  echo "=== procedural test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/procedural-plugin:test-quick
  echo "exit:$?"
  echo "=== procedural end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "procedural: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| procedural \\| pending \\|.*\\|", f"| procedural | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== flow start $(date -u +%H:%M:%S) ==="
OUT="$GEN/flow-test-quick.txt"
{
  echo "=== flow test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/flow-plugin:test-quick
  echo "exit:$?"
  echo "=== flow end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "flow: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| flow \\| pending \\|.*\\|", f"| flow | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== animate start $(date -u +%H:%M:%S) ==="
OUT="$GEN/animate-test-quick.txt"
{
  echo "=== animate test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/animate-plugin:test-quick
  echo "exit:$?"
  echo "=== animate end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "animate: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| animate \\| pending \\|.*\\|", f"| animate | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== shooting start $(date -u +%H:%M:%S) ==="
OUT="$GEN/shooting-test-quick.txt"
{
  echo "=== shooting test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/shooting-plugin:test-quick
  echo "exit:$?"
  echo "=== shooting end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "shooting: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| shooting \\| pending \\|.*\\|", f"| shooting | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== demonstrator start $(date -u +%H:%M:%S) ==="
OUT="$GEN/demonstrator-test-quick.txt"
{
  echo "=== demonstrator test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/demonstrator-plugin:test-quick
  echo "exit:$?"
  echo "=== demonstrator end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "demonstrator: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| demonstrator \\| pending \\|.*\\|", f"| demonstrator | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== sequence start $(date -u +%H:%M:%S) ==="
OUT="$GEN/sequence-test-quick.txt"
{
  echo "=== sequence test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/sequence-plugin:test-quick
  echo "exit:$?"
  echo "=== sequence end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "sequence: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| sequence \\| pending \\|.*\\|", f"| sequence | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== fem start $(date -u +%H:%M:%S) ==="
OUT="$GEN/fem-test-quick.txt"
{
  echo "=== fem test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/fem-plugin:test-quick
  echo "exit:$?"
  echo "=== fem end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "fem: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| fem \\| pending \\|.*\\|", f"| fem | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== architect start $(date -u +%H:%M:%S) ==="
OUT="$GEN/architect-test-quick.txt"
{
  echo "=== architect test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/architect-plugin:test-quick
  echo "exit:$?"
  echo "=== architect end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "architect: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| architect \\| pending \\|.*\\|", f"| architect | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== process start $(date -u +%H:%M:%S) ==="
OUT="$GEN/process-test-quick.txt"
{
  echo "=== process test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/process-plugin:test-quick
  echo "exit:$?"
  echo "=== process end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "process: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| process \\| pending \\|.*\\|", f"| process | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== lowpoly start $(date -u +%H:%M:%S) ==="
OUT="$GEN/lowpoly-test-quick.txt"
{
  echo "=== lowpoly test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/lowpoly-plugin:test-quick
  echo "exit:$?"
  echo "=== lowpoly end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "lowpoly: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| lowpoly \\| pending \\|.*\\|", f"| lowpoly | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== reasoning start $(date -u +%H:%M:%S) ==="
OUT="$GEN/reasoning-test-quick.txt"
{
  echo "=== reasoning test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/reasoning-plugin:test-quick
  echo "exit:$?"
  echo "=== reasoning end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "reasoning: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| reasoning \\| pending \\|.*\\|", f"| reasoning | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
echo "=== forms start $(date -u +%H:%M:%S) ==="
OUT="$GEN/forms-test-quick.txt"
{
  echo "=== forms test-quick start $(date -u +%H:%M:%S) ==="
  bun nx run @semio-tech/forms-plugin:test-quick
  echo "exit:$?"
  echo "=== forms end $(date -u +%H:%M:%S) ==="
} >"$OUT" 2>&1 || true
EC=$(grep -E "^exit:" "$OUT" | tail -1 | cut -d: -f2)
SUM=$(grep -E "Summary|Successfully|failed|FAIL|exceeded" "$OUT" | tail -5 | tr "\n" " | ")
if [ "$EC" = "0" ]; then ST=PASS; else ST=FAIL; fi
echo "forms: $ST exit=$EC"
python3 -c 'from pathlib import Path; import re; p=Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-p1.md"); t=p.read_text(); t=re.sub(r"\\| forms \\| pending \\|.*\\|", f"| forms | {0} | exit={1}; {2} |".format("'"$ST"'", "'"$EC"'", "'"$SUM"'"[:120]), t, count=1); p.write_text(t)'
