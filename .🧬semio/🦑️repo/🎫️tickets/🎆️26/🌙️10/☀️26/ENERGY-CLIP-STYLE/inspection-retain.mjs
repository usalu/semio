import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

const ticket=path.dirname(fileURLToPath(import.meta.url));
const generated=path.join(ticket,'🗑️generated');
const read=file=>JSON.parse(fs.readFileSync(file,'utf8'));
const digest=file=>crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
const checkedCopy=(source,destination,expected)=>{const before=digest(source);if(expected&&before!==expected)throw Error('Verified artifact identity mismatch');fs.copyFileSync(source,destination);if(digest(destination)!==before||digest(source)!==before)throw Error('Artifact copy identity mismatch: '+destination)};
let workspace=ticket;
while(!fs.existsSync(path.join(workspace,'🎓️teaching'))||!fs.existsSync(path.join(workspace,'🧰️framework'))){const parent=path.dirname(workspace);if(parent===workspace)throw Error('Workspace not found');workspace=parent}
const metrics=read(path.join(generated,'iv','chapter-measurements.json'));
const completed=read(path.join(generated,'i','publish-complete.json'));
const gate=read(path.join(generated,'i','publish-gate.json'));
if(!Number.isFinite(completed.completed)||completed.state!=='complete'||completed.videos!==17||completed.sources!==117||completed.chapters!==38||completed.chapter_exports!==38||gate.state!=='passed'||gate.canonical_sources!==117||gate.chapters!==38||gate.chapter_exports!==38||gate.excluded.length)throw Error('Expected all current production publications');
if(metrics.expected!==38||metrics.verified!==38||metrics.lessons!==17||!metrics.pass||!metrics.joins_pass||!metrics.order_pass||!metrics.provenance_pass||!Number.isFinite(metrics.checked)||metrics.checked<completed.completed)throw Error('Expected independently verified38 chapters,17 lessons and exact adjacent joins');
const chapters=metrics.chapters,videos=metrics.videos,records=[...videos,...chapters];
if(chapters.length!==38||videos.length!==17||new Set(records.map(record=>record.id)).size!==55||records.some(record=>record.pass!==true||!/^\w+$/.test(record.id)||!/^[0-9a-f]{64}$/.test(record.sha256)||!Number.isInteger(record.frames)||record.frames<1||!Number.isFinite(record.duration)||record.duration<=0))throw Error('Incomplete independent media identities');
const proofs=[read(path.join(generated,'iv','chapter-provenance.json')),read(path.join(generated,'iv','bim-closure.json'))];
if(proofs.some(proof=>proof.pass!==true||!Number.isFinite(proof.checked)||proof.checked<completed.completed||Object.keys(proof.files).length<6)||proofs[1].leaves!==6)throw Error('Expected final independent source and BIM closure proofs');
const inside=(root,file)=>{const relative=path.relative(root,file);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)};
const proofsCurrent=()=>proofs.every(proof=>Object.entries(proof.files).every(([file,sha])=>{const source=fs.realpathSync(file);return inside(workspace,source)&&/^[0-9a-f]{64}$/.test(sha)&&digest(source)===sha}));
if(!proofsCurrent())throw Error('Verified inputs changed before retention');
const evidence=proofs[1].evidence,counts=proofs[1].evidence_counts;
if(!Array.isArray(evidence)||counts?.clauses!==113||counts.bim_sources!==6||counts.optimization_122_frames!==9||new Set(evidence.map(item=>item.path)).size!==evidence.length||new Set(evidence.map(item=>path.basename(item.path))).size!==evidence.length||evidence.some(item=>!['bim-clause-sheet','optimization-122-stage'].includes(item.kind)||!/^[0-9a-f]{64}$/.test(item.sha256)||!/^[0-9a-f]{64}$/.test(item.source_sha256)||proofs[1].files[item.path]!==item.sha256))throw Error('Incomplete bound supplementary evidence');
const stageNames=['model','curvature','confirm_3'].flatMap(stage=>['early','settled','late'].map(phase=>'source-122-'+stage+'-'+phase+'.png')).sort();
if(JSON.stringify(evidence.filter(item=>item.kind==='optimization-122-stage').map(item=>path.basename(item.path)).sort())!==JSON.stringify(stageNames)||new Set(evidence.filter(item=>item.kind==='bim-clause-sheet').map(item=>item.clip_id)).size!==6)throw Error('Supplementary stage coverage mismatch');
const sourceRoot=fs.realpathSync(path.join(generated,'i'));
const catalog=read(videos[0].path.replace(/\.mp4$/,'.render.json')).presentation.catalog;
if(!catalog.courses?.length||records.some(record=>JSON.stringify(read(record.path.replace(/\.mp4$/,'.render.json')).presentation.catalog)!==JSON.stringify(catalog)))throw Error('Publication navigation snapshots differ');
const manifest=read(path.join(workspace,'🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json'));
const byClip=new Map(videos.map(record=>[record.id,record])),byChapter=new Map(chapters.map(record=>[record.id,record]));
if(evidence.some(item=>!byClip.has(item.clip_id)||!byChapter.has(item.scene)||byChapter.get(item.scene).clip_id!==item.clip_id||item.kind==='bim-clause-sheet'&&!item.clip_id.startsWith('bim_')||item.kind==='optimization-122-stage'&&item.clip_id!=='optimization_experiments'))throw Error('Supplementary evidence ownership mismatch');
for(const video of manifest.videos){const record=byClip.get(video.id),names=video.chapters.map(chapter=>chapter.scene);if(!record||JSON.stringify(record.chapters)!==JSON.stringify(names)||names.some(scene=>!byChapter.has(scene)||byChapter.get(scene).clip_id!==video.id||record.chapter_sha256[scene]!==byChapter.get(scene).sha256))throw Error('Declared lesson/chapter coverage mismatch')}
const output=path.join(ticket,'inspection-videos'),staging=path.join(generated,'inspection-delivery');
if(fs.existsSync(output)||fs.existsSync(staging))throw Error('Inspection delivery already exists');
fs.mkdirSync(path.join(staging,'chapters'),{recursive:true});
const retained=new Map();
for(const record of records){const input=fs.realpathSync(record.path);if(!inside(sourceRoot,input)||path.extname(input)!=='.mp4'||digest(input)!==record.sha256)throw Error('Verified source identity mismatch: '+record.id);const relative=byChapter.has(record.id)?'chapters/'+record.id+'.mp4':record.id+'.mp4';const destination=path.join(staging,relative);if(destination.length>256)throw Error('Delivery path exceeds repository limit');checkedCopy(input,destination,record.sha256);if(digest(destination)!==record.sha256||digest(input)!==record.sha256)throw Error('Media copy identity mismatch');checkedCopy(input.replace(/\.mp4$/,'.render.json'),destination.replace(/\.mp4$/,'.render.json'));if(byChapter.has(record.id)){checkedCopy(input.replace(/\.mp4$/,'.chapter.json'),destination.replace(/\.mp4$/,'.chapter.json'));for(const image of ['coverage','opening','closing']){const imageSource=path.join(generated,'iv','collection',record.id,image+'.png'),imageDestination=path.join(staging,'chapters',record.id+'-'+image+'.png');checkedCopy(imageSource,imageDestination);if(digest(imageSource)!==digest(imageDestination))throw Error('Audit image identity mismatch')}}retained.set(record.id,relative)}
fs.mkdirSync(path.join(staging,'evidence'));
const evidenceRoot=fs.realpathSync(path.join(generated,'iv'));
for(const item of evidence){const input=fs.realpathSync(item.path),destination=path.join(staging,'evidence',path.basename(input));if(!inside(evidenceRoot,input)||path.extname(input)!=='.png'||destination.length>256)throw Error('Invalid supplementary image path');checkedCopy(input,destination,item.sha256)}
if(!proofsCurrent())throw Error('Verified inputs changed during retention');
fs.renameSync(staging,output);
const link=file=>path.join(ticket,file).replaceAll('\\','/');
const clock=seconds=>Math.floor(Math.round(seconds)/60)+':'+String(Math.round(seconds)%60).padStart(2,'0');
const profile=metrics.profile;
let index='# Inspection Videos\n\nAll 17 complete lessons and 38 individually playable chapter windows. German spectator inspection: '+profile.width+' × '+profile.height+', '+profile.fps+' fps. Every adjacent chapter joins at its exact shared ancestor overview.\n\n[Verification]('+link('inspection-chapter-verification.md')+') · [Visual audit]('+link('inspection-final-epoch-visual.md')+') · [Delivery identities]('+link('inspection-delivery.md')+') · [Clause and stage evidence]('+link('inspection-evidence.md')+')\n';
let delivery='# Inspection Delivery\n\nAll retained movies are byte-identical to the independently decoded publications. The 17 lessons preserve every frame of their ordered 38 chapter movies.\n\n| Movie | Duration | Frames | Bytes | SHA-256 |\n| --- | ---: | ---: | ---: | --- |\n';
let lessonCount=0,chapterCount=0;
for(const course of catalog.courses){index+='\n## '+course.title[profile.language]+'\n';for(const topic of course.topics){if(!topic.videos.length)continue;index+='\n### '+topic.title[profile.language]+'\n';for(const clip of topic.videos){const video=byClip.get(clip.id);if(!video)throw Error('Missing declared clip');const file='inspection-videos/'+retained.get(clip.id);index+='\n#### '+clip.title[profile.language]+' · '+clock(video.duration)+'\n\n[Play complete clip]('+link(file)+')\n\n!['+clip.title[profile.language]+']('+link(file)+')\n\n| Chapter | Duration | Inspection |\n| --- | ---: | --- |\n';for(const chapter of clip.chapters){const measured=byChapter.get(chapter.id);if(!measured)throw Error('Missing declared chapter');index+='| ['+chapter.title[profile.language]+']('+link('inspection-videos/'+retained.get(chapter.id))+') | '+clock(measured.duration)+' | [Coverage]('+link('inspection-videos/chapters/'+chapter.id+'-coverage.png')+') · [Opening]('+link('inspection-videos/chapters/'+chapter.id+'-opening.png')+') · [Closing]('+link('inspection-videos/chapters/'+chapter.id+'-closing.png')+') |\n';chapterCount++}lessonCount++}}}
if(lessonCount!==17||chapterCount!==38)throw Error('Gallery coverage mismatch');
for(const record of records){const relative=retained.get(record.id);delivery+='| ['+record.id+']('+link('inspection-videos/'+relative)+') | '+record.duration.toFixed(3)+'s | '+record.frames+' | '+fs.statSync(path.join(output,relative)).size+' | '+record.sha256+' |\n'}
let evidenceReport='# Inspection Clause And Stage Evidence\n\nCurrent 113-clause BIM sheets and nine optimization experiment stage frames, bound to the final native source certificates and copied with before/after SHA-256 checks.\n\n| Clip | Evidence | Native Source SHA-256 | PNG SHA-256 |\n| --- | --- | --- | --- |\n';
for(const item of evidence)evidenceReport+='| '+item.clip_id+' | ['+path.basename(item.path)+']('+link('inspection-videos/evidence/'+path.basename(item.path))+') | '+item.source_sha256+' | '+item.sha256+' |\n';
fs.writeFileSync(path.join(ticket,'inspection-evidence.md'),evidenceReport);
fs.writeFileSync(path.join(ticket,'inspection.md'),index);
fs.writeFileSync(path.join(ticket,'inspection-delivery.md'),delivery);
process.stdout.write(JSON.stringify({lessons:lessonCount,chapters:chapterCount,output,index:path.join(ticket,'inspection.md')})+'\n');
