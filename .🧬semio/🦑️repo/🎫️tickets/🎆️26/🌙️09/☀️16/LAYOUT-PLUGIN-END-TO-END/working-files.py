import json, os, subprocess
from pathlib import Path
root=Path.cwd()
ticket=Path(__file__).resolve().parent.relative_to(root)
layout="✏️s/🔌️plugins/📏️layout"
store="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
scopes=[layout,store,plugin,"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation","Cargo.toml","Cargo.lock","🧰️framework/Cargo.toml","📋️build-owners.json"]
data=subprocess.run(["repo","--service=off","tools","exec","--light","git","--","status","--porcelain=v1","-z","--untracked-files=all","--",*scopes],cwd=root,check=True,stdout=subprocess.PIPE).stdout.decode()
statuses={}
rows=data.split("\0");index=0
while index<len(rows):
 row=rows[index];index+=1
 if not row:continue
 state,path=row[:2],row[3:];statuses[path]=state
 if "R" in state or "C" in state:index+=1
store_files=["⚙️engine/🌿️branch/🦀️.rs","⚙️engine/📥️lifted/🏪️store/🦀️.rs","⚙️engine/📥️lifted/🏪️store/🪞️delegating-accessors-and-reads/🦀️.rs","⚙️engine/📥️lifted/🏪️store/📑️delegating-operations-and-publication/🦀️.rs","⚙️engine/🧱️store-components/📥️lifted/🏪️store/🖱️store-cursor-operations/🦀️.rs","⚙️engine/🎞️live/📥️lifted/🏪️store/🎞️event-sourced-history-reprojection/🦀️.rs","⚙️engine/🎞️live/📥️lifted/🏪️store/📸️snapshot-reads-and-retained-replacements/🦀️.rs","⚙️engine/🎞️live/📥️lifted/🏪️store/⏳️deferred-reprojection-and-supersede-reports/🦀️.rs","🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs","🔁️replay/🔢️prefix-digests-replay-progress-and-edit-replay/🦀️.rs","🔁️replay/⏩️replay-owned-state-operations/🦀️.rs","🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs","🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🧪️tests/authority.json","🕰️history/🏁️replay/🦀️.rs","🕰️history/🏁️replay/checkpoint.rs","🕰️history/🏁️replay/checkout.rs","🕰️history/🏁️replay/📚️metadata/🦀️.rs","🕰️history/🏁️replay/checkpoint-order.rs","🕰️history/🎮️command/🦀️.rs","🕰️history/🎮️command/📦️packages/🦀️rust/Cargo.toml","🦀️.rs","🧩️composition/👥️space-member/📥️lifted/🏪️store/🦀️.rs","🧩️composition/👥️space-member/📥️lifted/🏪️store/🏪️space-member-for-artifact-store/🦀️.rs","🧩️composition/📬️publication/🤝️group/🦀️.rs"]
plugin_files=["🫧️transient/🧵️publication/🦀️.rs","🪟️window/🎚️config/🧭️owner-registry/🦀️.rs","🪟️window/🎚️config/🗄️store-owners/🦀️.rs","📜️declaration/🎞️media/🦀️.rs","🎞️media/📤️snapshot/🦀️.rs","🛠️tool-machine/🦀️.rs","🛠️tool-machine/🪞️window-preview/🦀️.rs","👥️presence/♻️retirement/🦀️.rs","📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs"]
plugin_files += ["🧵️instance/🏪️stores/📥️staging/📨️dispatch/🦀️.rs","🧵️instance/🪆️children/📥️staging/🧱️components/🦀️.rs"]
plugin_files += ["🧵️instance/👷️jobs/📥️staging/🔒️close-ladder/🦀️.rs","🧵️instance/🪆️children/📥️staging/🤝️group/🦀️.rs"]
selected={p for p in statuses if p.startswith(layout+"/")}
selected|={store+"/"+p for p in store_files}
selected|={plugin+"/"+p for p in plugin_files}
selected|={p for p in statuses if p.startswith(store+"/🔁️replay/🧪️tests/🔐️authority/") or p.startswith(scopes[3]+"/")}
selected|={"Cargo.toml","Cargo.lock","🧰️framework/Cargo.toml","📋️build-owners.json"}
process="🧰️framework/🔨️modules/🏃️process"
selected|={process+"/"+p for p in ["📋️project.json","🧪️testing/🗂️suites/🔣️.json","📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🧪️tests/🔣️.json","📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🧪️tests/🟦️.ts","📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🧪️tests/🧬️schema/🔣️.json"]}

