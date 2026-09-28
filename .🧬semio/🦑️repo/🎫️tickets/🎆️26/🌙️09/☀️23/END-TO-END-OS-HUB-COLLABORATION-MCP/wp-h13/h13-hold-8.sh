#!/bin/zsh
# 🎚️ H13 hold 8 (session 14c): (1) the retirement-wake law against a mutant without the wake (must FAIL), restored at once and
# re-run (must PASS); (2) semio-hub all-feature check after every kernel-db edit; (3) the full `os-hub:test-all-features` suite
# on the SQLite reader connections.
# usage: h13-hold-8.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
A="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"
WAKE='        self.terminal.store(true, std::sync::atomic::Ordering::Release);
        self.request_retirement_maintenance();
    }'
MUTANT='        self.terminal.store(true, std::sync::atomic::Ordering::Release);
    }'
python3 - "$A" "$WAKE" "$MUTANT" <<'PY'
import sys; p, old, new = sys.argv[1:]; s = open(p, encoding="utf-8").read(); assert s.count(old) == 1; open(p, "w", encoding="utf-8").write(s.replace(old, new, 1))
PY
zsh $W/h13-cargo.sh "$1-retirement-law-mutant" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast -- a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement
python3 - "$A" "$MUTANT" "$WAKE" <<'PY'
import sys; p, old, new = sys.argv[1:]; s = open(p, encoding="utf-8").read(); assert s.count(old) == 1; open(p, "w", encoding="utf-8").write(s.replace(old, new, 1))
PY
python3 $W/h13-retirement-wake-patch.py --dry-run | tail -1
zsh $W/h13-cargo.sh "$1-retirement-law" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast -- a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement file_reads_run_on_wal_readers
zsh $W/h13-cargo.sh "$1-check-hub" check -p semio-hub --all-features --lib --bins --tests
zsh $W/h13-cargo.sh "$1-hub-all-features" test -p semio-hub --all-features --no-fail-fast
