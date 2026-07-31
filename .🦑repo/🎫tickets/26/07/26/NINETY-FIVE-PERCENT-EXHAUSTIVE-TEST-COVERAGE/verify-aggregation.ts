#!/usr/bin/env bun
/** 📊Verification script: runs the real parseLcov/mergeLcov/summarizeCoverage pipeline against the
 * cargo-llvm-cov smoke-test output for mathematical_number, confirming the aggregation logic in
 * repo/lib/js/index.ts works end-to-end against real LCOV data. Ticket: 26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE. */
import { parseLcov, mergeLcov, summarizeCoverage } from "../../../../../../repo/lib/js/index.ts";
import { readFileSync } from "node:fs";

const text = readFileSync(process.argv[2] ?? ".repo/coverage/rust/_Users_ueli_Documents_semio_mathematical_number_rs.lcov", "utf8");
const records = parseLcov(text);
console.log("parsed records:", records.length, "files");
for (const r of records) console.log(" ", r.path, r.lines.size, "lines");

const merged = mergeLcov([records]);
const summary = summarizeCoverage(merged);
console.log("summary:", { linesFound: summary.linesFound, linesHit: summary.linesHit, pct: summary.pct.toFixed(2) });
console.log("worst 3:", summary.perFile.slice(0, 3));
