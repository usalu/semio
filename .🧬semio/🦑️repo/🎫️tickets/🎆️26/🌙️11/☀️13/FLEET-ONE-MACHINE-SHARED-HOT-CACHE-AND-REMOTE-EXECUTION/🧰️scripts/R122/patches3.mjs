const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/";
export const PN=L+"🟨️.mjs",SV=L+"🏊️project-farm/👷️service/🟨️.mjs";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0)throw Error("anchor missing: "+from.slice(0,80));if(text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,80));return text.replace(from,()=>to);};
export const all=[
 {file:SV,apply:edit(`/** 🧭️ The inference environment and the staging directory`,`/** 🗃️ Runs \`body\` with the private staging directory \`directory\` as the graph data directory: a farm child holds no custody of the graph store of its parent, so what a job writes beside its answer (the optional program receipts of the closure resolution) goes to a private directory that the parent removes and recomputes when it needs it, never to the store. */
export function stagedWithinV1(directory, body) {
  mkdirSync(directory, { recursive: true });
  return withBuildGraphStagingV1({ root: directory }, body);
}

/** 🧭️ The inference environment and the staging directory`)},
 {file:SV,apply:edit(`if (job.observe === true) return observeProjectRecordV1(environment, job);`,`if (job.observe === true) return stagedWithinV1(join(staging, "observe"), () => observeProjectRecordV1(environment, job));`)},
 {file:PN,apply:edit(`const setAside=error=>deferOwner(error)||`,`const failedOwner=error=>{if(error?.code!=="graph.farm-job-failed")return false;const owner=jobs.find(job=>job.json.root===error.semioProject||job.name===error.semioProject);if(owner===undefined)return false;const root=owner.json.root,reason=String(error.message).slice(0,2048);unavailable.set(root,{root,project:owner.name,source:buildPath(join(root,PROJECT_BASENAME)),sha256:createHash("sha256").update(JSON.stringify(owner.json)).digest("hex"),reason});delete owner.result.projects[owner.name];takeSay(()=>"owner "+owner.name+" is set aside, the read goes on and ends naming it: "+reason+" / Projekt "+owner.name+" wird zurückgestellt, der Lauf geht weiter und nennt es am Ende: "+reason);return true;};
    const setAside=error=>deferOwner(error)||failedOwner(error)||`)},
];
export const apply=(file,text,list=all)=>{for(const item of list)if(item.file===file)text=item.apply(text);return text;};
