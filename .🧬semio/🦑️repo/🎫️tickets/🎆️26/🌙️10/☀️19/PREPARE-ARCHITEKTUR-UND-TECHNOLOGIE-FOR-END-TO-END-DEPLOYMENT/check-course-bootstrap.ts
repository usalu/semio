import { readFileSync, mkdirSync, mkdtempSync } from 'node:fs';
import { strict as assert } from 'node:assert';
import { join, win32 } from 'node:path';
import { pathToFileURL } from 'node:url';
const root = process.cwd();
const native = join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap');
const corpus = JSON.parse(readFileSync(join(native, '🧪️tests/🧰️toolchain/🔣️.json'), 'utf8'));
const cppPolicy = JSON.parse(readFileSync(join(native, '🧬️schema/🔣️.json'), 'utf8')).definitions.MsvcBootstrap.const;
const { nativeBootstrapCommandV1 } = await import(pathToFileURL(join(native, '🟦️.ts')).href);
for (const row of corpus.commands) {
  let actual = null;
  try { actual = nativeBootstrapCommandV1(row.arguments); } catch {}
  assert.equal(actual, row.expected);
}
const { runCmdStatus } = await import(pathToFileURL(join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
const windows = readFileSync(join(native, '🔵️.ps1'), 'utf8');
const compilerStart = windows.indexOf('function Ensure-CppBuildTools {');
assert.ok(compilerStart > 0);
const compiler = windows.slice(compilerStart, windows.indexOf('\nfunction ', compilerStart + 1));
const tests = readFileSync(join(native, '🧪️tests/🧰️toolchain/🟦️.ts'), 'utf8');
const begin = tests.indexOf('const script = compiler + ');
const end = tests.indexOf('\n    const result = spawnSync', begin);
const expression = tests.slice(begin + 'const script = '.length, end).trim().replace(/;$/, '');
for (const row of corpus.msvc) {
  let script = new Function('compiler', 'row', 'return ' + expression)(compiler, row);
  script = script.replace("$script:actions -join ','\nexit $result", "if (($script:actions -join ',') -ne '" + row.actions.join(",") + "') { throw 'Unexpected compiler actions' }\nexit $result");
  const status = runCmdStatus('powershell.exe', ['-NoProfile', '-Command', script], { env: { ...process.env, SEMIO_TEST_CPP_POLICY: JSON.stringify(cppPolicy) } });
  assert.equal(status === 0, row.succeeds);
}
const installStart = windows.indexOf('function Install-CppBuildTools {');
assert.ok(installStart > 0);
const install = windows.slice(installStart, windows.indexOf('\nfunction ', installStart + 1));
const installBegin = tests.indexOf('const script = install + ');
const installEnd = tests.indexOf('\n    const policy = bootstrapSchema', installBegin);
const installExpression = tests.slice(installBegin + 'const script = '.length, installEnd).trim().replace(/;$/, '');
const installScript = new Function('install', 'return ' + installExpression)(install);
for (const fail of [false, true]) for (const code of [0, 3010, 42]) {
  const checks = installScript + '\n$case = $env:SEMIO_TEST_CPP_CASE | ConvertFrom-Json\nif ($script:removed -ne $true -or $script:launched -ne (!$case.fail)) { throw "Installer ownership order failed" }\nif (!$case.fail) { $wanted = Join-Path "C:\\ascii\\semio-00000000000000000000000000000000" $case.installer; if ($script:launch.path -ne $wanted -or $script:verification.path -ne $wanted -or $script:verification.sha256 -ne $case.policy.sha256 -or $script:verification.url -ne $case.policy.url) { throw "Verified ASCII path failed" }; if (($script:launch.arguments -join ",") -ne ($case.arguments -join ",") -or !$script:launch.waited -or !$script:launch.passed -or $script:launch.window -ne "Hidden") { throw "Installer argument contract failed" } }';
  const status = runCmdStatus('powershell.exe', ['-NoProfile', '-Command', checks + `\nif ($script:failed -ne $${fail || !corpus.msvcBootstrap.successCodes.includes(code)}) { throw "Unexpected installer status" }`], { env: { ...process.env, SEMIO_TEST_CPP_POLICY: JSON.stringify(cppPolicy), SEMIO_TEST_FAIL: fail ? '1' : '0', SEMIO_TEST_EXIT: String(code), SEMIO_TEST_CPP_CASE: JSON.stringify({ ...corpus.msvcBootstrap, policy: cppPolicy, fail }) } });
  assert.equal(status, 0);
}

const downloadStart = windows.indexOf('function Save-VerifiedDownload {');
const download = windows.slice(downloadStart, windows.indexOf('\nfunction ', downloadStart + 1));
const downloadBegin = tests.indexOf('const script = download + ');
const downloadEnd = tests.indexOf('\n  for (const row of corpus.downloads)', downloadBegin);
const downloadExpression = tests.slice(downloadBegin + 'const script = '.length, downloadEnd).trim().replace(/;$/, '');
const downloadScript = new Function('download', 'return ' + downloadExpression)(download);
for (const row of corpus.downloads) {
  for (const expected of [row.sha256, '0'.repeat(64)]) {
    const file = join(process.env.SEMIO_TICKET_DIR, '🗑️generated/checksum-probe.bin');
    const guarded = downloadScript.replace(/Save-VerifiedDownload -Url [^\n]+/, line => 'try { ' + line + '; exit 0 } catch { exit 1 }');
    const status = runCmdStatus('powershell.exe', ['-NoProfile', '-Command', guarded], { env: { ...process.env, SEMIO_TEST_TEXT: row.text, SEMIO_TEST_SHA256: expected, SEMIO_TEST_FILE: file } });
    assert.equal(status === 0, expected === row.sha256);
  }
}

const e2e = join(root, '🎓️teaching/🏛️architecture/📚️course');
const source = readFileSync(join(e2e, '🎭️e2e/🟦️.ts'), 'utf8');
const functionSource = (name) => { const start = source.indexOf('export function ' + name); return start < 0 ? '' : source.slice(start, source.indexOf('\n/**', start + 1)).replace(/^export /, ''); };
const module = new Bun.Transpiler({ loader: 'ts' }).transformSync(['endToEndRunDirectory', 'endToEndReportDirectory', 'endToEndPlaywrightArgumentsV1', 'endToEndPlan'].map(functionSource).join('\n'));
const helpers = new Function('join', 'mkdirSync', 'mkdtempSync', 'sep', 'COURSE_E2E_TOPOLOGIES', module + ';return { endToEndRunDirectory, endToEndReportDirectory, endToEndPlan }')(join, mkdirSync, mkdtempSync, win32.sep, ['dev', 'rehearsal']);
const cases = JSON.parse(readFileSync(join(e2e, '🧫️fixtures/🎭️e2e-output/🔣️.json'), 'utf8'));
const first = helpers.endToEndRunDirectory(join(process.env.SEMIO_TICKET_DIR, '🗑️generated/path-check'));
assert.notEqual(first, helpers.endToEndRunDirectory(join(process.env.SEMIO_TICKET_DIR, '🗑️generated/path-check')));
const budget = cases.pathBudget;
const work = win32.join(budget.repositories[0].root, budget.ticket, '🗑️generated', budget.run);
for (const row of cases.arguments) { let valid = true; try { helpers.endToEndPlan(['--', ...row.value]); } catch { valid = false; } assert.equal(valid, row.valid, JSON.stringify(row.value)); }
let longest = 0;
for (const row of cases.reports) {
  const output = helpers.endToEndReportDirectory(work, row.stack, row.phase);
  assert.equal(output, join(work, row.directory));
  for (const file of budget.files) {
    const length = win32.join(output, budget.testDirectory, file).length;
    assert.ok(length <= budget.maximum);
    longest = Math.max(longest, length);
  }
}
console.log(JSON.stringify({ commands: corpus.commands.length, checksumCases: corpus.downloads.length * 2, compilerBootstrapCases: 2, compilerCases: corpus.msvc.length, argumentCases: cases.arguments.length, reportCases: cases.reports.length, maximumArtifactPath: longest }));
