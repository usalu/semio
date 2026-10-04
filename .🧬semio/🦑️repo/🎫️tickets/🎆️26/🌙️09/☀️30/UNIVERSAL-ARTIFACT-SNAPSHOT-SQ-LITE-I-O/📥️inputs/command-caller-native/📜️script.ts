/** 🚦️ Executes the actual caller-owned control law with the repository Cargo policy. */
import {resolve} from "node:path";
import {runCargoTestsV1,readCargoTestPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
await runCargoTestsV1({manifestPath:resolve(import.meta.dir,"Cargo.toml"),packages:["semio-ticket-command-caller-native"],cwd:import.meta.dir,extraArgs:["--lib","command_transport_neutral_","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
