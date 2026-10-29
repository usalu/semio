import json, subprocess
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

for p in ticket.rglob("*"):
 if p.is_file() and "🗑️generated" not in p.parts and "__pycache__" not in p.parts:selected.add(p.relative_to(root).as_posix() if p.is_absolute() else p.as_posix())
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
for packet_name in ["owned-emission-working-files.json","input-admission-working-files.json","typing-wiring-working-files.json","typing-original-working-files.json","idml-working-files.json"]:
 extra=json.loads((ticket/packet_name).read_text())
 if isinstance(extra,list):selected.update(extra)
 else:
  for key in ["files","removed","reportsInputs"]:selected.update(extra.get(key,[]))
uncertain=[p for p in selected if p not in statuses and not (root/p).exists()]
selected-=set(uncertain)-(set(rust_packet.get("removed",[]))|set(source_packet.get("removed",[])))
(root/ticket/"working-files.json").write_text(json.dumps(sorted(selected),ensure_ascii=False,indent=2)+"\n")
unattributed=[p for p in statuses if p not in selected and (p.startswith(store+"/") or p.startswith(plugin+"/"))]
report="# Working Files Attribution Audit\n\nCompletion preparation only. The actual UI objective is not achieved; ticket/goal remain open. This list records touched files, not ownership of every peer hunk in those shared files. Scoped status and rust/fonts/typescript/mutations/shared-replay/audit reports plus writer confirmations establish the included domains. Generated outputs are excluded. Root/framework Cargo files are listed as touched shared roots even where our temporary member addition was later reverted.\n\nworking-files.json contains "+str(len(selected))+" exact repository-relative paths. All current task ticket inputs/reports are retained, including this audit and inventory generator. The Layout scoped status is corroborated by the artifact source ownership, font companion, mutation decisions and TypeScript packets; unrelated shared Store/plugin status is NOT copied wholesale.\n\n## Uncertain or Unresolved Paths\n\n"+("\n".join("- "+p for p in uncertain)or"No nonexistent inferred paths retained.")+"\n\n## Shared Status Excluded Pending Attribution\n\nThese paths changed in the same broad infrastructure tree but are outside our exact documented file roster; no peer work is claimed merely because status lists them.\n\n"+"\n".join("- "+p for p in unattributed)+"\n\nThe document StoreState/StoreMutationFor rollout, Editor/Viewer associated-contract changes and native build/cache custody implementation belong to concurrent owners unless the coordinator supplies specific same-task attribution. No git mutation, source edit, build or ticket close was performed by this inventory.\n"
(ticket/"working-files-audit.md").write_text(report)
print(json.dumps({"included":len(selected),"unresolved":uncertain,"sharedExcluded":len(unattributed)}))
