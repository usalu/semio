#!/usr/bin/env bun
/** 🧪️ Runs the owner-neutral mutation inventory conformance and native laws. */
import {resolve} from "node:path";
import {runOwnedCommand} from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import {runCargoTestsV1,readCargoTestPolicyV1} from "../../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
const [command,...args]=process.argv.slice(2),root=resolve(import.meta.dir,"../../../../../../.."),manifestPath=resolve(import.meta.dir,"Cargo.toml");
if(command==="conformance"){if(args.length)throw Error("conformance accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(import.meta.dir,"../../🧪️tests/🟦️.ts")],root,"mutation-inventory-conformance",45000,{env:process.env});}
else if(command==="test")await runCargoTestsV1({manifestPath,packages:["semio-framework-test-mutation-inventory"],cwd:root,extraArgs:["--lib",...args],environment:process.env},readCargoTestPolicyV1(process.env));
else throw Error("conformance|test");

