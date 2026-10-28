/** 🎯️ Which test owners a partial inference discovers again: all of them, or the ones under the roots that are not known yet. */
export interface TestOwnerScopeV1{readonly all:boolean;readonly roots:ReadonlySet<string>;readonly known:ReadonlySet<string>}
/** 🧪️ The feature files of the test owners to discover again, grouped by the directory of the owner's cases. */
export function selectTestFeaturesV1(features:readonly string[],scope:TestOwnerScopeV1):Map<string,string[]>;
/** 🔢️ The code point order of test owner directories. */
export const compareTestOwnersV1:(a:string,b:string)=>number;
/** 📏️ How many test owners are discovered one by one before the rest is discovered in one call. */
export const SEPARATE_TEST_OWNERS_V1:number;
/** 📦️ What discovering test owners answers: the contributions, the rows by project and the derived facts. */
export interface TestOwnerDiscoveryV1<Row>{readonly contributions:[string,{projects?:Record<string,unknown>;[key:string]:unknown}][];readonly rows:Map<string,Row[]>;readonly derived:readonly {readonly row:Row;readonly support:readonly Row[]}[]}
/** 🔭️ Discovers the owners of `groups` one by one, each in its own capture, and answers their contributions, the complete facts of every project and the derived facts those facts name. */
export function discoverTestOwnersV1<Row>(input:{tests:{discoverTestSharedV1(context:object):Promise<unknown>;discoverTestBuildProjectsV1(files:readonly string[],options:object,context:object):Promise<[string,{projects?:Record<string,unknown>}][]>};capture:(compute:()=>Promise<unknown>)=>Promise<{value:any;observations:readonly Row[]}>;context:object;groups:ReadonlyMap<string,readonly string[]>;separate?:number}):Promise<TestOwnerDiscoveryV1<Row>>;
