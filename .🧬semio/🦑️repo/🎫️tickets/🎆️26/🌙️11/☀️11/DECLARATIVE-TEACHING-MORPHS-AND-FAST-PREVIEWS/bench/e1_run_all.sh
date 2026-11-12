#!/usr/bin/env bash
# Usage: e1_run_all.sh <parallel jobs>; runs every (profile, module) line of e1_scenes.txt, logs to $T/🗑️generated/e1/logs
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS"
LOGS="$T/🗑️generated/e1/logs"
mkdir -p "$LOGS"
JOBS="${1:-3}"
tail -n +3 "$T/bench/e1_scenes.txt" | grep -v '^$' | while read -r module scenes; do
  for profile in preview final; do
    echo "$profile|$module|$scenes"
  done
done | xargs -P "$JOBS" -I{} bash -c '
  IFS="|" read -r profile module scenes <<< "{}"
  name="${profile}-${module//[^[:alnum:]]/_}"
  name="$(echo "$profile-$module" | md5sum | cut -c1-8)-$profile"
  echo "$profile $module $scenes" > "'"$LOGS"'/$name.txt"
  E1_ROOT="/c/git/semio/'"$T"'/🗑️generated/e1/root-$name" bash "'"$T"'/bench/e1_smoke.sh" "$profile" "$module" $scenes >> "'"$LOGS"'/$name.txt" 2>&1
  echo "exit $?" >> "'"$LOGS"'/$name.txt"
'
