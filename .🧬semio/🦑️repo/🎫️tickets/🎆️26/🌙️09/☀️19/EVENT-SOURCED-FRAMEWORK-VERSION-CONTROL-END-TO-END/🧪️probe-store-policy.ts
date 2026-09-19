/** 🧪️ Evaluates the tool-jobs store policy predicates against the live sources (the full gate stops early on an unrelated wgpu check). */
import { readFileSync } from "node:fs";
import { toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactEventSourcedMergeExact } from "../../../../../../../📜️script.ts";
import { toolJobCoverageSelfTests } from "../../../../../../../🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts";
const root = "/Users/ueli/Documents/semio/";
const read = (path: string) => readFileSync(root + path, "utf8");
const store = read("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs");
const vcs = read("🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs");
const causal = read("🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs");
const semio = read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs");
console.log("eventSourcedMerge", toolJobArtifactEventSourcedMergeExact(store));
console.log("historyLedgerAdmission", toolJobHistoryLedgerAdmissionExact(store, vcs));
console.log("structuralOwners", toolJobArtifactStoreStructuralOwnersExact(store, semio, causal, vcs));
console.log("selfTests", toolJobCoverageSelfTests());
