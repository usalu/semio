import {expect,test}from"bun:test";
import {join}from"node:path";
import {nativeStorageLiveRootsV1}from"../../🟦️.ts";
import {retireNativeImageStorageV1,retireNativeImageStorageWithWriterV1}from"../../../🍎️image/🟦️.ts";
import {retireNativeFixedPlatformV1,retireNativeFixedPlatformWithWriterV1}from"../../../🧱️fixed/🟦️.ts";
import {retireNativeImageShardV1,dismantleNativeImageShardsV1}from"../../../🍎️image/🧩️shard/🟦️.ts";
import {retireNativeRecomputableCapsuleV1}from"../../../🧹️whole/🟦️.ts";
import {quarantineStrandedNativeRootEntriesV1,removeEmptyNativeReceiverShellV1}from"../../../🧹️cleanup/🎛️command/🐚️shell/🟦️.ts";
import {detachNativeImageWithRetryV1}from"../../../🍎️image/🔁️detach/🟦️.ts";
import {removeNativeImageSiblingVolumesV1}from"../../../🍎️image/🔗️attach/🟦️.ts";
import {retireNativePlatformStorageV1}from"../../../🌐️platform/🟦️.ts";
import {detachDiskImagesUnderV1}from"../../../../../../🧪️testing/🏖️sandbox/🖼️images/🟦️.ts";
const live=join(nativeStorageLiveRootsV1()[0]!,".cache","b","h64-entry-negative"),any=undefined as never;
const entries:Readonly<Record<string,()=>unknown>>={
 retireNativeImageStorageV1:()=>retireNativeImageStorageV1({directory:live}as never),
 retireNativeImageStorageWithWriterV1:()=>retireNativeImageStorageWithWriterV1({directory:live}as never,any,any),
 retireNativeFixedPlatformV1:()=>retireNativeFixedPlatformV1({directory:live}as never),
 retireNativeFixedPlatformWithWriterV1:()=>retireNativeFixedPlatformWithWriterV1({directory:live}as never,any,any),
 retireNativeImageShardV1:()=>retireNativeImageShardV1({directory:live}as never),
 dismantleNativeImageShardsV1:()=>dismantleNativeImageShardsV1({directory:live}as never),
 retireNativeRecomputableCapsuleV1:()=>retireNativeRecomputableCapsuleV1(any,live,any),
 quarantineStrandedNativeRootEntriesV1:()=>quarantineStrandedNativeRootEntriesV1(join(live,"m")),
 removeEmptyNativeReceiverShellV1:()=>removeEmptyNativeReceiverShellV1(live,join(live,"m")),
 detachNativeImageWithRetryV1:()=>detachNativeImageWithRetryV1({mountPoint:join(live,"m")}as never),
 removeNativeImageSiblingVolumesV1:()=>removeNativeImageSiblingVolumesV1(any,join(live,"fixed.dmg")),
 retireNativePlatformStorageV1:()=>retireNativePlatformStorageV1({directory:live}as never),
 detachDiskImagesUnderV1:()=>detachDiskImagesUnderV1(join(nativeStorageLiveRootsV1()[0]!,".cache","b")),
};
test("every destructive storage entry refuses a test-runner call against the live checkout before any other check",async()=>{
 for(const[name,run]of Object.entries(entries)){
  const error=await Promise.resolve().then(run).then(()=>undefined,(reason:unknown)=>reason as{key?:string;code?:string});
  expect(error?.key,name).toBe("process.storage.root-capability-refused");expect(error?.code,name).toBe("refused");
 }
});
