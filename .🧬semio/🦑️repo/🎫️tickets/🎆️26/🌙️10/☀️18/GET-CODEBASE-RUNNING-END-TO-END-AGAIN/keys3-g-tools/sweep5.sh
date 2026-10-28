#!/bin/bash
# keys3-g: course:test again with every package.json/tsconfig.json granted (C1 is fixed by undeclared-g's batch) so the run reaches the Cargo.toml reads; then the unified log lines of those denials with the process names.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; U="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"; G3="$C/🗑️generated/keys3-g"
nice -n 10 bun "$U/target-run.mjs" k3d-course "$G3/closure-k3-course.jsonl" "@semio-tech/course:test" "$G/lists-c/_semio-tech_course__test.files" --rounds 40 --timeout 900 --extra "$G3/c1-all.files" > "$G3/sweep-k3d-course.out" 2> "$G3/sweep-k3d-course.err"
/usr/bin/log show --last 20m --style compact --predicate 'eventMessage CONTAINS "deny(1) file-read-data" AND eventMessage CONTAINS "Cargo.toml"' 2>/dev/null > "$G3/course-test-cargo-denials.log"
/usr/bin/grep -o "Sandbox: [^ ]*" "$G3/course-test-cargo-denials.log" | sort | uniq -c > "$G3/course-test-cargo-readers.txt"
echo "$(date +%H:%M:%S) course readers captured" >> "$G3/sweep.log"
