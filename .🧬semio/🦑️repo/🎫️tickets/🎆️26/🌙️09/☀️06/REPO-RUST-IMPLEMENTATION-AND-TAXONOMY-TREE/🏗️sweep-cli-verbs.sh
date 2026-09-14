#!/usr/bin/env bash
# 🧮️ Final verb sweep: every verb this ticket wired, both binaries, one diff each.
set -u
O="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🗑️generated/cli-verbs"
G="C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/semio-repo.exe"
R="C:/git/semio/target/release/semio.exe"
W="C:/Users/Ueli/AppData/Local/Temp/claude/C--git-semio/5b49befa-cd0c-465f-af94-3d28e7c9ed64/scratchpad/miniRepo"
mkdir -p "$O"

pass=0
fail=0

norm() { sed -E 's/"second":"[^"]*"/"second":"T"/g; s/"snapshot": "[0-9a-f]{64}"/"snapshot": "S"/; s/(go|rs)\.jsonl/OUT/; s/target\(s\) in [0-9.]+s/target(s) in Ts/; s/finished in [0-9.]+s/finished in Ts/; s/[0-9]+\.[0-9]+s$/Ts/' ; }

run() { # run <label> <cwd> <args...>
  local label="$1"; shift
  local cwd="$1"; shift
  ( cd "$cwd" && timeout 300 "$G" "$@" ) > "$O/x.go.raw" 2>&1; local ge=$?
  ( cd "$cwd" && timeout 300 "$R" "$@" ) > "$O/x.rs.raw" 2>&1; local re=$?
  norm < "$O/x.go.raw" > "$O/x.go.txt"
  norm < "$O/x.rs.raw" > "$O/x.rs.txt"
  if diff -q "$O/x.go.txt" "$O/x.rs.txt" > /dev/null && [ "$ge" -eq "$re" ]; then
    echo "PASS  $label (exit $ge)"
    pass=$((pass+1))
  else
    echo "FAIL  $label (go=$ge rs=$re)"
    diff "$O/x.go.txt" "$O/x.rs.txt" | head -10
    fail=$((fail+1))
  fi
}

runin() { # runin <label> <cwd> <stdin> <args...>
  local label="$1"; shift
  local cwd="$1"; shift
  local payload="$1"; shift
  ( cd "$cwd" && echo "$payload" | timeout 300 "$G" "$@" ) > "$O/x.go.raw" 2>&1; local ge=$?
  ( cd "$cwd" && echo "$payload" | timeout 300 "$R" "$@" ) > "$O/x.rs.raw" 2>&1; local re=$?
  norm < "$O/x.go.raw" > "$O/x.go.txt"
  norm < "$O/x.rs.raw" > "$O/x.rs.txt"
  if diff -q "$O/x.go.txt" "$O/x.rs.txt" > /dev/null && [ "$ge" -eq "$re" ]; then
    echo "PASS  $label (exit $ge)"
    pass=$((pass+1))
  else
    echo "FAIL  $label (go=$ge rs=$re)"
    diff "$O/x.go.txt" "$O/x.rs.txt" | head -10
    fail=$((fail+1))
  fi
}

SEMIO="C:/git/semio"
SMALL="C:/git/eg-ice-23.semio-tech.com"

echo "== auth =="
run "auth whoami"        "$SEMIO" auth whoami
run "auth status"        "$SEMIO" auth status

echo "== configure / micro-commit =="
run "configure"          "$SEMIO" configure
PATH="/usr/bin:/bin" run "micro-commit reset (no bun)" "$W" micro-commit reset

echo "== update =="
run "update"             "$SEMIO" update
run "update rust"        "$SEMIO" update rust
run "update --dry-run"   "$SEMIO" update --dry-run
run "update --apply (no dependabot)" "$W" update --apply

echo "== benchmark =="
run "benchmark --dry-run" "$SEMIO" benchmark --dry-run

echo "== ticket purge-artifacts =="
run "ticket purge-artifacts --all"            "$SEMIO" ticket purge-artifacts --all
run "ticket purge-artifacts 26/09/06/…"       "$SEMIO" ticket purge-artifacts 26/09/06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE

