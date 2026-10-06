#!/bin/zsh
# 🧾️ S5-RUNTIME owed runs — one step per call, in the foreground, only after the coordinator's "LOCKS OPEN":
#   zsh 🧪️s5-runtime-owed.sh <g|transient|w2a|fwt|kernel|actor|s22|k3-wasip2 <1|2|3>>
# Every cargo call passes gate v6 (rule 66: `🚦️gate.sh`, fewer than 3 cargos on the shared build dir; a test build also needs
# ≥ 25 GiB free; exit 5 after 15 min closed = no cargo) with CARGO_BUILD_JOBS=3 on the shared build and target dirs (never a
# private target dir without its build dir), and writes its whole output under 🗑️generated/s5-runtime/owed-<step>.txt, printing
# the verdict lines. A cargo that ends without a compile error and without a verdict is NO VERDICT: re-issue it once.
# `k3-wasip2 <1|2|3>` checks four of the twelve K3 crates per call (a wasm32 build: it needs the coordinator's word on the
# wasm build mutex first).
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
OUT="$T/🗑️generated/s5-runtime"
mkdir -p "$OUT"

verdict() {
  /usr/bin/grep -n "FAILED\|test result\|^error\|exit=" "$1" | cut -c1-240
}

tests() {
  local name="$1"
  shift
  zsh "$T/🚦️gate.sh" 3 25 || exit 5
  date > "$OUT/$name.txt"
  CARGO_BUILD_JOBS=3 CARGO_INCREMENTAL=0 cargo test "$@" >> "$OUT/$name.txt" 2>&1
  echo "exit=$?" >> "$OUT/$name.txt"
  date >> "$OUT/$name.txt"
  verdict "$OUT/$name.txt"
}

case "$1" in
  g)
    python3 "$T/🧪️s5-runtime-land.py" check g && python3 "$T/🧪️s5-runtime-land.py" land g
    ;;
  transient)
    tests owed-transient-root -p semio-framework-plugin --lib --features artifact-app-testing -- transient_root
    ;;
  w2a)
    tests owed-w2a -p semio-framework-plugin --lib --features artifact-app-testing -- time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body composed_child_history transient_root
    ;;
  fwt)
    tests owed-fwt -p semio-framework-time-travel
    ;;
  kernel)
    tests owed-kernel -p semio-framework --lib -- history_patch history_notices history_edit framework_notices
    ;;
  actor)
    tests owed-actor -p semio-framework-plugin --lib --features artifact-app-testing -- an_instance_always_acts_as_someone
    ;;
  s22)
    tests owed-s22 -p semio-framework-plugin --lib --features artifact-app-testing -- an_inverse_refusal_is_one_mutations_fatal hostile_history_edit_input a_blocking_mutation_without_editable_inputs
    ;;
  k3-wasip2)
    case "$2" in
      1) crates=(-p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d); features="semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly" ;;
      2) crates=(-p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-remodel-remodeling -p semio-s-artifact-raster-raster -p semio-s-artifact-wfc-grid2d); features="semio-s-artifact-wfc-grid2d/component-app-assembly" ;;
      3) crates=(-p semio-s-artifact-wfc-bitmap -p semio-s-artifact-layout-layout -p semio-s-artifact-draw-drawing -p semio-s-artifact-forms-forms); features="semio-s-artifact-wfc-bitmap/component-app-assembly" ;;
      *) echo "usage: zsh $0 k3-wasip2 <1|2|3>"; exit 2 ;;
    esac
    zsh "$T/🚦️gate.sh" || exit 5
    date > "$OUT/owed-k3-wasip2-$2.txt"
    CARGO_BUILD_JOBS=3 cargo check --keep-going --manifest-path "✏️s/Cargo.toml" --target wasm32-wasip2 --lib "${crates[@]}" --features "$features" >> "$OUT/owed-k3-wasip2-$2.txt" 2>&1
    echo "exit=$?" >> "$OUT/owed-k3-wasip2-$2.txt"
    date >> "$OUT/owed-k3-wasip2-$2.txt"
    /usr/bin/grep -n "^error\|exit=" "$OUT/owed-k3-wasip2-$2.txt" | cut -c1-240 | tail -20
    ;;
  *)
    echo "usage: zsh $0 <g|transient|w2a|fwt|kernel|actor|s22|k3-wasip2 <1|2|3>>"
    exit 2
    ;;
esac
