import {discoverTestContributions,oracleImportsInProduction,discoverTestCases,loadOracleRegistry,repoRootFromHere,repositoryContractBreaches} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const t=(l:string,f:()=>any)=>{const s=performance.now();const r=f();console.log("[prof]",l,Math.round(performance.now()-s),"ms");return r;};
const root=repoRootFromHere();
const contrib=t("discoverTestContributions",()=>discoverTestContributions(root));
const reg=t("loadOracleRegistry",()=>loadOracleRegistry(root));
const cases=t("discoverTestCases",()=>discoverTestCases(root));
const hits=t("oracleImportsInProduction",()=>oracleImportsInProduction(root,reg,cases));
t("repositoryContractBreaches",()=>repositoryContractBreaches(root,reg,cases,hits));
