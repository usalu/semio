  //#region MutationFacetContract
  for (const required of ["🧬️schema", "⚙️engine", "🚪️io", "\ud83c\udfd7\ufe0fbuilder", "\ud83e\ude93\ufe0fdecomposer"] as const) {
    if (!taxonomy.artifactComponentDirs.includes(required)) {
      problems.push(`artifactComponentDirs must include "${required}".`);
    }
    if (!taxonomy.artifactChildDirs.includes(required)) {
      problems.push(`artifactChildDirs must include "${required}".`);
    }
  }
  for (const banned of ["🗣️dsl", "🔧️op", "📡️spr", "🔺️diff", "📸️snapshot", "🧬️mutations"] as const) {
    if (taxonomy.artifactComponentDirs.includes(banned)) {
      problems.push(`artifactComponentDirs must not include root "${banned}" — absorbed under 🧬️schema.`);
    }
  }
  if (!Array.isArray(taxonomy.mutationChildDirs) || taxonomy.mutationChildDirs.length === 0) {
    problems.push(`mutationChildDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.mutationChildDirs) {
      if (!dir) {
        problems.push(`mutationChildDirs contains an empty entry.`);
        continue;
      }
      if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) {
        problems.push(`mutationChildDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
      }
    }
  }
  for (const required of ["🧬️mutations", "🦠️mutation", "↩️inverse"] as const) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
      problems.push(`taxonomyLeafParentDirs must include "${required}".`);
    }
  }
  //#endregion MutationFacetContract
  //#region IoFacetContract
  if ("mediaFormatDirs" in taxonomy) problems.push(`mediaFormatDirs is removed — use ioDirectionDirs + ioDirectionChildDirs.`);
  if ("ioFormatChildDirs" in taxonomy) problems.push(`ioFormatChildDirs is removed — use ioDirectionDirs + ioDirectionChildDirs.`);
  if (!Array.isArray(taxonomy.ioDirectionDirs) || taxonomy.ioDirectionDirs.length === 0) {
    problems.push(`ioDirectionDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.ioDirectionDirs) {
      if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) {
        problems.push(`ioDirectionDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
      }
      if (!taxonomy.ioDirectionChildDirs?.[dir]) {
        problems.push(`ioDirectionChildDirs must declare "${dir}".`);
      }
    }
  }
  const ioChildDirs = taxonomy.ioDirectionChildDirs ?? {};
  for (const [direction, child] of Object.entries(ioChildDirs)) {
    if (!(taxonomy.ioDirectionDirs ?? []).includes(direction)) {
      problems.push(`ioDirectionChildDirs key "${direction}" is not in ioDirectionDirs.`);
    }
    if (!child) problems.push(`ioDirectionChildDirs["${direction}"] is empty.`);
    else if (!taxonomy.taxonomyLeafParentDirs.includes(child)) {
      problems.push(`ioDirectionChildDirs["${direction}"] = "${child}" is missing from taxonomyLeafParentDirs.`);
    }
  }
  for (const required of ["📥️import", "📤️export", "🚪️io", "\ud83e\udde9\ufe0fdeserializers", "\ud83e\uddf5\ufe0fserializers"] as const) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
      problems.push(`taxonomyLeafParentDirs must include "${required}".`);
    }
  }
  //#endregion IoFacetContract
  //#region SchemaFacetContract
  if ("snapshotChildDirs" in taxonomy) problems.push(`snapshotChildDirs is removed — use schemaChildDirs + representationDirs.`);
  if ("diffChildDirs" in taxonomy) problems.push(`diffChildDirs is removed — use schemaChildDirs + representationDirs.`);
  if (!Array.isArray(taxonomy.schemaChildDirs) || taxonomy.schemaChildDirs.length === 0) {
    problems.push(`schemaChildDirs must be a non-empty array.`);
  } else {
    for (const required of ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const) {
      if (!taxonomy.schemaChildDirs.includes(required)) problems.push(`schemaChildDirs must include "${required}".`);
    }
  }
  if (!Array.isArray(taxonomy.representationDirs) || taxonomy.representationDirs.length === 0) {
    problems.push(`representationDirs must be a non-empty array.`);
  } else {
    for (const required of ["\ud83d\udcdd\ufe0ftext", "\ud83d\udcbe\ufe0fbinary"] as const) {
      if (!taxonomy.representationDirs.includes(required)) problems.push(`representationDirs must include "${required}".`);
      if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
        problems.push(`taxonomyLeafParentDirs must include representation "${required}".`);
      }
    }
  }
  if (!Array.isArray(taxonomy.textSpecFilenames) || taxonomy.textSpecFilenames.length !== 8) {
    problems.push(`textSpecFilenames must list exactly 8 leaves.`);
  }
  if (!Array.isArray(taxonomy.binarySpecFilenames) || taxonomy.binarySpecFilenames.length !== 6) {
    problems.push(`binarySpecFilenames must list exactly 6 leaves.`);
  }
  for (const required of ["\ud83c\udfd7\ufe0fbuilder", "\ud83e\ude93\ufe0fdecomposer", "⚙️engine", "🧬️schema"] as const) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
      problems.push(`taxonomyLeafParentDirs must include "${required}".`);
    }
  }
  for (const [key, dirs] of [
    ["configChildDirs", taxonomy.configChildDirs],
    ["presenceChildDirs", taxonomy.presenceChildDirs],
  ] as const) {
    if (!Array.isArray(dirs) || dirs.length === 0) {
      problems.push(`${key} must be a non-empty array.`);
      continue;
    }
    for (const dir of dirs) {
      if (!dir) problems.push(`${key} contains an empty entry.`);
      else if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) problems.push(`${key} member "${dir}" is missing from taxonomyLeafParentDirs.`);
    }
  }
  if (taxonomy.artifactComponentDirs.includes("🎒️pack") || taxonomy.artifactChildDirs.includes("🎒️pack")) {
    problems.push(`a bare "🎒️pack" is not an artifact facet — binary snapshot lives under 🧬️schema/📸️snapshot/💾️binary.`);
  }
  const schemaFormats = taxonomy.schemaFormats ?? {};
  if (Object.keys(schemaFormats).length === 0) problems.push(`schemaFormats must be a non-empty registry.`);
  for (const [formatId, format] of Object.entries(schemaFormats)) {
    if (!format.leafFilename.endsWith(format.extension)) {
      problems.push(`schemaFormats["${formatId}"] leafFilename must end with its extension (${JSON.stringify(format.leafFilename)} vs ${JSON.stringify(format.extension)}).`);
    }
    if (format.fieldCasing !== "snake" && format.fieldCasing !== "camel") {
      problems.push(`schemaFormats["${formatId}"].fieldCasing must be "snake" or "camel", got ${JSON.stringify(format.fieldCasing)}.`);
    }
  }
  const normativeSchemaLeaf = schemaFormats["� ${JSON.stringify(format.fieldCasing)}.`);
    }
  }
  const normativeSchemaLeaf = schemaFormats["🔣️jsonschema"]?.leafFilename;
  for (const [facet, specName] of Object.entries(taxonomy.artifactSchemaSpecFilenames ?? {})) {
    if (!(facet === "🧬️schema" || artifactFacetPathIsDeclared(facet, taxonomy))) {
      problems.push(`artifactSchemaSpecFilenames key "${facet}" is not a declared schema facet path.`);
    }
    if (specName !== normativeSchemaLeaf) {
      problems.push(`artifactSchemaSpecFilenames["${facet}"] = ${JSON.stringify(specName)} must be the normative schemaFormats["🔣️jsonschema"] leaf ${JSON.stringify(normativeSchemaLeaf)}.`);
    }
  }
  for (const [facet, specName] of Object.entries(taxonomy.appSchemaSpecFilenames ?? {})) {
    if (!appFacetPathIsDeclared(facet, taxonomy)) {
      problems.push(`appSchemaSpecFilenames key "${facet}" is not a declared app facet path.`);
    }
    if (specName !== normativeSchemaLeaf) {
      problems.push(`appSchemaSpecFilenames["${facet}"] = ${JSON.stringify(specName)} must be the normative schemaFormats["🔣️jsonschema"] leaf ${JSON.stringify(normativeSchemaLeaf)}.`);
    }
  }
  if (!Array.isArray(taxonomy.appComponentDirs) || taxonomy.appComponentDirs.length === 0) {
    problems.push(`appComponentDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.appComponentDirs) {
      if (!taxonomy.appChildDirs.includes(dir)) problems.push(`appComponentDirs member "${dir}" is missing from appChildDirs — the structural set must be a superset of the completeness set.`);
    }
  }
  for (const banned of ["🧮️config", "🕸️wasm"] as const) {
    if (taxonomy.appChildDirs.includes(banned) || taxonomy.appComponentDirs.includes(banned)) {
      problems.push(`a bare "${banned}" is not an app facet — use "🎚️config" and "🌉️wasm".`);
    }
  }
  //#endregion SchemaFacetContract
