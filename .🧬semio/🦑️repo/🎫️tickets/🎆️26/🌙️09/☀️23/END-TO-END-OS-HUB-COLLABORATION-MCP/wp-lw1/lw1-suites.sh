#!/bin/zsh
# 🧪️ LW1 → U6's row-6 proof form on the live tree (U6 `wp-u6/t4/r6-base.sh`, same 38-crate set so the build-dir units match U6's):
#   lw1-suites.sh build <list-file>          build every lib test binary without running (one cargo), write their paths to <list-file>
#   lw1-suites.sh suite <list-file> <crate>  run one binary as a whole suite (4 threads), re-run each red alone (`--exact`, 1 thread):
#                                            prints `SUITE <crate> reds=N`, then `RED <name>` / `ALONE-OK <name>` (U6 `suite.sh`)
#   lw1-suites.sh serial <list-file> <crate> [filter…]  run one binary serially (`--test-threads 1`, optional name filters) and
#                                            print its result line (full output beside the list: `<list-file>.<crate>.serial.txt`)
R=/Users/ueli/Documents/semio; T=$R/.tmp-ticket/wp-u6/t4
cmd=$1; list=$2; crate=$3; shift 3 2>/dev/null
bin_of() { /usr/bin/grep -E "/${1//-/_}-[0-9a-f]+\$" "$list" | head -1; }
case "$cmd" in
  build)
    args=(); for p in $(cat $T/crates.txt) semio-framework-plugin semio-s-artifact-stdio-bcf semio-s-artifact-stdio-xlsx semio-s-plugin-playbook-procedural; do args+=(-p $p); done
    cargo test --offline --no-run --message-format short --lib $args > "$list.raw" 2>&1; rc=$?
    /usr/bin/grep -E "^error" -A3 "$list.raw" | head -20
    /usr/bin/grep "Executable" "$list.raw" | sed 's/.*(\(.*\))$/\1/' | sort -u > "$list"
    echo "BINARIES $(/usr/bin/grep -c . "$list") rc=$rc"; exit $rc ;;
  suite)
    bin=$(bin_of "$crate"); [ -x "$bin" ] || { echo "SUITE $crate missing binary"; exit 2; }
    "$bin" --test-threads 4 > "$list.$crate.suite.txt" 2>&1; rc=$?
    reds=(${(f)"$(sed -n 's/^test \(.*\) \.\.\. FAILED$/\1/p' "$list.$crate.suite.txt")"})
    echo "SUITE $crate rc=$rc reds=${#reds} $(/usr/bin/grep -E '^test result|aborting|SIGABRT|signal:' "$list.$crate.suite.txt" | tr '\n' ' ')"
    for name in $reds; do
      if "$bin" --exact "$name" --test-threads 1 > /dev/null 2>&1; then echo "ALONE-OK $name"; else echo "RED $name"; fi
    done
    [ $rc -ne 0 ] && [ ${#reds} -eq 0 ] && exit $rc; exit $(( ${#reds} > 0 )) ;;
  serial)
    bin=$(bin_of "$crate"); [ -x "$bin" ] || { echo "SERIAL $crate missing binary"; exit 2; }
    "$bin" "$@" --test-threads 1 > "$list.$crate.serial.txt" 2>&1; rc=$?
    echo "SERIAL $crate rc=$rc $(/usr/bin/grep -E '^test result|aborting|FAILED$' "$list.$crate.serial.txt" | tr '\n' ' ')"; exit $rc ;;
esac
