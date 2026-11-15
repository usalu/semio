#!/bin/bash
TK="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
rm -f "$TK/🗑️generated/heat-a-out/MANYDONE"
cd "$TK/🗑️generated/heat-a-out/m"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_PROFILE=preview
for sc in SpecProbe LawProbe; do
 uv run --project "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim -ql -s --media_dir "$TK/🗑️generated/heat-a-out/m" "$TK/heat-a/probe.py" $sc > "$TK/🗑️generated/heat-a-out/one-$sc.log" 2>&1
done
for pair in "🧲️fundamentals Beat8_UWert" "🧲️fundamentals Beat5_Zusammenfassung" "🧱️conduction Beat6_Waermebruecken" "🧱️conduction Beat5_Gebaeudehuelle" "🧲️fundamentals Beat7_Waermedurchlasswiderstand"; do
  bash "$TK/heat-a/one.sh" $pair
done
echo x > "$TK/🗑️generated/heat-a-out/MANYDONE"
