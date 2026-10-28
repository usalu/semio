/** 🪟️ Contract: the row paths of a fact index are native absolute paths (`resolve` of the observation module), the config of a withheld owner is a workspace relative `/` path; a native Windows path must be judged by components like a POSIX path, else every withheld owner counts as unfounded on that host and is inferred again by every build. */
import{expect,test}from"bun:test";
import{createFactIndexV1,unfoundedWithheldOwnersV1}from"../../../🟨️.mjs";

const fact={sha256:"0".repeat(64),size:1,mtimeMs:1,ctimeMs:1,ino:1};
const row=(path:string)=>({kind:"file",path,fact});
const owners={founded:{kind:"unavailable",config:"p/x.ts"},unfounded:{kind:"unavailable",config:"p/y.ts"},root:{kind:"unavailable",config:"z.ts"}};
const index=()=>createFactIndexV1(new Map([["founded",[row("C:\\ws\\p\\x.ts")]],["unfounded",[row("C:\\ws\\p\\other.ts")]],["root",[row("C:\\ws\\z.ts")]]]) as never);

test("a native Windows path is judged by its components",()=>{
 expect(unfoundedWithheldOwnersV1(index() as never,owners,"\\")).toEqual(["unfounded"]);
});

test("a backslash is an ordinary character where the separator is a slash",()=>{
 expect(unfoundedWithheldOwnersV1(index() as never,owners,"/").sort()).toEqual(["founded","root","unfounded"]);
});

test("a POSIX path is unchanged by either separator rule",()=>{
 const posix=createFactIndexV1(new Map([["founded",[row("/ws/p/x.ts")]],["unfounded",[row("/ws/p/other.ts")]],["root",[row("/ws/z.ts")]]]) as never);
 expect(unfoundedWithheldOwnersV1(posix as never,owners,"/")).toEqual(["unfounded"]);
 expect(unfoundedWithheldOwnersV1(posix as never,owners,"\\")).toEqual(["unfounded"]);
});
