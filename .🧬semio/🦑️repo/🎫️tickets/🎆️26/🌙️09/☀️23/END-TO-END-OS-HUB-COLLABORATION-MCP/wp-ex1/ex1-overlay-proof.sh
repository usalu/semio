#!/bin/zsh
# 🧪️ EX1 overlay proof (session 15): the example-loader set applied AFTER T6 row 12 — the base is a fresh APFS clone of S20's
# rebased faults overlay (row 12 + everything live up to its rebase) — under ONE overlay-lane hold, private build-dir seeded
# with registry units only, disk guard 40 GiB (preamble rules 3/23/25/28). The set script, fixtures and laws are snapshotted at
# queue time into the capture dir, so later edits to the prepared set do not race the proof. ONE EX1 proof in the lane at a time.
#   S  sections applied (`--only`): default `sdk stdio laws plugins-a plugins-b plugins-c examples-stdio`; the crates the
#      sections touch are listed from the clone BEFORE the apply (`--list-crates`)
#   R  resolver + catalogue laws (SDK dummy)
#   C  stdio editor-catalog law (88 shipped editors: resolver + handler example laws, then the existing catalog contract)
#   L  per touched plugin crate: its example-catalog laws (one cargo per crate after one parallel build, so a compile error
#      never hides another plugin's result)
#   T  per touched crate (SDK and framework excluded — R covers them): its FULL `--lib` suite, same per-crate isolation
#   W  wasm32-wasip2 `--lib` check per touched crate
# usage: EX1_STEPS="R C L W" zsh ex1-overlay-proof.sh <tag> [sections…]   (default steps R C L W; capture
# `.🧬semio/🌐hub/s15-ex1-runs/<tag>/proof.txt`)
tag="$1"; shift
sections=("$@")
[ $# -eq 0 ] && sections=(sdk stdio laws plugins-a plugins-b plugins-c examples-stdio)
HUB="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
BASE="$HUB/s14-s20-overlay-faults"
OV="$HUB/s15-ex1-overlay"
RUN="$HUB/s15-ex1-runs/$tag"
OUT="$RUN/proof.txt"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-ex1
mkdir -p "$RUN"
cp "$W/ex1-example-loaders.py" "$RUN/ex1-example-loaders.py"
mkdir -p "$RUN/fixtures" "$RUN/laws" && cp "$W/fixtures/"*.json "$RUN/fixtures/" && cp "$W/laws/"*.json "$RUN/laws/"
echo "QUEUED $(date '+%F %T') sections=${sections[*]} steps=${EX1_STEPS:-R C L W} base=$BASE" > "$OUT"
export CARGO_BUILD_BUILD_DIR="$OV/.ex1-build" CARGO_TARGET_DIR="$OV/.ex1-target" CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
export EX1_OV="$OV" EX1_BASE="$BASE" EX1_RUN="$RUN" EX1_SECTIONS="${sections[*]}" EX1_STEPS="${EX1_STEPS:-R C L W}"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay ex1 -- zsh -c '
  free=$(df -g / | awk "NR==2 {print \$4}")
  echo "=== lane $(date "+%T") load=$(sysctl -n vm.loadavg) free=${free}GiB"
  [ "$free" -ge 40 ] || { echo "DISK GUARD: ${free} GiB free < 40, not building"; exit 3; }
  echo "=== clone base $(date "+%T")"
  python3 - "$EX1_BASE" "$EX1_OV" <<"PY" || exit 4
import ctypes, os, shutil, sys
base, overlay = sys.argv[1], sys.argv[2]
libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]
os.makedirs(overlay, exist_ok=True)
for entry in sorted(os.listdir(base)):
    if entry.startswith(".ex1-") or entry in (".t14-trash",):
        continue
    target = os.path.join(overlay, entry)
    if os.path.lexists(target):
        shutil.rmtree(target) if os.path.isdir(target) and not os.path.islink(target) else os.unlink(target)
    if libc.clonefile(os.path.join(base, entry).encode(), target.encode(), 1) != 0:
        raise OSError(ctypes.get_errno(), entry)
print("cloned", len(os.listdir(base)), "entries")
PY
  crates=($(python3 "$EX1_RUN/ex1-example-loaders.py" --list-crates --root "$EX1_OV" --only ${=EX1_SECTIONS}))
  echo "=== crates ${#crates} ${crates[*]}"
  echo "=== S apply ${EX1_SECTIONS} $(date "+%T")"
  python3 "$EX1_RUN/ex1-example-loaders.py" --write --root "$EX1_OV" --only ${=EX1_SECTIONS} || { echo "APPLY FAILED"; exit 5; }
  [ -d "$EX1_OV/.ex1-build/debug" ] || python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-t14/overlay-build-seed.py "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug" "$EX1_OV/.ex1-build/debug"
  cd "$EX1_OV" || exit 6
  steps=(${=EX1_STEPS})
  if (( ${steps[(Ie)R]} )); then
    echo "=== R resolver law $(date "+%T")"
    nice -n 15 cargo test -p semio-framework-plugin --lib --no-fail-fast -- app::mutation_fixture::dummy::the_example_resolver_loads_or_refuses_by_code app::mutation_fixture::dummy::set_active_example_loads_the_registered_catalogue_without_a_declared_action; echo "R rc=$? $(date "+%T")"
  fi
  if (( ${steps[(Ie)C]} )); then
    echo "=== C stdio editor-catalog law $(date "+%T")"
    nice -n 15 cargo test -p semio-s-plugin-stdio --test editor_catalog --no-fail-fast; echo "C rc=$? $(date "+%T")"
  fi
  if (( ${steps[(Ie)L]} )); then
    plugins=(${(M)crates:#semio-s-plugin-*})
    plugins=(${plugins:#semio-s-plugin-stdio})
    echo "=== L build ${#plugins} plugin crates $(date "+%T")"
    packages=(); for crate in $plugins; do packages+=(-p $crate); done
    nice -n 15 cargo test --no-run --lib $packages; echo "L build rc=$? $(date "+%T")"
    for crate in $plugins; do
      echo "=== L $crate $(date "+%T")"
      nice -n 15 cargo test --lib -p $crate --no-fail-fast -- loads_every_example_it_publishes the_example_catalog_lists_every_editor_once; echo "L $crate rc=$? $(date "+%T")"
    done
  fi
  if (( ${steps[(Ie)T]} )); then
    tested=(${crates:#semio-framework-plugin})
    tested=(${tested:#semio-framework})
    echo "=== T build ${#tested} crates $(date "+%T")"
    packages=(); for crate in $tested; do packages+=(-p $crate); done
    nice -n 15 cargo test --no-run --lib $packages; echo "T build rc=$? $(date "+%T")"
    for crate in $tested; do
      echo "=== T $crate $(date "+%T")"
      nice -n 15 cargo test --lib -p $crate --no-fail-fast; echo "T $crate rc=$? $(date "+%T")"
    done
  fi
  if (( ${steps[(Ie)W]} )); then
    for crate in $crates; do
      echo "=== W $crate $(date "+%T")"
      nice -n 15 cargo check --target wasm32-wasip2 --lib -p $crate; echo "W $crate rc=$? $(date "+%T")"
    done
  fi
' >> "$OUT" 2>&1
echo "EXIT rc=$? $(date '+%F %T')" >> "$OUT"
