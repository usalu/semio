import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🧪️tests/🟦️.ts",source=readFileSync(path,"utf8"),before="when ?1='cargo' then 'cargo' when ?1 in ('run','exec')",after="when ?1='cargo' then 'cargo' when ?1 in ('incremental','conformance','concurrency','bench','service','units') then 'utility' when ?1 in ('run','exec')";
if(!source.includes(before))throw Error("Selection oracle changed");writeFileSync(path,source.replace(before,after));
