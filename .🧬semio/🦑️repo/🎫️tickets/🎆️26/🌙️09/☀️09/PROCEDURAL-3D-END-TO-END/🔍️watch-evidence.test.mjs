import { test } from "node:test";
import assert from "node:assert/strict";
import { deadlineMs, reactEvidence, runId, wgpuEvidence } from "./🔍️watch-evidence.mjs";

test("fresh named React evidence must pass both hover directions and clear", () => {
  const summary={label:"canonical-react-new",ok:true}, hover={graphToPreview:true,previewToGraph:true,cleared:true,error:null};
  const marker={id:"new",started:100,deadline:200};
  const evidence=(dir,record=summary,mark=hover,modified=110,now=120)=>reactEvidence("new",dir,record,mark,marker,modified,now);
  assert.equal(evidence("probe-canonical-react-new-123",summary,hover,99),false);
  assert.equal(evidence("probe-canonical-react-new-123",summary,hover,110,201),false);
  assert.equal(evidence("probe-canonical-react-old-123"),false);
  assert.equal(evidence("probe-canonical-react-new-123",{...summary,ok:false},hover),false);
  assert.equal(evidence("probe-canonical-react-new-123",summary,{...hover,cleared:false}),false);
  assert.equal(evidence("probe-canonical-react-new-123",summary,hover),true);
});

test("WGPU required outcomes reject failure, blocked, partial and missing proof", () => {
  const labels=["boot:edit-editor","edit:box-shell-bidirectional-hover","world3d:hover","world3d:selection-click","world3d:wheel-orbit"];
  const rows=labels.map(label=>({label,verdict:"pass"}));
  assert.equal(wgpuEvidence({booted:true},rows,""),true);
  assert.equal(wgpuEvidence({booted:true},rows.slice(1),""),false);
  assert.equal(wgpuEvidence({booted:true},[...rows,{label:"edit:picker",verdict:"blocked"}],""),false);
  assert.equal(wgpuEvidence({booted:true},rows.map(row=>row.label==="world3d:hover"?{...row,verdict:"partial"}:row),""),false);
  assert.equal(wgpuEvidence({booted:false},rows,""),false);
  assert.equal(wgpuEvidence({booted:true},rows.slice(0,1),"boot"),true);
});

test("run names and deadlines reject ambiguous input", () => {
  assert.throws(()=>runId("../old"));assert.throws(()=>runId(""));assert.equal(runId("new-1"),"new-1");
  assert.throws(()=>deadlineMs("0"));assert.equal(deadlineMs("2"),2000);
});
