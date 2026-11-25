#!/bin/zsh
# 🔬️ Trial runs of the takeover of batch MV-1 on copies (nothing in the catalog is written). The queue of the machine is long, so the steps of one call run one after the other inside ONE reservation of the memory queue (1200 MB): exp.sh oak (the 1:200 gap repair `fix-english-oak.ts` on a copy, then its print suitability and its gate) · exp.sh locust <tag> [carry flags…] (the plates of `carry.ts` with other flags on a copy of `black-locust`, then the silhouette guard `measure.ts` and the gate). Several calls in one reservation: exp.sh all. Results: `$GEN/takeover/<name>.log`.
self="${0:A}"; export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
o="$GEN/takeover"; mkdir -p "$o" "$GEN/try"
if [[ -z "$INSIDE" ]]; then cd "$R" && INSIDE=1 exec repo capacity run --memory=1200 -- "$self" "$@"; fi
run() { local dir="$1" t="$2"; shift 2; (cd "$dir" && /usr/bin/time -l -a -o "$o/mem-exp.txt" timeout "$t" "$@"); }
case "$1" in
  oak)
    run "$T" 900 "$B" "$W/fix-english-oak.ts" --out "$GEN/try/english-oak-f.json" 2>&1 | tail -2
    run "$T" 2400 "$B" "$W/suit.ts" "$GEN/try/english-oak-f.json" > "$o/oak-suit.log" 2>&1; echo "suit exit $?"; tail -8 "$o/oak-suit.log"
    run "$T" 1800 "$B" "$M/gate.ts" "$GEN/try/english-oak-f.json" > "$o/oak-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$o/oak-gate.log")";;
  locust)
    tag="$2"; shift 2; f="$GEN/try/black-locust-$tag.json"
    run "$T" 3000 "$B" "$W/carry.ts" black-locust --out "$f" "$@" > "$o/locust-$tag-carry.log" 2>&1; echo "carry exit $? $(tail -1 "$o/locust-$tag-carry.log" | cut -c1-200)"
    run "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$f" > "$o/locust-$tag-measure.log" 2>&1; cut -c1-300 "$o/locust-$tag-measure.log"
    run "$T" 1800 "$B" "$M/gate.ts" "$f" > "$o/locust-$tag-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$o/locust-$tag-gate.log")";;
  all)
    echo "== oak"; "$self" oak; echo "== locust none"; "$self" locust none --limit 99; echo "== locust l1"; "$self" locust l1 --limit 1.0;;
esac
