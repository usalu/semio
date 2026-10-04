/** 🏭️ Runs the ticket-only canonical owner asset producer through actual repository Cargo policy. */
import {resolve} from "node:path";
import {runCargoTestsV1,readCargoTestPolicyV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
await runCargoTestsV1({manifestPath:resolve(import.meta.dir,"Cargo.toml"),packages:["semio-ticket-csv-tsv-core-fixture-producer"],cwd:import.meta.dir,extraArgs:["--lib","canonical_csv_tsv_owner_asset_producer","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
