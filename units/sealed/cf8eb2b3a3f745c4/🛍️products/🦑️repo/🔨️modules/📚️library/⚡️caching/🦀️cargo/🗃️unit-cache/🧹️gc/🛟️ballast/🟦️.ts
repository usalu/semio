import {dirname,join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {tryClaimV1,type ClaimV1} from "../../🔒️lock/🟦️.ts";
import {ensureNativeImageReserveFileV1,releaseNativeImageReserveFileV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🛟️reserve/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.gc;
/** 🛟️ Where the ballast of a cache root lives: a file of real bytes that is given back the moment the image is full, so the collector and the admission can still create their claim and record files and free the rest. */
export function ballastPathV1(root:string):string{return join(root,rules.ballastFile);}

/** 🆘️ Gives the ballast back; `true` when bytes were freed. Never throws. The file logic is the image reserve of the native storage layer. */
export function releaseBallastV1(root:string):boolean{return releaseNativeImageReserveFileV1(ballastPathV1(root));}

/** 🧱️ Keeps the ballast whole while the image has room for it (four times its size free, so it never takes the last bytes); `present`, `created` or `skipped` when the image is too full or the write failed. Never throws. */
export function ensureBallastV1(root:string,space:(root:string)=>Readonly<{availableBytes:number}>|undefined=nativeFreeSpaceSyncV1):"present"|"created"|"skipped"{return ensureNativeImageReserveFileV1(ballastPathV1(root),rules.ballastBytes,4,()=>space(root));}

/** 🚪️ Takes a claim like `tryClaimV1`, but on a full image gives the ballast back and tries once more, so the collector that has to free space is not locked out by the lack of it. */
export function tryClaimWithBallastV1(directory:string,name:string,claim:(directory:string,name:string)=>ClaimV1|undefined=tryClaimV1):ClaimV1|undefined{
 try{return claim(directory,name);}
 catch(error){if((error as NodeJS.ErrnoException).code!=="ENOSPC"||!releaseBallastV1(dirname(directory)))throw error;return claim(directory,name);}
}
