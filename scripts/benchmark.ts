import { execSync } from "child_process";
import * as fs from "fs";
import * as path from "path";

const REPORT_FILE = "temp/benchmark.csv";

// Helper to run command and capture output
function runCommand(command: string, cwd: string): string {
  try {
    return execSync(command, { cwd, encoding: "utf-8", stdio: "pipe" });
  } catch (e: any) {
    console.error(`Command failed: ${command}`);
    console.error(e.stderr);
    return "";
  }
}

interface BenchmarkResult {
  test: string;
  lang: string;
  time: string;
}

const RESULTS: BenchmarkResult[] = [];

function parseOutput(lang: string, output: string) {
  // Expected output format: "TestName,TimeInSeconds"
  const lines = output.split("\n");
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const parts = trimmed.split(",");
    if (parts.length === 2) {
      RESULTS.push({
        test: parts[0],
        lang: lang,
        time: parts[1],
      });
    }
  }
}

console.log("Running benchmarks...");

// 1. Typescript
console.log("Running Typescript...");
const tsOutput = runCommand("npx tsx benchmark.ts", "js/semio");
parseOutput("Typescript", tsOutput);

// 2. Python
console.log("Running Python...");
const pyOutput = runCommand("uv run benchmark.py", "py/semio");
parseOutput("Python", pyOutput);

// 3. Go
console.log("Running Go...");
const goOutput = runCommand("go run benchmark/main.go", "go/semio");
parseOutput("Go", goOutput);

// 4. Dotnet
console.log("Running C#...");
// Use Release configuration for accurate benchmarking
const csOutput = runCommand("dotnet run --project Semio.Benchmark/Semio.Benchmark.csproj --configuration Release", "net");
parseOutput("C#", csOutput);

// 5. Rust
console.log("Running Rust...");
// Use release profile for accurate benchmarking
const rsOutput = runCommand("cargo run --release --example benchmark", "rs/semio");
parseOutput("Rust", rsOutput);


// Collating results
const tests = new Set(RESULTS.map((r) => r.test));
const langs = ["Typescript", "Python", "Go", "C#", "Rust"];

let csvContent = "Test," + langs.join(",") + "\n";

for (const test of Array.from(tests).sort()) {
  const row = [test];
  for (const lang of langs) {
    const res = RESULTS.find((r) => r.test === test && r.lang === lang);
    row.push(res ? res.time : "");
  }
  csvContent += row.join(",") + "\n";
}

const reportPath = path.resolve(REPORT_FILE);
const reportDir = path.dirname(reportPath);
if (!fs.existsSync(reportDir)) {
  fs.mkdirSync(reportDir, { recursive: true });
}

fs.writeFileSync(reportPath, csvContent);
console.log(`Benchmark report written to ${reportPath}`);
