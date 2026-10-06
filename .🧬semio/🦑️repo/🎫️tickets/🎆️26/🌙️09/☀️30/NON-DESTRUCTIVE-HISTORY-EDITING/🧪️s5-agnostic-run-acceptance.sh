#!/bin/zsh
# ⏪️ S5-AGNOSTIC (fleet rule 48, TESTS TARGETED): runs the cross-plugin acceptance laws of ONE plugin crate in ONE gated foreground
# `cargo test` — `history_edit_acceptance_law!` (`history_edits_end_to_end`, `history_edit_inputs_resolve`), `composed_reload_law!`
# (`documents_reload_identically`), `composed_child_history_law!` (`child_history_edits_end_to_end`) and the derive-emitted payload laws
# (`semio_payload_law_*`) — writes the full output to 🗑️generated/s5-agnostic/<crate>.txt and appends one row to
# 🗑️generated/s5-agnostic/acceptance-results.tsv (time, crate, exit, class, G12, inputs, reload, child, payload pass/fail, summary).
# Single-flight and re-issuable: a call that finds its crate already running waits for it (a Bash call is capped at 10 min), a call that
# finds it finished only reports; `--force` runs it again. The gate is build gate v5 (rules 55/59: < 4 cargo, `CARGO_BUILD_JOBS=3`,
# one extra cycle while another WP holds `landing`) plus "no coordinator activation holds `landing`" plus "the foundation is neither BUILDING nor RED
# younger than 6 min" (rules 56/63, `🗑️generated/coord/foundation.status`) plus "my own harness landing is not in flight" plus "no other acceptance run of mine is alive"; below 25 GiB free
# nothing starts. When every wired law passes, the crate's own lib-test
# executable (a leaf deliverable of this run in the shared build dir, 100–300 MiB) is removed under its unit `.lock`; `S5_KEEP=1` keeps it.
# Usage: <crate> [--force] [extra,features]
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
G="$T/🗑️generated/s5-agnostic"
crate="$1"; shift
force=0; extra=""
for arg in "$@"; do
  if [ "$arg" = "--force" ]; then force=1; else extra="$arg"; fi
done
[ -n "$crate" ] || { echo "usage: <crate> [--force] [extra,features]"; exit 2; }
mkdir -p "$G/run"
log="$G/$crate.txt"; pidf="$G/run/$crate.pid"; exitf="$G/run/$crate.exit"
alive() { [ -f "$pidf" ] && kill -0 "$(cat "$pidf")" 2>/dev/null }
report() { awk -F'\t' -v crate="$crate" '$2 == crate' "$G/acceptance-results.tsv" 2>/dev/null | tail -1 | cut -c1-1800 }
if alive; then
  waited=0
  while alive && [ "$waited" -lt 540 ]; do sleep 10; waited=$((waited + 10)); done
  if alive; then echo "STILL RUNNING $crate: $(tail -1 "$log" 2>/dev/null | cut -c1-240)"; exit 3; fi
  echo "finished $(cat "$exitf" 2>/dev/null)"; report; exit 0
