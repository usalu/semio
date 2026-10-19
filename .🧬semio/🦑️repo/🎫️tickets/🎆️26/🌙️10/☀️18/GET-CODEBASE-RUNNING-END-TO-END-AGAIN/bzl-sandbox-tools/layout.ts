import {bazelCacheLayoutV1,bazelCacheFlagsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts";
const r="/Users/ueli/Documents/semio/.cache/b";
const layout=bazelCacheLayoutV1({tools:r+"/t/m",dependencies:r+"/d/m",products:r+"/p/m"});
console.log(JSON.stringify(bazelCacheFlagsV1(layout,process.argv[2]!,{environment:{}})));
