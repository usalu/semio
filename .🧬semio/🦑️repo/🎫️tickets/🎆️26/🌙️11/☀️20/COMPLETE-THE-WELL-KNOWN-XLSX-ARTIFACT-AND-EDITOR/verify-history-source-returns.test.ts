import {test} from "bun:test";
import {verifyMutationSourceCancellation} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🟦️.ts";
import {originalHistoryInputOracle} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🟦️.ts";

test("mutation Source cancellation returns all borrowed issuer aliases", verifyMutationSourceCancellation);
test("history Source cancellation restores the original ledger slot", originalHistoryInputOracle);
