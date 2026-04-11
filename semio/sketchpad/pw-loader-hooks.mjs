import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';

// Resolve `esbuild` from repo root (hoisted); devcontainer used a fixed `/workspaces/semio` path.
const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(join(__dirname, '..', '..', 'package.json'));
const esbuild = require('esbuild');

export async function resolve(specifier, context, nextResolve) {
  // Stub every CSS module as empty ESM. Relying on `format: css-noop` + load() still hit Node's
  // "Unknown file extension .css" for some package subpaths (e.g. @xyflow/react/dist/style.css).
  const cssish =
    /\.css(\?|#|$)/i.test(specifier) ||
    (specifier.startsWith("file:") && /\.css(\?|#|$)/i.test(specifier));
  if (cssish) {
    return { url: "data:text/javascript,export default {}", format: "module", shortCircuit: true };
  }
  let result;
  try {
    result = await nextResolve(specifier, context);
  } catch (e) {
    if (specifier.endsWith('.css')) {
      return { url: 'data:text/javascript,export default {}', format: 'module', shortCircuit: true };
    }
    throw e;
  }
  const resolvedPath = String(result.url).split(/[?#]/)[0];
  if (resolvedPath.endsWith(".css")) {
    return { url: "data:text/javascript,export default {}", format: "module", shortCircuit: true };
  }
  if ((result.url.endsWith('.ts') || result.url.endsWith('.tsx')) && !result.url.includes('node_modules')) {
    return { ...result, format: 'ts-esm' };
  }
  return result;
}

function stubViteGlob(source) {
  let result = '';
  let i = 0;
  const needle = 'import.meta.glob';
  while (i < source.length) {
    const idx = source.indexOf(needle, i);
    if (idx === -1) {
      result += source.slice(i);
      break;
    }
    result += source.slice(i, idx) + '((() => ({}))';
    i = idx + needle.length;
    if (source[i] === '<') {
      let depth = 1;
      i++;
      while (i < source.length && depth > 0) {
        if (source[i] === '<') depth++;
        else if (source[i] === '>') depth--;
        i++;
      }
    }
    if (source[i] === '(') {
      let depth = 1;
      i++;
      while (i < source.length && depth > 0) {
        if (source[i] === '(') depth++;
        else if (source[i] === ')') depth--;
        i++;
      }
    }
    result += ')';
  }
  return result;
}

export async function load(url, context, nextLoad) {
  if (url.endsWith('.css') || context.format === 'css-noop') {
    return { format: 'module', shortCircuit: true, source: 'export default {}' };
  }
  // Handle JSON imports that need type: json attribute
  if (url.endsWith('.json') && !context.importAttributes?.type) {
    const filePath = fileURLToPath(url);
    const json = readFileSync(filePath, 'utf8');
    return { format: 'module', shortCircuit: true, source: `export default ${json}` };
  }
  if (context.format === 'ts-esm') {
    const filePath = fileURLToPath(url);
    let source = readFileSync(filePath, 'utf8');
    source = stubViteGlob(source);
    const loader = url.endsWith('.tsx') ? 'tsx' : 'ts';
    const result = esbuild.transformSync(source, {
      loader,
      format: 'esm',
      target: 'esnext',
      sourcemap: false,
      jsx: 'automatic',
    });
    return { format: 'module', shortCircuit: true, source: result.code };
  }
  return nextLoad(url, context);
}
