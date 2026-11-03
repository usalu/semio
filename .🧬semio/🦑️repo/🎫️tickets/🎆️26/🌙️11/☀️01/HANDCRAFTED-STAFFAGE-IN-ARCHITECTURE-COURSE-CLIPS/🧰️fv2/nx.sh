#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fv2"
mkdir -p "$G/logs"
until grep -q DONE "$G/counts.tsv"; do sleep 20; done
: > "$G/nx.tsv"
cd "$W"

run_one() {
  local label="$1" lang="$2" id="$3" log start rc dur ran res
  log="$G/logs/nx-$label.$lang.log"
  start=$(date +%s)
  SEMIO_TEACHING_LANGUAGE="$lang" timeout 1500 bash "$T/m9_tests.sh" "$id" > "$log" 2>&1 < /dev/null
  rc=$?
  dur=$(( $(date +%s) - start ))
  ran=$(grep -E "^Ran [0-9]+ test" "$log" | tail -1 | awk '{print $2}')
  res=$(grep -E "^(OK|FAILED)" "$log" | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%ss\n' "$label" "$lang" "$rc" "${ran:-none}" "${res:-no-result}" "$id" "$dur" >> "$G/nx.tsv"
}

lang="$1"
cd "$W/🎓️teaching"
for f in $(find 🏛️architecture/🏢️nakagin/🧪️tests 🏛️architecture/📊️excel/🎬️clip/🧪️tests 🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests -name "🐍️*.py" | sort); do
  mod="🎓️teaching.$(echo "${f%.py}" | tr '/' '.')"
  for c in $(grep -oE "^class [A-Za-z0-9_]+" "$f" | awk '{print $2}'); do
    if [ "$c" = SceneContract ] && echo "$f" | grep -q "excel"; then
      for m in $(awk '/^class SceneContract/{on=1;next} /^class /{on=0} on && /^    def test_/{sub(/\(.*/,"",$2);print $2}' "$f"); do
        [ "$m" = test_native_lifecycle ] && continue
        run_one "$(echo "$mod" | md5sum | cut -c1-6)-$c-$m" "$lang" "$mod.$c.$m"
      done
    else
      run_one "$(echo "$mod" | md5sum | cut -c1-6)-$c" "$lang" "$mod.$c"
    fi
  done
done
echo DONE >> "$G/nx.tsv"
