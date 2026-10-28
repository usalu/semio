/** 🧪 Exercise actual cache registry retirement without browser mocks. */
import {writeFile} from 'node:fs/promises';
const {LocalSourceRevisionRegistryV1}=await import('file:///C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/📝️source-revision/registry.ts');
function owner(){let active=true;const listeners=new Set();return {identity:{sessionInstanceId:1},get active(){return active;},subscribeRetirement(f){listeners.add(f);return ()=>listeners.delete(f);},retire(){active=false;for(const f of [...listeners])f();}};}
const plugin={},registry=new LocalSourceRevisionRegistryV1(1),first=owner();registry.acquire(first,plugin);first.retire();const result={sizeAfterRetire:registry.size,nextOwnerAdmitted:registry.acquire(owner(),plugin)!==null};await writeFile(new URL('./🗑️generated/source-registry-retirement-probe.json',import.meta.url),JSON.stringify(result,null,2));console.log('[DEBUG] '+JSON.stringify(result));
