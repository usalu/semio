/** 🪪️ Contract: a physical identity above 2^53 (a 64-bit NTFS file id, a 128-bit ReFS id) crosses the protected maintenance request exactly: the token is the JSON integer while it is safe and the decimal digits above, two neighbouring ids never collapse into one rounded number, and a refusal never mentions a safe-integer limit. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{nativePhysicalIdentityTokenV1,nativePhysicalIdentityNumberV1,readNativeProtectedMaintenanceRequestV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import observationSchema from"../../🧬️schema/🔣️.json"with{type:"json"};

const big=(1n<<63n)+5n,request=(dev:unknown,ino:unknown)=>({version:1,purpose:"mounted-parent-maintenance",wait:false,receiptSha256:"a".repeat(64),directory:{dev,ino,mode:16877},backing:{dev,ino,size:1048576,allocatedBytes:1048576},root:{dev,ino:5,mode:16877}});

test("an identity of 2^63+5 round-trips exactly and neighbours stay distinct",()=>{
 expect(nativePhysicalIdentityTokenV1(big)).toBe("9223372036854775813");expect(nativePhysicalIdentityTokenV1(big-1n)).not.toBe(nativePhysicalIdentityTokenV1(big));
 expect(Number(big)).toBe(Number(big-1n));expect(nativePhysicalIdentityTokenV1(9007199254740991n)).toBe(9007199254740991);expect(nativePhysicalIdentityTokenV1(9007199254740992n)).toBe("9007199254740992");expect(nativePhysicalIdentityTokenV1((1n<<127n)+1n)).toBe(((1n<<127n)+1n).toString());
 const read=readNativeProtectedMaintenanceRequestV1(request(1234,nativePhysicalIdentityTokenV1(big)));expect(read.backing.ino).toBe("9223372036854775813");expect(BigInt(read.backing.ino as string)).toBe(big);expect(read.backing.dev).toBe(1234);
});

test("tokens are canonical and bounded, and a number above 2^53 is refused as a number",()=>{
 expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,"5"))).toThrow();expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,"09223372036854775813"))).toThrow();expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,9223372036854775813))).toThrow();expect(()=>readNativeProtectedMaintenanceRequestV1(request(1,"9007199254740991"))).toThrow();
 expect(()=>nativePhysicalIdentityTokenV1(-1n)).toThrow();expect(()=>nativePhysicalIdentityTokenV1(1n<<128n)).toThrow();expect(nativePhysicalIdentityNumberV1(7)).toBe(7);expect(()=>nativePhysicalIdentityNumberV1("9223372036854775813")).toThrow("above 2^53");
});

test("the dependency observation wire carries the same exact tokens and no producer converts an identity with Number",()=>{
 const directory={dev:1,ino:"9223372036854775813",mode:16877,kind:"directory"},bad={dev:1,ino:9223372036854775813,mode:16877,kind:"directory"};
 expect(validateJsonSchemaSubset(observationSchema.$defs.Directory as never,directory,observationSchema as never)).toEqual([]);expect(validateJsonSchemaSubset(observationSchema.$defs.Directory as never,bad,observationSchema as never).length).toBeGreaterThan(0);
 const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");expect(source).toContain("ino:nativePhysicalIdentityTokenV1(row.ino)");expect(source).not.toContain("ino:number(row.ino)");expect(source).not.toContain("dev:number(row.dev)");
 const fixed=readFileSync(fileURLToPath(new URL("../../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts",import.meta.url)),"utf8"),from=fixed.indexOf("function protectedMaintenancePhysicalFactsV1"),to=fixed.indexOf("export function captureNativeProtectedMaintenancePhysicalV1"),facts=fixed.slice(from,to);
 expect(facts).toContain("{bigint:true}");expect(facts).not.toMatch(/folder\.ino!==|file\.ino!==|mounted\.ino!==/u);
});