for parent,directories,files in os.walk(ticket,topdown=True,followlinks=False):
 directories[:]=[name for name in directories if name not in {"🗑️generated","__pycache__"}]
 for name in files:
  path=Path(parent)/name
  if path.is_file():selected.add(path.as_posix())
selected|={(ticket/"working-files.json").as_posix(),(ticket/"working-files-audit.md").as_posix()}
packet=json.loads((ticket/"mutations-working-files.json").read_text())
for key in ["createdUpdated","removed","reportsInputs"]:
 selected.update(packet.get(key,[]))
rust_packet=json.loads((ticket/"rust-working-files.json").read_text())
for key in ["files","removed","shared_manifests","ticket_inputs"]:
 selected.update(rust_packet.get(key,[]))
source_packet=json.loads((ticket/"source-lifetime-working-files.json").read_text())
for key in ["files","removed","reportsInputs"]:
 selected.update(source_packet.get(key,[]))
host_packet=json.loads((ticket/"host-policy-working-files.json").read_text())
for key in ["files","removed","reportsInputs"]:
 selected.update(host_packet.get(key,[]))
for packet_name in ["owned-emission-working-files.json","input-admission-working-files.json","typing-wiring-working-files.json","typing-original-working-files.json","idml-working-files.json","typing-fold-working-files.json","typing-edit-working-files.json","typing-edit-inlet-working-files.json","setup-publication-working-files.json","typescript-recheck-working-files.json","sdk-actor-admission-working-files.json","dependency-layout-working-files.json","unready-capsule-working-files.json","mounted-initial-working-files.json","compiler-preview-route-working-files.json","opaque-metadata-working-files.json","generation-absent-working-files.json","provider-pressure-working-files.json"]:
 extra=json.loads((ticket/packet_name).read_text())
 if isinstance(extra,list):selected.update(extra)
 else:
  for key in ["files","createdUpdated","removed","reportsInputs"]:selected.update(extra.get(key,[]))
drawing_removed=set()
drawing_packets=['drawing-root-context-renderer-current-working-files.json','drawing-context-public-fixture-repair-working-files.json','drawing-worker-owned-return-working-files.json','drawing-worker-turn-current-api-working-files.json','drawing-pdf-fill-frontier-working-files.json','drawing-ts-flatten-test-first-working-files.json','drawing-ts-flatten-body-working-files.json','drawing-retirement-cpu-source-working-files.json','drawing-ts-buffer-implementation-working-files.json','drawing-ts-buffer-current-law-working-files.json','drawing-retirement-cpu-test-first-working-files.json','drawing-raster-stack-planner-source-working-files.json','drawing-original-fold-working-files.json','drawing-boolean-pixels-owning-source-files.json','drawing-scene-boolean-owning-source-files.json','drawing-scene-raster-owning-source-files.json','drawing-trace-placement-owned-counters-files.json','drawing-ts-buffer-test-first-working-files.json','drawing-raster-stack-test-first-working-files.json','drawing-neutral-extraction-removal-working-files.json','drawing-raster-borrowed-error-message-working-files.json','drawing-original-work-source-packet.json','drawing-stroke-test-migration-working-files.json','drawing-query-literal-errors-working-files.json','drawing-private-stroke-test-working-files.json','drawing-png-typescript-working-files.json','drawing-path-boolean-source-working-files.json','drawing-native-nested-slot-depth-working-files.json','drawing-native-lower-working-files.json','drawing-font-producer-admission-working-files.json','drawing-font-normalization-working-files.json','drawing-font-catalog-working-files.json','drawing-cursor-current-working-files.json','drawing-canonical-wire-cpu-working-files.json','drawing-bytes-actual-turn-working-files.json','drawing-boolean-heap-working-files.json',"drawing-json-reference-working-files.json","drawing-selection-working-files.json","drawing-retirement-trait-working-files.json","model-validation-companion-working-files.json","model-test-ownership-working-files.json","owned-validation-working-files.json","setter-tokenization-working-files.json","collection-delimiter-working-files.json","drawing-text-work-working-files.json","drawing-native-composition-test-first-working-files.json","drawing-serializer-original-test-working-files.json","retirement-macro-working-files.json","drawing-neutral-work-stable-packet.json","drawing-neutral-work-edited-inputs.json","drawing-preparation-funded-working-files.json","drawing-font-typescript-working-files.json","drawing-numeric-typescript-working-files.json","drawing-scene-paint-test-first-working-files.json","drawing-scene-paint-source-working-files.json","drawing-scene-paint-test-migration-working-files.json","drawing-scene-paint-callers-working-files.json","drawing-scene-paint-direct-tests-working-files.json","drawing-vector-direct-tests-working-files.json","drawing-coverage-test-first-working-files.json","drawing-scene-wallet-working-files.json","drawing-scene-paint-phase-working-files.json","drawing-diagnostic-message-working-files.json","drawing-diagnostic-message-root-working-files.json","drawing-coverage-source-working-files.json",
    "drawing-coverage-test-migration-working-files.json",
    'drawing-flatten-independent-reference-working-files.json',
    'drawing-lower-current-working-files.json',
    'drawing-scene-paint-settlement-working-files.json',
    'drawing-path-raster-source-working-files.json',
    'drawing-path-raster-test-first-working-files.json',
    'drawing-fill-source-working-files.json',
    'drawing-fill-test-first-working-files.json',"drawing-pixels-observation-working-files.json"]
