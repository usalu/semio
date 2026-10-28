const { spawnSync } = require('node:child_process');
const path = require('node:path');
const env = { ...process.env, SEMIO_TOOL_PYTHON: process.env.SEMIO_TOOL_PYTHON || 'C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe' };
for (const verifier of ['verify-mixed-pdf.py', 'verify-matrix-pdf.py', 'verify-directed-pdf.py', 'same-table-native.py']) {
  const args = ['tools', 'exec', '--light', 'python', '--', path.join(__dirname, verifier), ...process.argv.slice(2)];
  const result = spawnSync('repo', args, { env, stdio: 'inherit', shell: false });
  if (result.error) throw result.error;
  if (result.status !== 0) { process.exitCode = result.status ?? 1; break; }
}
