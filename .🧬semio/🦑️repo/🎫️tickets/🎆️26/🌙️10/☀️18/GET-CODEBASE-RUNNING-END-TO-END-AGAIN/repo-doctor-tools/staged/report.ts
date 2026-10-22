import {doctorTextV1,type DoctorFindingV1,type DoctorLanguageV1,type DoctorPhaseV1,type DoctorReportV1,type DoctorWaitV1} from "../🟦️.ts";

const PHASE_KEY:Readonly<Record<DoctorPhaseV1,string>>={"waiting-lease":"phaseWaitingLease","waiting-capacity":"phaseWaitingCapacity",holding:"phaseHolding",admitted:"phaseAdmitted",running:"phaseRunning",orphan:"phaseOrphan"};
const SEVERITY_KEY={error:"severityError",warn:"severityWarn",info:"severityInfo"} as const;
const VERDICT_KEY={ok:"verdictOk",degraded:"verdictDegraded",blocked:"verdictBlocked"} as const;
const PROBE_KEY={ok:"probeOk",unavailable:"probeUnavailable",failed:"probeFailed",timeout:"probeTimeout",skipped:"probeSkipped"} as const;
const STATE_KEY={running:"rootStateRunning",waiting:"rootStateWaiting",gone:"rootStateGone",unknown:"rootStateUnknown"} as const;

/** 🖨️ Renders the fix of a finding as one or two lines. */
function fixLines(language:DoctorLanguageV1,row:DoctorFindingV1):readonly string[]{
 const note=row.fix.note[language];
 if(row.fix.kind==="automatic")return [doctorTextV1(language,"fixAutomatic",{command:row.fix.command??"",note})];
 if(row.fix.kind==="command")return [doctorTextV1(language,"fixCommand",{command:row.fix.command??"",note})];
 if(row.fix.kind==="wait")return [doctorTextV1(language,"fixWait",{note})];
 return [row.fix.command===undefined?doctorTextV1(language,"fixAsk",{note}):doctorTextV1(language,"fixAskCommand",{note,command:row.fix.command})];
}

/** ⏳️ Renders one wait chain: who waits, behind whom and the root cause. */
function waitLines(language:DoctorLanguageV1,row:DoctorWaitV1):readonly string[]{
 const lines=[row.kind==="lease"
  ?doctorTextV1(language,"waitLease",{pid:row.pid,command:row.command,waited:row.waitedSeconds,subject:row.subject,position:row.position===null?"":doctorTextV1(language,"waitPosition",{position:row.position})})
  :doctorTextV1(language,"waitCapacity",{pid:row.pid,command:row.command,waited:row.waitedSeconds,operation:row.subject,reason:row.reason})];
 for(const party of row.behind)lines.push(doctorTextV1(language,party.role==="holder"?"behindHolder":party.role==="ahead"?"behindAhead":"behindGrant",{pid:party.pid,command:party.command,held:party.heldSeconds,reserved:party.reservedMegabytes}));
 if(row.root)lines.push(doctorTextV1(language,"rootLine",{pid:row.root.pid,command:row.root.command,state:doctorTextV1(language,STATE_KEY[row.root.state]),held:row.root.heldSeconds===null?"":doctorTextV1(language,"rootHeld",{held:row.root.heldSeconds})}));
 return lines;
}

/** 📄️ Renders the report in one language: verdict, wait chains, findings with their fixes, running processes, repairs and probes. */
export function formatDoctorReportInV1(language:DoctorLanguageV1,report:DoctorReportV1):readonly string[]{
 const text=(key:string,values:Readonly<Record<string,string|number|boolean|null|undefined>>={}):string=>doctorTextV1(language,key,values),lines:string[]=[];
 lines.push(text("header",{verdict:text(VERDICT_KEY[report.verdict])}),text("counts",report.counts));
 const ran=(area:string):boolean=>report.probes.some(row=>row.area===area&&row.status!=="skipped");
 if(report.waits.length>0||ran("leases")&&ran("capacity")){
  lines.push("","## "+text("headingWaits"));
  if(report.waits.length===0)lines.push(text("noWaits"));
  for(const row of report.waits)lines.push(...waitLines(language,row));
 }
 lines.push("","## "+text("headingFindings"));
 if(report.findings.length===0)lines.push(text("noFindings"));
 for(const row of report.findings){
  lines.push(text("findingLine",{severity:text(SEVERITY_KEY[row.severity]),title:row.title[language],area:row.area,owner:row.owner}),text("detailLine",{detail:row.detail[language]}),...fixLines(language,row));
 }
 if(report.processes.length>0||ran("processes")){
 lines.push("","## "+text("headingProcesses"));
 if(report.processes.length===0)lines.push(text("noProcesses"));
 }
 for(const row of report.processes)lines.push(text("processLine",{pid:row.pid,phase:text(PHASE_KEY[row.phase]),via:row.via===null?"":text("processVia",{via:row.via}),age:row.ageSeconds,rss:row.rssMegabytes,cpu:row.cpuPercent,command:row.command}));
 if(report.repairs.length){
  lines.push("","## "+text("headingRepairs"));
  for(const row of report.repairs){
   const key=row.status==="done"?(row.dryRun?"repairDry":"repairDone"):row.status==="nothing"?"repairNothing":row.status==="unavailable"?"repairUnavailable":row.status==="cancelled"?"repairCancelled":"repairFailed";
   lines.push(text(key,{id:row.id,removed:row.removed,bytes:row.bytes,reason:row.reason}));
  }
 }
 lines.push("","## "+text("headingProbes"));
 for(const row of report.probes)lines.push(text("probeLine",{area:row.area,status:text(PROBE_KEY[row.status]),duration:row.durationMs,reason:row.reason?": "+row.reason:""}));
 return lines;
}

/** 🗞️ Renders the report in every requested language, one block after the other when two are requested. */
export function formatDoctorReportV1(report:DoctorReportV1,languages:readonly DoctorLanguageV1[]):readonly string[]{
 return languages.flatMap((language,index)=>[...index>0?["",""]:[],...formatDoctorReportInV1(language,report)]);
}
