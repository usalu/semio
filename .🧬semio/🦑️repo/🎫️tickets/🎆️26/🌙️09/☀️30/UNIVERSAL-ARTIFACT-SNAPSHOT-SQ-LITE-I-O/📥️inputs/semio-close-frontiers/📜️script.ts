const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const input=`${ticket}/📥️inputs/semio-close-frontiers`;
const owner="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio";
if(process.argv[2]==="verify-owned"){
  let failed=false;
  const cases=[
    ["@semio-tech/flow-flow-rs:test","editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows","immutable-flow-edit-offset-after-semio-close-private.log"],
    ["@semio-tech/flow-flow-rs:test","finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child","immutable-flow-finalize-alternative-after-semio-close-private.log"],
    ["@semio-tech/flow-flow-rs:test","node_graph_edit::tests","immutable-flow-original-nineteen-after-semio-close-private.log"],
    ["@semio-tech/framework-os-kernel:test-native","artifact_store_snapshot_roots_and_final_envelope_transfer_in_exact_close_order","immutable-semio-close-store-independent-native-private.log"],
  ];
  for(const [target,filter,log] of cases){
    const file=Bun.file(`${ticket}/🗑️generated/${log}`),writer=file.writer();
    const child=Bun.spawn(["bun","nx","run",target,"--skip-nx-cache","--excludeTaskDependencies",`--args=quick ${filter} --no-fail-fast -- --nocapture`],{stdout:"pipe",stderr:"pipe",env:process.env});
    const pump=async(stream:ReadableStream<Uint8Array>)=>{for await(const chunk of stream)writer.write(chunk);};
    await Promise.all([pump(child.stdout),pump(child.stderr)]);
    const code=await child.exited;await writer.end();failed||=code!==0;
    console.log("[DEBUG] "+JSON.stringify({target,filter,exitCode:code,log:`${ticket}/🗑️generated/${log}`}));
  }
  process.exit(failed?1:0);
}
const pairs:{path:string,before:string,after:string}[]=[];
function replace(s:string,a:string,b:string){if(s.split(a).length!==2)throw Error("exact Semio close span guard");return s.replace(a,b);}
async function edit(path:string,change:(s:string)=>Promise<string>|string){const before=await Bun.file(path).text();pairs.push({path,before,after:await change(before)});}
if(process.argv[2]==="demand"){
  await edit(`${owner}/🧪️tests/🔬️unit/🦀️.rs`,async s=>{if(s.includes("supersession_and_prefix_owners_follow_the_exact_semio_close_cursor"))throw Error("demand already present");return s+await Bun.file(`${input}/law.rs`).text();});
  const path=`${owner}/🧪️tests/🧫️fixtures/close-frontiers/🔣️.json`;
  if(await Bun.file(path).exists())throw Error("neutral fixture already exists");
  pairs.push({path,before:"",after:await Bun.file(`${input}/🔣️.json`).text()});
}else if(process.argv[2]==="terminal-witness-join"){
  await edit(`${owner}/🧪️tests/🔬️unit/🦀️.rs`,s=>replace(s,'assert!(store.close_owned_phase_witness().starts_with("semio/Complete/"));','assert_eq!(store.close_owned_phase_witness(), case["terminal_phase_witness"].as_str().unwrap(), "Store removes only the completed terminal-empty disposer");'));
  await edit(`${owner}/🧪️tests/🧫️fixtures/close-frontiers/🔣️.json`,s=>replace(s,'"expected_supersessions":1,','"expected_supersessions":1,"terminal_phase_witness":"uninstalled",'));
}else if(process.argv[2]==="fixture-join"){
  await edit(`${owner}/🧪️tests/🔬️unit/🦀️.rs`,s=>replace(s,"../../🧫️fixtures/close-frontiers/🔣️.json","../🧫️fixtures/close-frontiers/🔣️.json"));
}else if(process.argv[2]==="provider"){
  await edit(`${owner}/🦀️.rs`,s=>{
    s=replace(s,"    EnvelopeMetadata,\n    TailSnapshot,","    EnvelopeMetadata,\n    PrefixSnapshots,\n    TailSnapshot,");
    s=replace(s,"                            6 => dsl::ArtifactStoreCloseStringLane::TailUndoEditId,","                            6 => dsl::ArtifactStoreCloseStringLane::TailUndoEditId,\n                            7 => dsl::ArtifactStoreCloseStringLane::Supersessions,");
    s=replace(s,"                            self.phase = SemioStoreClosePhase::TailSnapshot;\n                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })\n                        }\n                    },\n                    SemioStoreClosePhase::TailSnapshot", "                            self.phase = SemioStoreClosePhase::PrefixSnapshots;\n                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })\n                        }\n                    },\n                    SemioStoreClosePhase::PrefixSnapshots => match store.take_prefix_snapshot_retirement().map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))? {\n                        Some(retirement) => {\n                            *self.active = Some(retirement);\n                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })\n                        }\n                        None => {\n                            self.phase = SemioStoreClosePhase::TailSnapshot;\n                            Ok(dsl::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })\n                        }\n                    },\n                    SemioStoreClosePhase::TailSnapshot");
    return s;
  });
}else throw Error("known Semio close command required");
await Bun.write(`${input}/${process.argv[2]}-guarded-pairs.json`,JSON.stringify({pairs},null,2)+"\n");
for(const pair of pairs)if((await Bun.file(pair.path).exists()?await Bun.file(pair.path).text():"")!==pair.before)throw Error(`concurrent guard failed: ${pair.path}`);
for(const pair of pairs)await Bun.write(pair.path,pair.after);
console.log(JSON.stringify({mounted:pairs.length,command:process.argv[2]}));
