#!/bin/zsh
# 🧪️ L1: dry-run every window-3 set on the live tree (no writes to the tree), one capture per set under wp-l1/generated/dry-<tag>/.
# usage: zsh l1-dry.sh <tag> [<set>…]   (no set = all)
setopt no_bg_nice
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket
OUT="$T/wp-l1/generated/dry-$1"; shift; mkdir -p "$OUT"
typeset -A DRY
DRY=(
  lb2-p6 "python3 $T/wp-lb2/lb2-p6-declared-catalogs.py --dry-run"
  lb2-p1 "python3 $T/wp-lb2/lb2-p1-arena-budget.py --dry-run"
  lb2-p7 "python3 $T/wp-lb2/lb2-p7-table-arena.py --dry-run"
  t14-all "python3 $T/wp-t14/land-w3.py --check"
  p9 "cd $T/wp-p9/patches && python3 p9-agent-lane.py --dry-run"
  s20-initializer "python3 $T/wp-s20/s20-patch-initializer.py --dry-run"
  s20-poll-yield "python3 $T/wp-s20/s20-patch-archive-poll-yield.py --dry-run"
  s20-document-verbs "python3 $T/wp-s20/s20-patch-document-verbs.py --dry-run"
  s20-chunk-staging "python3 $T/wp-s20/s20-patch-chunk-staging.py --dry-run"
  s20-retire-load-request "python3 $T/wp-s20/s20-patch-retire-load-request.py --dry-run"
  st2-code "python3 $T/wp-st2/st2-apply.py --dry-run --part code"
  st2-r10 "python3 $T/wp-st2/st2-apply.py --dry-run --part r10"
  cx1 "python3 $T/wp-st2/cx1-apply.py --dry-run"
  nx1b-rebuild "python3 $T/wp-st2/nx1b-apply.py --dry-run --part rebuild"
  lb2-p5 "python3 $T/wp-lb2/lb2-p5-docx-xlsx-opc-reds.py --dry-run"
  lb2-p3 "python3 $T/wp-lb2/lb2-p3-row-actions.py --dry-run"
  wg11-painter "python3 $T/wp-wg11/wg11-table-row-painter-patch.py"
  sh2-space-home "python3 $T/wp-sh2/sh2-apply.py"
  sh2-b1-compose "python3 $T/wp-sh2/b1/b1-compose.py"
  s18-twin "python3 $T/wp-s18/s18-named-layout-rust-twin.py --dry-run"
  wg11-reseed "python3 $T/wp-wg10/patch-rebootstrap-reseed.py"
  wg11-board "python3 $T/wp-wg11/wg11-board-presence-pointer-patch.py"
  wg11-a11y "python3 $T/wp-wg11/wg11-text-accessible-name-patch.py"
  wg11-shell-turn "python3 $T/wp-wg11/wg11-shell-turn-patch.py"
  wg11-harness "python3 $T/wp-wg11/wg11-harness-land.py"
  c12-splice "python3 $T/wp-c12/splice/patch/c12-splice-patch.py"
  c13-p1 "python3 $T/wp-c13/p1-foreign-transition-refused.py --dry-run"
  av2 "python3 $T/wp-av2/av2-apply.py"
  en2 "zsh $T/wp-en2/en2-land.sh dry"
  s19-gen-archive-load "python3 $T/wp-s19/s19-stage.py apply gen-archive-load"
  s19-norm-examples "python3 $T/wp-s19/s19-stage.py apply norm-examples"
  s19-norm-args "python3 $T/wp-s19/s19-stage.py apply norm-args"
  s19-norm-cleanup "python3 $T/wp-s19/s19-stage.py apply norm-cleanup"
  s19-flow-extensions "python3 $T/wp-s19/s19-stage.py apply flow-extensions"
  s20-cad-solids "python3 $T/wp-s20/s20-patch-cad-solids.py --dry-run"
  s20-process-formats "python3 $T/wp-s20/s20-patch-process-formats.py --dry-run"
  s20-silent-exports "python3 $T/wp-s20/s20-patch-silent-exports.py --dry-run"
  h13-refill "python3 $T/wp-h13/h13-transport-refill-patch.py --dry-run"
  h14-codec-origin "python3 $T/wp-h14/h14-codec-origin.py"
  g12-live-catalog "python3 $T/wp-g12/g12-live-catalog.py --dry-run"
  z4-b123 "python3 $T/wp-z4/b123-fresh-clone.py $R"
  z4-lifecycle "python3 $T/wp-z4/devcontainer-lifecycle/apply.py $R"
  r10-png "python3 $T/wp-r10/owned-png-host.py"
  r10-zip "python3 $T/wp-r10/owned-zip.py"
  r10-image "python3 $T/wp-r10/surface-image-error.py"
  r10-three "python3 $T/wp-r10/three-manifests.py"
)
ORDER=(lb2-p6 lb2-p1 lb2-p7 t14-all p9 s20-initializer s20-poll-yield s20-document-verbs s20-chunk-staging s20-retire-load-request st2-code st2-r10 cx1 nx1b-rebuild lb2-p5 lb2-p3 wg11-painter sh2-space-home sh2-b1-compose s18-twin wg11-reseed wg11-board wg11-a11y wg11-shell-turn wg11-harness c12-splice c13-p1 av2 en2 s19-gen-archive-load s19-norm-examples s19-norm-args s19-norm-cleanup s19-flow-extensions s20-cad-solids s20-process-formats s20-silent-exports h13-refill h14-codec-origin g12-live-catalog z4-b123 z4-lifecycle r10-png r10-zip r10-image r10-three)
[ $# -gt 0 ] && ORDER=("$@")
cd $R || exit 2
for s in $ORDER; do
  t0=$(date +%s)
  ( cd $R && eval "${DRY[$s]}" ) > "$OUT/$s.txt" 2>&1
  rc=$?
  printf '%-26s rc=%-3s %4ss | %s\n' "$s" "$rc" "$(( $(date +%s) - t0 ))" "$(tail -2 "$OUT/$s.txt" | tr '\n' ' ' | cut -c1-230)"
done
