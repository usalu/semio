#!/bin/zsh
# ⏪️ Runs the history-edit acceptance laws (`history_edits_end_to_end`, `history_edit_inputs_resolve`) and the derive-emitted G8 payload
# laws (`semio_payload_law_*`; S3-AGNOSTIC, design §16.3) for the named plugin crates in ONE gated foreground cargo invocation (fleet rule
# 30; one feature-unified build of the shared framework crates), then appends one line per crate to
# 🗑️generated/s3-agnostic/acceptance-results.tsv (batch, crate, exit, passed, failed, summary). Usage: <batch-name> <crate>…
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s3-agnostic"
mkdir -p "$G"
batch=$1
shift
packages=()
features=()
for crate in "$@"; do
  packages+=(-p "$crate")
  manifest=$(/usr/bin/grep -l "^name = \"$crate\"" ✏️s/🔌️plugins/*/🗿️artifacts/*/📦️packages/🦀️rust/Cargo.toml | head -1)
  /usr/bin/grep -q "^component-app-assembly" "$manifest" && features+=("$crate/component-app-assembly")
done
feature_args=()
[ ${#features[@]} -gt 0 ] && feature_args=(--features "${(j:,:)features}")
until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 14 ]; do sleep 30; done
echo "start $(date '+%T')" > "$G/acceptance-$batch.log"
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-s3-agnostic cargo test --manifest-path ✏️s/Cargo.toml "${packages[@]}" "${feature_args[@]}" --lib --no-fail-fast -j 4 --message-format=short -- history_edits_end_to_end history_edit_inputs_resolve semio_payload_law --nocapture --test-threads=1 >> "$G/acceptance-$batch.log" 2>&1
code=$?
echo "exit=$code $(date '+%T')" >> "$G/acceptance-$batch.log"
python3 - "$G/acceptance-$batch.log" "$G/acceptance-results.tsv" "$batch" "$code" "$@" <<'PY'
import re, sys
log, tsv, batch, code, crates = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5:]
text = open(log, encoding="utf-8", errors="replace").read()
sections = re.split(r"^\s+Running unittests .*?\(.*?/deps/([a-z0-9_]+)-[0-9a-f]+\)\s*$", text, flags=re.M)
runs = {sections[i]: sections[i + 1] for i in range(1, len(sections) - 1, 2)}
with open(tsv, "a", encoding="utf-8") as out:
    for crate in crates:
        body = runs.get(crate.replace("-", "_"))
        if body is None:
            errors = [line for line in text.splitlines() if crate.replace("semio-s-artifact-", "") in line and "error" in line][:3]
            out.write(f"{batch}\t{crate}\t{code}\t-\t-\tNOT RUN (no test binary): {' | '.join(errors)[:900]}\n")
            print(crate, "NOT RUN")
            continue
        result = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", body)
        passed, failed = (result.group(1), result.group(2)) if result else ("-", "-")
        lines = [line for line in body.splitlines() if line.startswith("[history-edit-") or line.startswith("test ") and "FAILED" in line]
        panics = [body.splitlines()[i + 1][:600] for i, line in enumerate(body.splitlines()) if "panicked at" in line and i + 1 < len(body.splitlines())]
        summary = " | ".join(lines + panics)[:1500]
        out.write(f"{batch}\t{crate}\t{code}\t{passed}\t{failed}\t{summary}\n")
        print(crate, passed, "passed", failed, "failed")
PY
echo "done exit=$code"
