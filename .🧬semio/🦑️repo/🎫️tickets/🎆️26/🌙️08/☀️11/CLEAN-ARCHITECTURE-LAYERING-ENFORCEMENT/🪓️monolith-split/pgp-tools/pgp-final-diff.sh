#!/bin/zsh
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pgp
F=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split
D=$F/pgp-tools/pgp-diff.sh
echo "### A1 apply (all formats, whole ✏️s) from the live tree"; $D fa apply 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
python3 $F/pgp-tools/pgp-selfrec-compare.py $SC/fa/py $SC/fa/ts
echo "### R1 apply --closing (stdio) from the restored pre-state"; PGP_BASE=$SC/pre $D fr1 apply --closing --formats 🟦️typescript --under "✏️s/🔌️plugins/🗄️stdio" 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
echo "### R2 dxf / md replace routes"; PGP_BASE=$SC/fr1/ts $D fr2d apply --closing --replace Vec3 --formats 🟦️typescript --under "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf" 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
PGP_BASE=$SC/fr1/ts $D fr2m apply --closing --replace MdBlockAdded,MdListItemAdded --formats 🟦️typescript --under "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md" 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
echo "### R3 draw replace"; PGP_BASE=$SC/pre $D fr3 apply --replace DrawingTopology --formats 🟦️typescript --under "✏️s/🔌️plugins/🖍️draw" 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
echo "### M14 replace of the 14 untyped-map files"; $D fm apply --replace FlowTopology,PresentationTopology,ShootingTopology,SequenceTopology,FormsTopology,LayoutTopology,PlaybookTopology,ProcedureTopology,RemodelingInference,RasterTopology,SemioFlowTopology,SemioMeshInference,SemioGraphTopology,Puzzle2dFlatPosition --formats 🟦️typescript 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
echo "### H hub replace"; PGP_BASE=$SC/hubbase $D fh apply --replace SpaceConfig,SpacePresence --formats 🟦️typescript --under "🌎️hub" 2>&1 | /usr/bin/grep -E 'exit|identical|DIFFER|changed'
echo "### live-equality checks"
for rel in "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🟦️.ts"; do
  if cmp -s "$SC/fr1/ts/$rel" "$SC/base/$rel"; then echo "identical to the live generated file: ...${rel: -52}"; else echo "DIFFERS from live: ...${rel: -52}"; fi; done
cmp -s "$SC/fr2d/ts/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/🟦️.ts" "$SC/base/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/🟦️.ts" && echo "identical to the live generated file: dxf snapshot"
cmp -s "$SC/fr3/ts/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts" "$SC/base/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts" && echo "identical to the live generated file: draw inferences"
echo "### done"
