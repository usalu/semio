import {connectGrpcV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/📡️wire/🚪️grpc/🟦️.ts";
import {fleetHeadersV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
const root="/Users/ueli/Documents/semio",endpoint=process.argv[2]!,mode=process.argv[3]!;
const method="build.bazel.remote.execution.v2.Capabilities/GetCapabilities";
const client=connectGrpcV1(endpoint,mode==="sign"?{sign:(m:string,c:string)=>fleetHeadersV1(root,"submit","execution","POST","http://x/"+m,c)}:{});
const metadata=mode==="bearer"?{authorization:"Bearer "+process.env.SEMIO_REMOTE_CACHE_TOKEN}:{};
try{await client.unary(method,{instance_name:"semio"},{metadata});console.log(mode,"OK");}catch(e){console.log(mode,"FAIL",String((e as Error).message));}finally{client.close();}
