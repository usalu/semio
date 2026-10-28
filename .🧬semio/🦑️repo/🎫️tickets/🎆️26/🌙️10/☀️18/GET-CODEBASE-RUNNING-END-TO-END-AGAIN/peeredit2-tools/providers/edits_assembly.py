A = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
N = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs"
EDITS = [
(A, '''closureOf: (entries, extras) => closures.closureOf(entries, extras)''', '''closureOf: (entries, extras, role) => closures.closureOf(entries, extras, role)'''),
(A, '''policy.inputs.push(...internCommandSources(await closures.closureOf([lockOwner], [`{workspaceRoot}/${buildPath(relative(workspaceRoot, join(dirname(lockOwner), "🔣️schema.json")))}`])), loaded.cargoLockInput(nativeManifest, workspaceRoot));''', '''policy.inputs.push(...internCommandSources(await closures.closureOf([lockOwner], [`{workspaceRoot}/${buildPath(relative(workspaceRoot, join(dirname(lockOwner), "🔣️schema.json")))}`], "cargo-lock-owner")), loaded.cargoLockInput(nativeManifest, workspaceRoot));'''),
(A, '''    namedInputs[name] = sourceRepresentation === "source" ? loaded.deduplicateProjectInputs(inputs, root) : loaded.compactSourceInputs(inputs, workspaceRoot, root);
  }
  const project = { ...json, name: json.name, root, namedInputs, targets: normalized };''', '''    namedInputs[name] = sourceRepresentation === "source" ? loaded.deduplicateProjectInputs(inputs, root) : loaded.compactSourceInputs(inputs, workspaceRoot, root);
  }
  if (json.metadata?.semio?.scriptProviders !== undefined) for (const [name, inputs] of await closures.scriptProviderGroups(root, json.metadata.semio.scriptProviders)) {
    if (Object.hasOwn(namedInputs, name)) throw new Error(`Duplicate script provider group ${name}`);
    const handle = closureHandleOfV1(inputs);
    if (handle) { namedInputs[name] = [handle.marker]; symbolicGroups.push([name, false]); continue; }
    namedInputs[name] = sourceRepresentation === "source" ? loaded.deduplicateProjectInputs(inputs, root) : loaded.compactSourceInputs(inputs, workspaceRoot, root);
  }
  const project = { ...json, name: json.name, root, namedInputs, targets: normalized };'''),
(N, '''await closureOf([artifactSource], ["{projectRoot}/package.json"]) : [];''', '''await closureOf([artifactSource], ["{projectRoot}/package.json"], "artifact-sources") : [];'''),
(N, '''{ runtime: 'node -p "process.platform.concat(process.arch)"' }]) : [];
  const cargoManifest''', '''{ runtime: 'node -p "process.platform.concat(process.arch)"' }], "artifact-command") : [];
  const cargoManifest'''),
]
