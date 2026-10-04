# Nx Dependency Authority Source Draft

This unmounted source fragment belongs in the existing owned Nx patch. It accepts owned configuration declarations and bounded primitive digest results. No resolver callback is evaluated for serialization.

```js
/** 🏛️ Reads each declared owned dependency authority as a bounded digest, without evaluating import resolution callbacks. */
async function readDependencyResolutionAuthority(nxJson, projects) {
    const declarations = Object.entries(nxJson.pluginsConfig ?? {}).filter(([, value]) => value?.dependencyResolutionAuthority).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0);
    if (!declarations.length) return null;
    if (declarations.length > 16) throw new Error('Too many dependency resolution authorities');
    const hash = require('node:crypto').createHash('sha256');
    for (const [name, options] of declarations) {
        const descriptor = options.dependencyResolutionAuthority;
        if (typeof descriptor.module !== 'string' || typeof descriptor.export !== 'string') throw new Error('Invalid dependency resolution authority declaration');
        const path = path_1.resolve(workspace_root_1.workspaceRoot, descriptor.module), local = path_1.relative(workspace_root_1.workspaceRoot, path);
        if (path_1.isAbsolute(local) || local === '..' || local.startsWith('..' + path_1.sep)) throw new Error('Dependency resolution authority must belong to the workspace');
        const owner = await import(require('node:url').pathToFileURL(path).href);
        if (typeof owner[descriptor.export] !== 'function') throw new Error('Dependency resolution authority is missing its owned interface');
        const value = await owner[descriptor.export](workspace_root_1.workspaceRoot, projects);
        if (typeof value !== 'string' || !/^[a-f0-9]{64}$/.test(value)) throw new Error('Invalid dependency resolution authority digest');
        hash.update(JSON.stringify([name, descriptor.module, descriptor.export, value]));
    }
    return hash.digest('hex');
}

```
