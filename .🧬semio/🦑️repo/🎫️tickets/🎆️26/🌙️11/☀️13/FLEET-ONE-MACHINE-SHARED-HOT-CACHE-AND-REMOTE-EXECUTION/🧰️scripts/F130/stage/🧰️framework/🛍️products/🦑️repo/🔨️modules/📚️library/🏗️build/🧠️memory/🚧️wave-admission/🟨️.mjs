import{noticeLanguagesV1}from"../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import{currentHostProbeV1,readHostSampleV1,wholeHostProbeV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import{BUILD_HOST_CAPACITY_POLICY_V1}from"../../🌡️host-capacity/🎛️control/🟦️.ts";
import{footprintGateV1,footprintOverGrantV1}from"../🦶️footprint/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🚧️ The declared numbers of the wave admission: the schema file is the single source. */
export const WAVE_ADMISSION_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));

/** 🗣️ The bilingual lines of the wave admission, read from the schema file. */
export const WAVE_ADMISSION_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));
const P=WAVE_ADMISSION_POLICY_V1,MEGABYTE=1048576,RANK={normal:0,warn:1,critical:2},megabytes=bytes=>Math.round(bytes/MEGABYTE);

/** 💬️ Renders one line of the wave admission as one line per language of the environment. */
export function renderWaveAdmissionMessageV1(key,fields,environment=process.env){
 return[...noticeLanguagesV1(P.languages,environment)].map(language=>{
  const template=WAVE_ADMISSION_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed wave admission message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

/** ⚖️ What the live state asks of the admission of new projects: `overGrant` from the over-grant share of the grant, `hostBusy` while the host pages now (pressure level at `holdPressure`, swap-out rate at the limit or available memory under the floor; swap USED never holds), else `admit`; an unknown host sample (undefined) holds nothing. */
export function decideWaveAdmissionV1({footprintBytes,grantBytes,host,footprintPolicy,policy=P}){
 if(footprintOverGrantV1({footprintBytes,grantBytes,...footprintPolicy===undefined?{}:{policy:footprintPolicy}}))return"overGrant";
 if(host===undefined)return"admit";
 const level=RANK[host.pressure]??0,rate=host.swapOutMegabytesPerSecond;
 return level>=RANK[policy.holdPressure]||(typeof rate==="number"&&rate>=policy.swapOutMegabytesPerSecond)||host.availableMegabytes<policy.availableFloorMegabytes?"hostBusy":"admit";
}

/** 🌡️ Reads the live host pressure through the host-capacity sampler (`readHostSampleV1`: macOS vm_stat, sysctl vm.swapusage and the memorystatus level, Linux /proc/meminfo, /proc/vmstat and PSI, Windows commit charge): `current()` answers at once with the last reading (undefined when none or older than `sampleStaleMilliseconds`) and starts one background reading when it is older than `sampleTtlMilliseconds` (one reading at a time, a failed one is ignored), so a look never spawns a process; `refresh()` waits for a fresh one. The swap-out rate needs two readings, the reader keeps the previous one. */
export function createHostPressureReaderV1({read=previous=>readHostSampleV1(BUILD_HOST_CAPACITY_POLICY_V1,wholeHostProbeV1(currentHostProbeV1()),previous),now=()=>Date.now(),ttlMilliseconds=P.sampleTtlMilliseconds,staleMilliseconds=P.sampleStaleMilliseconds}={}){
 let sample,at=-Infinity,flight,previous;
 const begin=()=>flight??=(async()=>{try{const row=await read(previous);previous={atMs:row.host.atMs,swappedOutMegabytes:row.swappedOutMegabytes};sample=row.host;at=now();}catch{}finally{flight=undefined;}return sample;})();
 const known=()=>now()-at<staleMilliseconds?sample:undefined;
 return Object.freeze({
  current(){if(now()-at>=ttlMilliseconds)void begin();return known();},
  async refresh(){if(flight!==undefined)await flight;await begin();return known();},
 });
}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/**
 * 🚦️ The admission of new projects and of the start of a wave under memory pressure: `holds()` answers synchronously with the state that holds (`overGrant`, `hostBusy`) or `admit` (always `admit` in a process without footprint gate); `between(signal)` waits, cancellable, while a state holds: over grant it releases between waves (at most once per `settleMinimumMilliseconds`, no cooldown), every `progressMilliseconds` it says a bilingual line with the numbers, and after `overGrantMaxWaitMilliseconds` or `hostMaxWaitMilliseconds` it returns `reduced`: `narrow()` is then true until the state is gone and the caller admits `reducedWidth` project at a time, never zero.
 * @see ./🧬️schema/🔣️.json
 * @param {{gate?:()=>any;host?:{current:()=>any;refresh:()=>Promise<any>};sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;report?:(line:string)=>void;environment?:Record<string,string|undefined>;policy?:typeof WAVE_ADMISSION_POLICY_V1}} options
 */
export function createWaveAdmissionV1({gate=footprintGateV1,host=createHostPressureReaderV1(),sleep=defaultSleep,now=()=>Date.now(),report,environment=process.env,policy=P}={}){
 let reduced=false,lastSettle=-Infinity;
 const stats={holds:0,waits:0,waitedMilliseconds:0,settles:0,reduced:0};
 const read=()=>{const installed=gate(),grantBytes=installed?.grantBytes?.();return{installed,grantBytes,footprintBytes:grantBytes>0?installed.footprintBytes():0,sample:host.current()};};
 const decide=row=>row.installed===undefined||!(row.grantBytes>0)?"admit":decideWaveAdmissionV1({footprintBytes:row.footprintBytes,grantBytes:row.grantBytes,host:row.sample,policy});
 const fields=(row,seconds,limit)=>({footprint:megabytes(row.footprintBytes),grant:megabytes(row.grantBytes??0),percent:row.grantBytes>0?Math.round(row.footprintBytes*100/row.grantBytes):0,rate:row.sample?.swapOutMegabytesPerSecond??"?",ratelimit:policy.swapOutMegabytesPerSecond,pressure:row.sample?.pressure??"?",holdlevel:policy.holdPressure,available:row.sample?.availableMegabytes??"?",floor:policy.availableFloorMegabytes,swapused:row.sample?.swapUsedMegabytes??"?",seconds,limit,width:policy.reducedWidth});
 const say=(key,values)=>{if(report!==undefined)for(const line of renderWaveAdmissionMessageV1(key,values,environment))report(line);};
 const holds=()=>{const verdict=decide(read());if(verdict==="admit")reduced=false;return verdict;};
 return Object.freeze({
  holds,
  narrow:()=>{if(reduced&&holds()==="admit")reduced=false;return reduced;},
  async between(signal){
   let row=read(),verdict=decide(row);
   if(verdict==="admit"){reduced=false;return"admit";}
   if(reduced)return"reduced";
   stats.holds++;
   const began=now();let said=-Infinity;
   while(verdict!=="admit"){
    signal?.throwIfAborted();
    const limit=verdict==="overGrant"?policy.overGrantMaxWaitMilliseconds:policy.hostMaxWaitMilliseconds,waited=now()-began;
    if(waited>=limit){reduced=true;stats.reduced++;say("reduced",{...fields(row,Math.round(waited/1000),Math.round(limit/1000)),state:verdict});return"reduced";}
    if(now()-said>=policy.progressMilliseconds){said=now();say(verdict==="overGrant"?"overGrant":"hostBusy",fields(row,Math.round(waited/1000),Math.round(limit/1000)));}
    if(verdict==="overGrant"&&now()-lastSettle>=policy.settleMinimumMilliseconds){lastSettle=now();stats.settles++;try{await row.installed.settle(true);}catch{}}
    stats.waits++;
    await sleep(policy.stepMilliseconds,signal);stats.waitedMilliseconds+=policy.stepMilliseconds;
    row=read();verdict=decide(row);
   }
   if(said>-Infinity)say("resumed",fields(row,Math.round((now()-began)/1000),0));
   return"admit";
  },
  stats:()=>Object.freeze({...stats}),
 });
}

let shared;
/** 🔗️ The wave admission of this process (created at the first call, with the real gate and the real host sampler; its lines go to stderr). */
export function waveAdmissionV1(){return shared??=createWaveAdmissionV1({report:line=>{try{process.stderr.write(line+"\n");}catch{}}});}
