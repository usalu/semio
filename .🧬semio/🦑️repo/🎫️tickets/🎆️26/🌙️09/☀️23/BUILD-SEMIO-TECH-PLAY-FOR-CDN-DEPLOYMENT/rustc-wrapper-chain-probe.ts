import {spawnSync} from "node:child_process";import {mkdtempSync} from "node:fs";import {tmpdir} from "node:os";import {join} from "node:path";
import {unitCacheWrapperV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {compilerWrapperEnvironmentV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
const cache=mkdtempSync(join(tmpdir(),"u-")),wrapper=unitCacheWrapperV1(cache)!;
const chain=compilerWrapperEnvironmentV1("rustc-scheduler-stub",join(cache,"l"),{SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_WRAPPER:wrapper});
console.log(Object.keys(chain).includes("RUSTC"),chain.RUSTC_WRAPPER===wrapper);
const run=spawnSync(chain.RUSTC_WRAPPER!,["rustc","-vV"],{encoding:"utf8",env:{...process.env,...chain,SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"])}});
console.log(run.status,run.stdout.split("\n")[0],run.stderr.slice(0,200));
