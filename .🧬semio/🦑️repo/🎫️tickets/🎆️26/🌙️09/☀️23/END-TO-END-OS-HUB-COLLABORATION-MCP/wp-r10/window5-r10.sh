#!/bin/zsh
# 🪟️ R10 WINDOW 5 step (T0-style, strictly serial, one caller at a time; preamble 15 + session-14 rules 20/25/28).
#   pre-r4   BEFORE L1's round 4: taxonomy (+290 names: the pre-existing config/ShellHost/host-tests/space-home/caching set incl.
#            🐳️containers/🔁️lifecycle, the stdio contract ✏️editing chain, 26 dirs new since the T4 pass, the round-4 sets' 19 planned
#            dirs, H14's 🌳️most-general-dialect) → targets (AV2 video-render-export ×2 in the root 📋️project.json) → render
#            (launch.json from the registry generator) → plan (AV2 check released) → verify (registry check + launch laws + boot).
#   post-r4  AFTER L1 reports T6R4 GREEN: re-probe the round-4 scopes for any directory the sets created that was not planned
#            (prints them; a non-empty list = one more `taxonomy` apply), schema-catalog regen + `schema verify` + `schema check`.
#   dry      every step's dry run on the live tree (nothing written).
# Revert: `bun wp-r10/window3-apply.ts revert <taxonomy|targets|render|plan> --apply`; schema catalog: its backup dir.
# Logs: .🧬semio/🌐hub/s14-r10-logs/window5-<phase>-<time>.txt
setopt no_bg_nice pipe_fail
here=/Users/ueli/Documents/semio/.tmp-ticket/wp-r10
root=/Users/ueli/Documents/semio
logs="$root/.🧬semio/🌐hub/s14-r10-logs"
state="$root/.🧬semio/🌐hub/s14-r10-state"
log="$logs/window5-$1-$(date '+%H%M%S').txt"
r4_scopes=('✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space' '✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract' '✏️s/🔌️plugins/🗄️stdio/🧪️tests' '✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing' '✏️s/🔌️plugins/🏭️process/🧪️tests' '✏️s/🔌️plugins/📋️forms/🧪️tests' '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store' '🧰️framework/🛍️products/💻️os/🧫️fixtures' '🧰️framework/🔨️modules/🚪️io' '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack')
release_av2() {
  python3 -c "import json,sys; p=sys.argv[1]; s=json.load(open(p,encoding='utf-8')); [c.pop('hold',None) for c in s['planChecks'] if c['check']['id']=='video-render-export']; open(p,'w',encoding='utf-8').write(json.dumps(s,ensure_ascii=False,indent=1)+'\n')" "$here/window3-spec.json"
}
{
  echo "[window5-r10] $1 start $(date '+%F %T') load $(sysctl -n vm.loadavg)"
  case "$1" in
    dry)
      for step in taxonomy targets render plan; do (cd "$here" && bun window3-apply.ts "$step") || { echo "[window5-r10] DRY $step FAILED"; exit 1; }; done
      (cd "$here" && python3 trace-slots-default-off.py --dry-run)
      (cd "$here" && bun schema-catalog-regen.ts | /usr/bin/grep -v '^  "')
      ;;
    pre-r4)
      zsh "$here/window3-run.sh" taxonomy --probe-only || exit $?
      zsh "$here/window3-run.sh" targets --probe-only || exit $?
      zsh "$here/window3-run.sh" render --probe-only || exit $?
      release_av2
      zsh "$here/window3-run.sh" plan || exit $?
      (cd "$here" && bun plan-targets-check.ts && bun harness-registry-audit.ts)
      zsh "$here/window3-run.sh" verify || exit $?
      ;;
    post-r4)
      (cd "$here" && bun taxonomy-kinds.ts --json "$state/probe-s15-post-r4.json" "${r4_scopes[@]}" | /usr/bin/grep '^\[taxonomy-kinds\]')
      python3 -c "import json,sys; live={r['path'] for r in json.load(open(sys.argv[1]))}; rows=[r for r in json.load(open(sys.argv[2])) if r['path'] not in live]; print('[window5-r10] unresolved directories the round created:', len(rows)); [print('  ', r['path'], r['parentKind']) for r in rows]; json.dump(rows, open(sys.argv[3],'w'), ensure_ascii=False, indent=1)" "$state/probe-s15-all.json" "$state/probe-s15-post-r4.json" "$state/probe-s15-post-r4-new.json"
      (cd "$here" && bun schema-catalog-regen.ts --apply | /usr/bin/grep -v '^  "') || { echo "[window5-r10] SCHEMA REGEN FAILED"; exit 1; }
      (cd "$root" && bun ./📜️script.ts schema verify && bun ./📜️script.ts schema check | tail -3) || echo "[window5-r10] SCHEMA VERIFY/CHECK RED"
      ;;
    *) echo "usage: zsh window5-r10.sh <dry|pre-r4|post-r4>"; exit 2 ;;
  esac
  echo "[window5-r10] $1 DONE $(date '+%T')"
} 2>&1 | tee "$log"
