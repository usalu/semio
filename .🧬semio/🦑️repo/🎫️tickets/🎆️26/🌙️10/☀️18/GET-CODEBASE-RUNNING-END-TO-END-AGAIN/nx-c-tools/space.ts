const root="/Users/ueli/Documents/semio",contract=await import(root+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts");
const space=contract.nativeImageSpaceV1(root+"/.cache/b/g/m",6441402368);
console.log(JSON.stringify({bsize:space.bsize,bavail:space.bavail,availMiB:space.bavail*space.bsize/1048576}));