drawing_packets += ['drawing-root-current-working-files.json','drawing-writer-consolidation-working-files.json','drawing-writer-test-first-working-files.json','drawing-equality-source-working-files.json']
def drawing_path(value):
 path=Path(value)
 if path.is_absolute():return path.relative_to(root).as_posix()
 return path.as_posix()
drawing_packets += ['drawing-native-storage-working-files.json', 'drawing-font-producer-current-working-files.json', 'drawing-glyph-helper-auth-test-first.json']
drawing_packets += ['drawing-clone-close-ownership-working-files.json']
drawing_packets += ['drawing-ordered-shared-work-files.json']
drawing_packets += ['drawing-original-return-source-current.json']
drawing_packets += ['drawing-clone-caller-outcome-working-files.json']
drawing_packets += ['drawing-dynamic-test-owner-working-files.json', 'drawing-ordered-work-current-files.json']
drawing_packets += ['drawing-native-ledger-work-working-files.json', 'drawing-cancel-alias-working-files.json', 'drawing-child-inherited-budget-working-files.json', 'drawing-source-loan-callers-working-files.json', 'drawing-native-decode-command-3.json', 'drawing-font-outline-command-2.json', 'drawing-sealed-work-command-5.json']
drawing_packets += ['drawing-projection-test-outcome-working-files.json','drawing-native-sequence-storage-working-files.json']
drawing_packets += ['drawing-original-slot-work-working-files.json', 'drawing-retained-map-work-working-files.json', 'drawing-ordered-sync-removal-working-files.json', 'drawing-native-map-working-files.json', 'drawing-native-set-working-files.json', 'drawing-font-run-current-working-files.json', 'drawing-font-bidi-test-first.json', 'drawing-projection-boxed-test-first.json']
drawing_packets += ['drawing-retirement-queue-work-working-files.json', 'drawing-retirement-queue-test-outcome-working-files.json', 'drawing-retirement-turn-work-working-files.json', 'drawing-persistent-project-work-working-files.json', 'drawing-persistent-builder-normal-working-files.json', 'drawing-persistent-set-working-files.json', 'drawing-persistent-splice-working-files.json', 'drawing-persistent-read-working-files.json', 'drawing-ordered-sync-removal-working-files.json', 'drawing-retained-map-test-outcome-working-files.json', 'drawing-font-run-bidi-command-2.json', 'drawing-named-rank-test-first.json']
drawing_packets += ["drawing-indexed-work-test-first.json","drawing-named-index-test-first.json","drawing-font-run-bidi-command-3.json"]
drawing_packets += ["drawing-named-source-working-files.json"]
drawing_packets += ["drawing-cancel-root-working-files.json","drawing-named-test-outcome-accesses.json","drawing-font-paragraph-command.json","drawing-font-corpus-run-command.json","drawing-job-registry-construction-working-files.json"]
drawing_packets += ["drawing-cancel-root-test-first.json"]
drawing_packets += ["drawing-font-corpus-run-command-2.json","drawing-controller-current-owner-working-files.json"]
drawing_packets += ['drawing-value-upper-working-files.json', 'drawing-native-io-builder-working-files.json', 'drawing-value-sort-owner-working-files.json', 'drawing-paged-text-close-working-files.json', 'drawing-paged-receive-working-files-2.json']
drawing_packets += ['drawing-read-owner-working-files.json', 'drawing-native-io-renderer-supplier-working-files.json', 'drawing-text-owned-working-files.json']
drawing_packets += ['drawing-source-return-place-working-files.json', 'drawing-standalone-paint-value-working-files.json', 'drawing-glyph-owned-corpus-working-files.json']
drawing_packets += ['drawing-job-failure-owner-working-files.json', 'drawing-continuation-view-working-files.json', 'drawing-svg-owned-source-working-files.json', 'drawing-svg-original-caller-working-files.json', 'drawing-glyph-svg-product-working-files.json', 'drawing-png-public-type-reexport-working-file.json']
for packet_name in drawing_packets:
 extra=json.loads((ticket/packet_name).read_text())
 if isinstance(extra,list):
  for value in extra:selected.add(drawing_path(value["path"] if isinstance(value,dict) else value))
  continue
 for value in extra.get("files",[])+extra.get("inputs",[]):
  selected.add(drawing_path(value["path"] if isinstance(value,dict) else value))
  if isinstance(value,dict)and value.get("status")=="removed":drawing_removed.add(drawing_path(value["path"]))
 for row in extra.get("changes",[]):selected.add(drawing_path(row["path"]))
 for value in extra.get("reportsInputs",[]):
  selected.add(drawing_path(value) if (root/value).exists() else (ticket/value).as_posix())
 for value in extra.get("reports",[]):selected.add((ticket/value).as_posix())
 if extra.get("report"):
  value=extra["report"];selected.add(drawing_path(value) if Path(value).is_absolute() or (root/value).exists() else (ticket/value).as_posix())
 for value in extra.get("removed",[]):drawing_removed.add(drawing_path(value["path"] if isinstance(value,dict) else value))
 for row in extra.get("entries",[]):
  if row.get("status")=="removed":drawing_removed.add(drawing_path(row["path"]))
