B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    s=open(B+path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:70])
        s=s.replace(old,new)
    open(B+path,"w",encoding="utf8").write(s)

R="🌐️remote-execution/"
edit(R+"🛰️agent/💽️disk/🟦️.ts",[
("""/** 💽️ What a build worker needs on its volume, in gibibytes: `reserveGigabytes` stays free for the machine itself (its blob cache of 256 MiB, swap, the replica and the owner's own builds), `perActionGigabytes` is the worst case one action adds (the largest compiler outputs measured are 140 MB; an output is written beside a temporary copy and the dependency file, so 0.5 GiB leaves a margin). */
export type RemoteExecutionWorkerDiskV1=Readonly<{reserveGigabytes:number;perActionGigabytes:number}>;

/** 🧮️ How many actions the free space allows at once and what that was measured on. */
export type RemoteExecutionDiskSlotsV1=Readonly<{slots:number;freeBytes:number|undefined;needBytes:number;oneActionBytes:number}>;""",
"""/** 💽️ What a build worker needs, in gibibytes, on two volumes: `reserveGigabytes` stays free on the volume of the work directory (its partition image, which no other consumer protects), `hostReserveGigabytes` stays free on the volume of the checkout for the machine itself (swap, the replica and the owner's own builds, because a partition image grows into the host volume), `perActionGigabytes` is the worst case one action adds to each (the largest compiler outputs measured are 140 MB; an output is written beside a temporary copy and the dependency file, so 0.5 GiB leaves a margin). */
export type RemoteExecutionWorkerDiskV1=Readonly<{reserveGigabytes:number;hostReserveGigabytes:number;perActionGigabytes:number}>;

/** 🧮️ How many actions the free space allows at once and what that was measured on: the numbers are those of the volume that limits (`volume`), the work volume when both allow the same. */
export type RemoteExecutionDiskSlotsV1=Readonly<{slots:number;freeBytes:number|undefined;needBytes:number;oneActionBytes:number;volume:"work"|"host"}>;"""),
("""/** 📏️ The free space a worker with `concurrency` slots needs to serve all of them: the reserve plus the worst case of every slot. */
export const remoteExecutionDiskNeedBytesV1=(disk:RemoteExecutionWorkerDiskV1,concurrency:number):number=>Math.ceil((disk.reserveGigabytes+Math.max(1,concurrency)*disk.perActionGigabytes)*GIBIBYTE);

/** 🎚️ The number of actions the free space of the worker's volume allows to run at the same time: none below the reserve plus one action, one more for every `perActionGigabytes` above it, never more than `concurrency`. A volume that cannot be asked does not restrict. A machine with little free space therefore keeps serving a few units instead of none. Pure. */
export function remoteExecutionDiskSlotsV1(input:Readonly<{freeBytes:number|undefined;concurrency:number;disk:RemoteExecutionWorkerDiskV1}>):RemoteExecutionDiskSlotsV1{
 const {freeBytes,concurrency,disk}=input,reserve=disk.reserveGigabytes*GIBIBYTE,per=disk.perActionGigabytes*GIBIBYTE,need=remoteExecutionDiskNeedBytesV1(disk,concurrency);
 const one=Math.ceil(reserve+per);
 if(freeBytes===undefined)return Object.freeze({slots:concurrency,freeBytes,needBytes:need,oneActionBytes:one});
 const slots=per<=0?concurrency:Math.max(0,Math.min(concurrency,Math.floor((freeBytes-reserve)/per)));
 return Object.freeze({slots,freeBytes,needBytes:need,oneActionBytes:one});
}""",
"""/** 📏️ The free space a worker with `concurrency` slots needs to serve all of them on the volume with the reserve `reserveGigabytes`: the reserve plus the worst case of every slot. */
export const remoteExecutionDiskNeedBytesV1=(disk:Readonly<{reserveGigabytes:number;perActionGigabytes:number}>,concurrency:number):number=>Math.ceil((disk.reserveGigabytes+Math.max(1,concurrency)*disk.perActionGigabytes)*GIBIBYTE);

const volumeSlotsV1=(freeBytes:number|undefined,concurrency:number,reserveGigabytes:number,perActionGigabytes:number):RemoteExecutionDiskSlotsV1=>{
 const reserve=reserveGigabytes*GIBIBYTE,per=perActionGigabytes*GIBIBYTE,need=remoteExecutionDiskNeedBytesV1({reserveGigabytes,perActionGigabytes},concurrency),one=Math.ceil(reserve+per);
 if(freeBytes===undefined)return Object.freeze({slots:concurrency,freeBytes,needBytes:need,oneActionBytes:one,volume:"work"});
 const slots=per<=0?concurrency:Math.max(0,Math.min(concurrency,Math.floor((freeBytes-reserve)/per)));
 return Object.freeze({slots,freeBytes,needBytes:need,oneActionBytes:one,volume:"work"});
};

/** 🎚️ The number of actions the free space of the two volumes allows to run at the same time: on each volume none below its reserve plus one action, one more for every `perActionGigabytes` above it, never more than `concurrency`; the smaller of the two counts, and the numbers reported are those of the volume that limits. A volume that cannot be asked does not restrict. A machine with little free space therefore keeps serving a few units instead of none, and a work volume that holds nothing but the worker is not charged the machine's reserve. Pure. */
export function remoteExecutionDiskSlotsV1(input:Readonly<{freeBytes:number|undefined;hostFreeBytes?:number|undefined;concurrency:number;disk:RemoteExecutionWorkerDiskV1}>):RemoteExecutionDiskSlotsV1{
 const {concurrency,disk}=input,work=volumeSlotsV1(input.freeBytes,concurrency,disk.reserveGigabytes,disk.perActionGigabytes),host=volumeSlotsV1(input.hostFreeBytes,concurrency,disk.hostReserveGigabytes,disk.perActionGigabytes);
 return host.slots<work.slots?Object.freeze({...host,volume:"host" as const}):work;
}"""),
])
edit(R+"🛰️agent/🟦️.ts",[
("disk?:RemoteExecutionWorkerDiskV1;freeBytes?:()=>number|undefined;","disk?:RemoteExecutionWorkerDiskV1;freeBytes?:()=>number|undefined;hostFreeBytes?:()=>number|undefined;"),
("remoteExecutionDiskSlotsV1({freeBytes:(options.freeBytes??(()=>remoteExecutionFreeBytesV1(options.directory)))(),concurrency",
 "remoteExecutionDiskSlotsV1({freeBytes:(options.freeBytes??(()=>remoteExecutionFreeBytesV1(options.directory)))(),hostFreeBytes:(options.hostFreeBytes??(()=>remoteExecutionFreeBytesV1(options.root)))(),concurrency"),
('{free:((space.freeBytes??0)/2**30).toFixed(1),path:options.directory.split(/[\\\\/]/u).filter(Boolean).slice(-3).join("/"),',
 '{free:((space.freeBytes??0)/2**30).toFixed(1),path:(space.volume==="host"?options.root:options.directory).split(/[\\\\/]/u).filter(Boolean).slice(-3).join("/"),'),
("With `disk` the free space of the work directory is part of every admission:","With `disk` the free space of the work directory and of the checkout (the host volume) is part of every admission:"),
])
edit("🔣️policy.json",[('''        "reserveGigabytes": 3,
        "perActionGigabytes": 0.5''','''        "reserveGigabytes": 0.5,
        "hostReserveGigabytes": 3,
        "perActionGigabytes": 0.5''')])
edit("🏃️execution/🧬️schema/🌐️remote-execution.json",[('''          "required": [
            "reserveGigabytes",
            "perActionGigabytes"
          ],''','''          "required": [
            "reserveGigabytes",
            "hostReserveGigabytes",
            "perActionGigabytes"
          ],'''),
('''              "description": "Free space of the volume of the work directory that stays untouched for the machine itself: the worker's blob cache (256 MiB), swap, the replica and the owner's own builds. A worker takes no action below this plus one action."
            },''','''              "description": "Free space of the volume of the work directory (its partition image) that stays untouched: a margin for the worker's own blob cache (256 MiB) and the file system. A worker takes no action below this plus one action."
            },
            "hostReserveGigabytes": {
              "type": "number",
              "minimum": 0,
              "maximum": 100000,
              "description": "Free space of the volume of the checkout (the host volume, into which a partition image grows) that stays untouched for the machine itself: swap, the replica and the owner's own builds. A worker takes no action below this plus one action."
            },''')])