fi
if [ -f "$exitf" ] && [ "$force" = 0 ]; then echo "already ran $(cat "$exitf")"; report; exit 0; fi
echo $$ > "$pidf"; rm -f "$exitf"
manifest=$(/usr/bin/grep -l "^name = \"$crate\"" ✏️s/🔌️plugins/*/🗿️artifacts/*/📦️packages/🦀️rust/Cargo.toml | head -1)
[ -n "$manifest" ] || { echo "no manifest names $crate"; rm -f "$pidf"; exit 2; }
features=()
/usr/bin/grep -q "^component-app-assembly" "$manifest" && features+=("component-app-assembly")
[ -n "$extra" ] && features+=(${(s:,:)extra})
feature_args=()
[ ${#features[@]} -gt 0 ] && feature_args=(--features "${(j:,:)features}")
holder() { cat "$T/🗑️generated/coord/locks/landing/owner" 2>/dev/null }
foundation_red() {
  local line="$(cat "$T/🗑️generated/coord/foundation.status" 2>/dev/null)"
  [[ "$line" == BUILDING* ]] && return 0
  [[ "$line" == RED* ]] || return 1
  local since="$(date -j -f '%T' "${${=line}[2]}" +%s 2>/dev/null)"
  [ -n "$since" ] || return 1
  local age=$(( $(date +%s) - since ))
  [ "$age" -ge 0 ] && [ "$age" -lt 360 ]
}
quiet() {
  [ "$(pgrep -x cargo | wc -l | tr -d ' ')" -lt 4 ] \
    && [ "$(holder)" != "COORDINATOR-ACTIVATION" ] && [ ! -e "$G/run/landing-hold" ] && ! foundation_red && ! pgrep -f "history_edit_inputs_resolve documents_reload_identically" > /dev/null
}
gate() {
  while true; do
    until quiet; do sleep 20; done
    [ -z "$(holder)" ] && return
    sleep 20
    quiet && return
  done
}
echo "queued $(date '+%F %T') features=${features[*]}" > "$log"
gate
free=$(df -g /System/Volumes/Data | awk 'NR==2{print $4}')
if [ "$free" -lt 25 ]; then echo "DISK: $free GiB free < 25 GiB — nothing started (OWED, rule 55)" | tee -a "$log"; rm -f "$pidf"; exit 4; fi
echo "start $(date '+%F %T') free=${free}GiB" >> "$log"
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s5-agnostic
cargo test --manifest-path ✏️s/Cargo.toml -p "$crate" "${feature_args[@]}" --lib --no-fail-fast --message-format=short -- history_edits_end_to_end history_edit_inputs_resolve documents_reload_identically semio_payload_law --nocapture --test-threads=1 >> "$log" 2>&1
code=$?
echo "exit=$code $(date '+%F %T')" >> "$log"
if [ "$code" != 0 ] && [ -e "$G/run/landing-hold" ]; then echo "INTERRUPTED $crate for my own landing (no row; run it again)" | tee -a "$log"; rm -f "$pidf"; exit 7; fi
python3 - "$log" "$G/acceptance-results.tsv" "$crate" "$code" "${S5_KEEP:-0}" <<'PY'
import fcntl, os, re, shutil, sys, time
log, tsv, crate, code, keep = sys.argv[1:6]
text = open(log, encoding="utf-8", errors="replace").read()
lines = text.splitlines()
executable = None
for match in re.finditer(r"^\s+Running unittests .*?\((.*?/out/" + re.escape(crate.replace("-", "_")) + r"-[0-9a-f]+)\)\s*$", text, flags=re.M):
    executable = match.group(1)
names = re.findall(r"^test (\S+) \.\.\. ", text, flags=re.M)
failed = set()
block = re.search(r"^failures:\n((?:    \S+\n)+)\ntest result", text, flags=re.M)
if block:
    failed = {name.strip() for name in block.group(1).splitlines()}
result = re.search(r"^test result: (\w+)\. (\d+) passed; (\d+) failed", text, flags=re.M)
def law(segment):
    hits = [name for name in names if name.rsplit("::", 1)[-1] == segment]
    if not hits:
        return "-"
    if not result:
        return "ABORT"
    return "FAIL" if any(name in failed for name in hits) else "ok"
g12, inputs, reload, child = law("history_edits_end_to_end"), law("history_edit_inputs_resolve"), law("documents_reload_identically"), law("child_history_edits_end_to_end")
payload = [name for name in names if "semio_payload_law" in name]
payload_failed = sum(1 for name in payload if name in failed)
panics = []
for index, line in enumerate(lines):
    if "panicked at" in line:
        body = []
        for follow in lines[index + 1:index + 8]:
            if not follow.strip() or follow.startswith("note:") or follow.startswith("stack backtrace"):
                break
            body.append(follow.strip())
        panics.append((line.split("panicked at")[-1].strip()[:160] + " " + " ".join(body))[:700])
summaries = [line[line.index("["):] for line in lines if "[history-edit-" in line or "[documents-reload]" in line or "[child-history-edit]" in line]
errors = [line for line in lines if re.match(r"^\S.*:\d+:\d+: error", line) or line.startswith("error")]
if not names and not result:
    red = re.findall(r"could not compile `([^`]+)`", text)
    kind = "COMPILE-RED" if red or errors else "NOT-RUN"
    summary = (("could not compile " + ", ".join(dict.fromkeys(red)) + ": ") if red else "") + " | ".join(errors[:3])
elif not result:
    kind, summary = "ABORT", "the test process died without a result after " + (names[-1] if names else "no test") + " | " + " | ".join(lines[-4:])
else:
    wired = [status for status in (g12, inputs, reload, child) if status != "-"]
    kind = "NO-LAW" if not wired else ("PASS" if all(status == "ok" for status in wired) else "FAIL")
    summary = " | ".join(summaries + panics)
row = [time.strftime("%F %T"), crate, code, kind, g12, inputs, reload, child, f"{len(payload) - payload_failed}/{payload_failed}", summary.replace("\t", " ")[:4000]]
with open(tsv, "a", encoding="utf-8") as out:
    out.write("\t".join(row) + "\n")
print("\t".join(row)[:1800])
if executable and kind == "PASS" and keep != "1" and os.path.isfile(executable):
    unit = os.path.dirname(os.path.dirname(executable))
    try:
        handle = os.open(os.path.join(unit, ".lock"), os.O_RDWR | os.O_CREAT)
        fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        print("executable kept (unit locked):", executable)
    else:
        size = os.path.getsize(executable)
        os.remove(executable)
        shutil.rmtree(executable + ".dSYM", ignore_errors=True)
        os.close(handle)
        print(f"removed test executable {size / 2**20:.0f} MiB: {executable}")
elif executable:
    print("executable kept:", executable)
PY
echo "exit=$code $(date '+%F %T')" > "$exitf"
rm -f "$pidf"
