#!/usr/bin/env bun
/** ◻️ Runs this artifact's declared producer through the general inventory command. */
import {resolve} from "node:path";
import {runMutationInventoryCargoProducerCommand} from "../../../../../../🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🚪️command/🟦️.ts";
await runMutationInventoryCargoProducerCommand({manifestPath:resolve(import.meta.dir,"../📦️packages/🦀️rust/Cargo.toml"),binary:"semio-s-artifact-block-2d-mutation-inventory",cwd:resolve(import.meta.dir,"../../../../../..")});
