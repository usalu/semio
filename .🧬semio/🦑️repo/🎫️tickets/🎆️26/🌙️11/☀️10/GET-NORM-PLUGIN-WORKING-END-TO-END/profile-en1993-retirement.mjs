import { readFileSync, writeFileSync } from 'node:fs';
import { Session } from 'node:inspector';
import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';

const [runner, output, ...args] = process.argv.slice(2);
process.argv = [process.argv[0], runner, ...args];
const startMarker = process.env.NORM_RETIREMENT_START ?? 'runtime archive 12500:';
const endMarker = process.env.NORM_RETIREMENT_END ?? 'runtime archive cleanup';
let quote;
if (process.env.NORM_RETIREMENT_QUOTE_WASM) {
  const originalInstantiate = WebAssembly.instantiate;
  const instrumented = new WebAssembly.Module(readFileSync(process.env.NORM_RETIREMENT_QUOTE_WASM));
  WebAssembly.instantiate = async (module, imports) => {
    const isMain = WebAssembly.Module.exports(module).some(({ kind }) => kind === 'memory');
    const instance = await originalInstantiate(isMain ? instrumented : module, imports);
    if (isMain) quote = instance.exports;
    return instance;
  };
}
const inspector = new Session();
inspector.connect();
inspector.post('Profiler.enable');
inspector.post('Profiler.setSamplingInterval', { interval: 100 });
const require = createRequire(pathToFileURL(runner));
const { stdout, stderr } = await import(pathToFileURL(require.resolve('@bytecodealliance/preview2-shim/cli')));
const observe = (stream, handler) => {
  const write = stream.write.bind(stream);
  stream.write = (chunk) => {
    handler(Buffer.from(chunk).toString());
    return write(chunk);
  };
};
let started = false;
let recentOutput = '';
observe(stdout.getStdout(), (chunk) => {
  recentOutput = (recentOutput + chunk).slice(-250);
  if (!started && recentOutput.includes(startMarker)) {
    started = true;
    inspector.post('Profiler.start');
    console.error('[DEBUG] retirement profile started');
  }
});
let stopped = false;
let recentError = '';
const stop = () => {
  if (!started || stopped) return;
  stopped = true;
  if (quote) console.error('[DEBUG] retirement quote', Object.keys(quote)
    .filter((key) => /^quote\d+$/.test(key)).map((key) => quote[key].value));
  inspector.post('Profiler.stop', (failure, result) => {
    if (failure) throw failure;
    writeFileSync(output, JSON.stringify(result.profile));
  });
};
observe(stderr.getStderr(), (chunk) => {
  recentError = (recentError + chunk).slice(-250);
  if (recentError.includes(endMarker)) stop();
});
process.on('exit', stop);
await import(pathToFileURL(runner));