echo "== test =="
run "test (all)"                      "$W" test
run "test 🧰️demo"                     "$W" test "🧰️demo"
run "test repo://p/u/demo"            "$W" test "repo://p/u/demo"
run "test bundle engine"              "$W" test "repo://p/u/@demo/b/l/engine"
run "test bundle kernel"              "$W" test "repo://p/u/@demo/b/l/kernel"
run "test bundle core"                "$W" test "repo://p/u/@demo/b/l/core"

echo "== mermaid =="
run "mermaid loc-by-technologies-bundles-folders-files" "$W" mermaid loc-by-technologies-bundles-folders-files
run "mermaid loc-by-language"                           "$W" mermaid loc-by-language
run "mermaid loc-by-contributors"                       "$W" mermaid loc-by-contributors

echo "== loc =="
run "loc --json"                    "$SMALL" loc --json
run "loc --md"                      "$SMALL" loc --md
run "loc --text"                    "$SMALL" loc --text
run "loc --json --by-contributors"  "$SMALL" loc --json --by-contributors
run "loc --md --by-contributors"    "$SMALL" loc --md --by-contributors
run "loc --json --history"          "$SMALL" loc --json --history
run "loc --json --languages Go,Rust" "$SMALL" loc --json --languages Go,Rust

echo "== export =="
rm -f "$W/go.jsonl" "$W/rs.jsonl"
( cd "$W" && timeout 300 "$G" export "$W/go.jsonl" ) > "$O/x.go.raw" 2>&1; ge=$?
( cd "$W" && timeout 300 "$R" export "$W/rs.jsonl" ) > "$O/x.rs.raw" 2>&1; re=$?
norm < "$O/x.go.raw" > "$O/x.go.txt"; norm < "$O/x.rs.raw" > "$O/x.rs.txt"
if diff -q "$O/x.go.txt" "$O/x.rs.txt" > /dev/null && [ "$ge" -eq "$re" ]; then echo "PASS  export (exit $ge)"; pass=$((pass+1)); else echo "FAIL  export"; diff "$O/x.go.txt" "$O/x.rs.txt"|head -10; fail=$((fail+1)); fi

echo "== contributor =="
rm -rf "$W/.🧬semio/🦑️repo/🧑️‍💻️devs"
( cd "$W" && timeout 60 "$G" contributor add gouser "Go User" go@example.com --json ) > "$O/x.go.raw" 2>&1; ge=$?
cp "$W/.🧬semio/🦑️repo/🧑️‍💻️devs/gouser/🧑️‍💻️contributor.json" "$O/c.go.doc" 2>/dev/null
rm -rf "$W/.🧬semio/🦑️repo/🧑️‍💻️devs"
( cd "$W" && timeout 60 "$R" contributor add gouser "Go User" go@example.com --json ) > "$O/x.rs.raw" 2>&1; re=$?
cp "$W/.🧬semio/🦑️repo/🧑️‍💻️devs/gouser/🧑️‍💻️contributor.json" "$O/c.rs.doc" 2>/dev/null
if diff -q "$O/x.go.raw" "$O/x.rs.raw" >/dev/null && diff -q "$O/c.go.doc" "$O/c.rs.doc" >/dev/null && [ "$ge" -eq "$re" ]; then echo "PASS  contributor add (stdout + document)"; pass=$((pass+1)); else echo "FAIL  contributor add"; diff "$O/x.go.raw" "$O/x.rs.raw"|head -6; diff "$O/c.go.doc" "$O/c.rs.doc"|head -6; fail=$((fail+1)); fi
run "contributor remove" "$W" contributor remove gouser --json

echo "== sync management (offline) =="
PATH="/usr/bin:/bin" run "sync management" "$W" sync management --json

echo "== ticket delete (mutation surface) =="
run "graphql ticketDelete" "$W" graphql 'mutation { ticketDelete(input: {year: 26, month: 9, day: 6, slug: "DEMO", noManagement: true}) }' --json

echo
echo "TOTAL pass=$pass fail=$fail"
