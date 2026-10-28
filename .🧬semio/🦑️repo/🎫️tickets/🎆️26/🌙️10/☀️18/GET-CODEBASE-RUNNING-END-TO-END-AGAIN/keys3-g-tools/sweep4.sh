#!/bin/bash
# keys3-g: after the other queues, ONE oracle run of @semio-tech/course:test, then the unified log lines of its Cargo.toml denials WITH the process names (who reads the 146 manifests).
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; U="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"; G3="$C/🗑️generated/keys3-g"
for _ in $(seq 1 900); do /usr/bin/grep -q "end k3-ac-build" "$G3/sweep.log" 2>/dev/null && break; sleep 20; done
echo "$(date +%H:%M:%S) start k3-course-test" >> "$G3/sweep.log"
nice -n 10 bun "$U/target-run.mjs" k3-course-test "$G3/closure-k3-course.jsonl" "@semio-tech/course:test" "$G/lists-c/_semio-tech_course__test.files" --rounds 2 --timeout 900 > "$G3/sweep-k3-course-test.out" 2> "$G3/sweep-k3-course-test.err"
echo "$(date +%H:%M:%S) end k3-course-test rc=$?" >> "$G3/sweep.log"
/usr/bin/log show --last 15m --style compact --predicate 'eventMessage CONTAINS "deny(1) file-read-data" AND eventMessage CONTAINS "Cargo.toml"' 2>/dev/null | /usr/bin/awk '{ for (i=1;i<=NF;i++) if ($i ~ /^Sandbox:/) { print $(i+1) } }' | /usr/bin/sed 's/([0-9]*)//' | sort | uniq -c > "$G3/course-test-cargo-readers.txt"
echo "$(date +%H:%M:%S) readers captured" >> "$G3/sweep.log"
