#!/bin/zsh
# 🔬️ One laboratory round of `vol.ts` under ONE grant of the memory queue (the processes run one after the other inside it): lab.sh <round-name> <id>… For every asset: the backup fitted with `ops/<id>.json` (`fit2.ts`, four passes; OPS=<folder> takes another ops folder) as $GEN/lab/<round>/<id>.base.json and its report, then the plan `plans/<id>.try.json` tried on it; the result goes to $GEN/lab/<round>/<id>.json, the chosen makes to $GEN/lab/<round>/<id>.decided.json, the log to $GEN/lab/<round>/<id>.log. PLAN=<suffix> takes `plans/<id>.<suffix>.json`, REPORT=0 skips the report.
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
round="$1"; shift; d="$GEN/lab/$round"; mkdir -p "$d"
inner="$d/run.sh"; : > "$inner"
for a in "$@"; do
  src="$d/$a.base.json"; cp "$GEN/backup/$a.json" "$src"
  for i in 1 2 3 4; do print -r -- "\"$B\" \"$W/fit2.ts\" \"$src\" --ops \"$W/${OPS:-ops}/$a.json\" --out \"$src\" > \"$d/$a.fit$i.log\" 2>&1" >> "$inner"; done
  [[ "${REPORT:-1}" == 1 ]] && print -r -- "\"$B\" \"$W/vol.ts\" \"$src\" --report > \"$d/$a.report.log\" 2>&1" >> "$inner"
  print -r -- "\"$B\" \"$W/vol.ts\" \"$src\" --plan \"$W/plans/$a.${PLAN:-try}.json\" --out \"$d/$a.json\" --decided \"$d/$a.decided.json\" --budget ${BUDGET:-2400} > \"$d/$a.log\" 2>&1" >> "$inner"
done
"$W/q.sh" ${MB:-2500} ${TIMEOUT:-9000} "$T" /bin/zsh "$inner"
echo "LABDONE $round $(date +%H:%M)" >> "$d/done.txt"
