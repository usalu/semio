import type {ConformanceContribution} from "../🔎️discovery/🟦️.ts";
/** 🧩️ Captured current package metadata is supplied by its defining repository owner. */
export interface FixtureContributionPackage {readonly directory:string;readonly manifest:string;readonly name:string;readonly metadata:unknown;}
function record(value:unknown):value is Record<string,unknown>{return value!==null&&typeof value==="object"&&!Array.isArray(value);}
/** 🧭️ Every present Specific artifact participates; other owners contribute explicitly. */
export function currentFixtureContribution(pkg:FixtureContributionPackage):ConformanceContribution {
 const semio=record(pkg.metadata)&&record(pkg.metadata.semio)?pkg.metadata.semio:null;
 const declaration=semio?.conformance;
 return {name:pkg.name,manifest:pkg.manifest,eligible:semio?.role==="artifact"&&(pkg.directory.startsWith("✏️s/")||declaration!==undefined),declaration};
}
