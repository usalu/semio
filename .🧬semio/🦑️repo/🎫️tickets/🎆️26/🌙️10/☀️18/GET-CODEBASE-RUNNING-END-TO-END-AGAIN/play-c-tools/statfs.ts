import {nativeStatfsV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
const root=process.argv[2]!;const fs=await nativeStatfsV1(root,{bigint:true});console.log(root.slice(-30),"avail GiB",Number(fs.bavail*fs.bsize)/2**30,"bsize",fs.bsize);
