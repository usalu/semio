#!/bin/bash
TK="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
cd "$TK/🗑️generated/heat-a-out/m"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_PROFILE=preview
for sc in SpecProbe LawProbe; do
 uv run --project "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim -ql -s --media_dir "$TK/🗑️generated/heat-a-out/m" "$TK/heat-a/probe.py" $sc > "$TK/🗑️generated/heat-a-out/one-$sc.log" 2>&1
done
bash "$TK/heat-a/one.sh" 🧱️conduction Beat1_MakroUndMikro
cd /c/git/semio
SEMIO_TEST_MANIM=1 SEMIO_TEACHING_OUTPUT_ROOT="$TK/🗑️generated/heat-a-out" uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧱️conduction.🧪️tests.🐍️" > "$TK/🗑️generated/heat-a-out/unittest.log" 2>&1
echo x > "$TK/🗑️generated/heat-a-out/FINALDONE"