removed=set(rust_packet.get("removed",[]))|set(source_packet.get("removed",[]))|set(packet.get("removed",[]))|drawing_removed
generated_excluded=sorted(p for p in selected if "🗑️generated" in Path(p).parts or "__pycache__" in Path(p).parts)
selected-=set(generated_excluded)
uncertain=[p for p in selected if p not in statuses and not (root/p).exists() and p not in removed]
selected-=set(uncertain)
(root/ticket/"working-files.json").write_text(json.dumps(sorted(selected),ensure_ascii=False,indent=2)+"\n")
unattributed=[p for p in statuses if p not in selected and (p.startswith(store+"/") or p.startswith(plugin+"/"))]
report="# Working Files Attribution Audit\n\nCompletion preparation only. The actual UI objective is not achieved; ticket/goal remain open. This list records touched files, not ownership of every peer hunk in those shared files. Scoped status and rust/fonts/typescript/mutations/shared-replay/audit reports plus writer confirmations establish the included domains. Generated outputs are excluded. Root/framework Cargo files are listed as touched shared roots even where our temporary member addition was later reverted.\n\nworking-files.json contains "+str(len(selected))+" exact repository-relative paths. All current task ticket inputs/reports are retained, including this audit and inventory generator. The Layout scoped status is corroborated by the artifact source ownership, font companion, mutation decisions and TypeScript packets; unrelated shared Store/plugin status is NOT copied wholesale.\n\n## Uncertain or Unresolved Paths\n\n"+("\n".join("- "+p for p in uncertain)or"No nonexistent inferred paths retained.")+"\n\n## Attributed Historical Removals\n\n"+"\n".join("- "+p for p in sorted(removed))+"\n\n## Shared Status Excluded Pending Attribution\n\nThese paths changed in the same broad infrastructure tree but are outside our exact documented file roster; no peer work is claimed merely because status lists them.\n\n"+"\n".join("- "+p for p in unattributed)+"\n\nThe document StoreState/StoreMutationFor rollout, Editor/Viewer associated-contract changes and native build/cache custody implementation belong to concurrent owners unless the coordinator supplies specific same-task attribution. No git mutation, source edit, build or ticket close was performed by this inventory.\n"
(ticket/"working-files-audit.md").write_text(report)
inventory={"included":len(selected),"unresolved":sorted(uncertain),"sharedExcluded":sorted(unattributed),"historicalRemoved":sorted(removed),"generatedExcluded":generated_excluded,"providerPacket":"provider-pressure-working-files.json","drawingPackets":drawing_packets,"attribution":"Exact documented task packets and existing scoped attribution only; shared files do not claim peer hunks."}
(root/ticket/"🗑️generated"/"working-files-attribution.json").write_text(json.dumps(inventory,ensure_ascii=False,indent=2)+"\n")
(root/ticket/"🗑️generated"/"working-files-scoped-status.json").write_text(json.dumps(statuses,ensure_ascii=False,indent=2)+"\n")
print(json.dumps({"included":len(selected),"unresolved":uncertain,"sharedExcluded":len(unattributed),"generatedExcluded":len(generated_excluded)}))
